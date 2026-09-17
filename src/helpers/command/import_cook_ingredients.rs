use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use clap::Args;

use diesel::{ExpressionMethods, SelectableHelper, upsert::excluded};
use diesel_async::RunQueryDsl;
use tracing::info;

use super::Command;
use crate::{
    common::repository::RepositoryError,
    entities::{
        dietary_restriction::model::{DietaryRestrictionForm, IngredientDietaryRestriction},
        ingredient::model::{Ingredient, IngredientForm},
    },
    helpers::{
        AppState,
        command::csv_extractor::{self, CsvExtractor},
    },
    schema::{dietary_restriction, ingredient, ingredient_dietary_restriction},
};

#[derive(Args)]
pub struct ImportCookIngredientsCommand {
    #[arg(long, short)]
    pub file: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not import")]
    CouldNotImport,
}

impl ImportCookIngredientsCommand {
    pub async fn execute_import(
        import_data: &Vec<IngredientRowImport>,
        app_state: &AppState,
    ) -> Result<(), RepositoryError> {
        let diet_slugs: HashSet<String> = import_data
            .iter()
            .filter_map(|s| s.dietary_restrictions.as_ref())
            .flatten()
            .cloned()
            .collect();

        let diet_forms: Vec<DietaryRestrictionForm> = diet_slugs
            .into_iter()
            .map(|slug| DietaryRestrictionForm { slug })
            .collect();

        let mut conn = app_state.database.get().await?;
        let mut tx = conn.build_transaction().read_write();

        tx.run(async |ts_conn| -> Result<(), RepositoryError> {
            let diets: Vec<(String, uuid::Uuid)> = diesel::insert_into(dietary_restriction::table)
                .values(diet_forms)
                .on_conflict(dietary_restriction::slug) // If row exists with this slug => skip update, return the row as upserted anyway
                .do_update()
                .set(dietary_restriction::slug.eq(excluded(dietary_restriction::slug)))
                .returning((dietary_restriction::slug, dietary_restriction::id))
                .get_results(ts_conn)
                .await?;

            let diet_map: HashMap<String, uuid::Uuid> = diets.into_iter().collect();

            let ingredient_diet_insert_forms: Vec<IngredientForm> = import_data
                .into_iter()
                .map(|v| IngredientForm { slug: v.slug.clone() })
                .collect();

            let ingredients: Vec<Ingredient> = diesel::insert_into(ingredient::table)
                .values(ingredient_diet_insert_forms)
                .on_conflict(ingredient::slug) // If row exists with this slug => skip update, return the row as upserted anyway
                .do_update()
                .set(ingredient::slug.eq(excluded(ingredient::slug)))
                .returning(Ingredient::as_returning())
                .get_results(ts_conn)
                .await?;

            let ingredient_map: HashMap<String, uuid::Uuid> = ingredients
                .into_iter()
                .map(|ingredient| (ingredient.slug, ingredient.id))
                .collect();

            // insert the ingredient >-< diet xrefs
            let mut dietary_restriction_forms: Vec<IngredientDietaryRestriction> = Vec::new();
            for IngredientRowImport {
                slug: ingredient_slug,
                fresh_for_days: _,
                dietary_restrictions,
            } in import_data
            {
                if let Some(dietary_restrictions) = dietary_restrictions {
                    let ingredient_id = ingredient_map
                        .get(ingredient_slug)
                        .unwrap_or_else(|| panic!("Could not find {} in upsert set", ingredient_slug));

                    for diet_slug in dietary_restrictions {
                        let dietary_restriction_id = diet_map
                            .get(diet_slug)
                            .unwrap_or_else(|| panic!("Could not find {} in upsert set", diet_slug))
                            .clone();
                        dietary_restriction_forms.push(IngredientDietaryRestriction {
                            ingredient_id: ingredient_id.clone(),
                            dietary_restriction_id,
                        });
                    }
                }
            }

            if !dietary_restriction_forms.is_empty() {
                diesel::insert_into(ingredient_dietary_restriction::table)
                    .values(dietary_restriction_forms)
                    .on_conflict((
                        ingredient_dietary_restriction::dietary_restriction_id,
                        ingredient_dietary_restriction::ingredient_id,
                    ))
                    .do_nothing()
                    .execute(ts_conn)
                    .await?;
            }

            Ok(())
        })
        .await?;

        Ok(())
    }
}

impl Command for ImportCookIngredientsCommand {
    async fn execute(&self, app_state: &AppState) {
        let start = Instant::now();
        info!("Parsing CSV `{}`", &self.file);
        let mut extractor: CsvExtractor<IngredientRowImport> = match CsvExtractor::try_new(&self.file) {
            Ok(e) => e,
            Err(e) => panic!("Could not instantiate csv extractor: {}", e),
        };

        info!("Parsing import data");
        let rows = match extractor.read_all() {
            Ok(e) => e,
            Err(e) => panic!("Could not read rows: {}", e),
        };

        info!("Importing {} rows", rows.len());
        match Self::execute_import(&rows, app_state).await {
            Ok(e) => {
                info!("Executed in {}", humantime::format_duration(start.elapsed()));
                e
            }
            Err(e) => panic!("Could not import rows: {}", e),
        }
    }
}

#[derive(Clone, Debug)]
pub struct IngredientRowImport {
    pub slug: String,
    pub fresh_for_days: Option<i32>,
    pub dietary_restrictions: Option<Vec<String>>,
}

impl TryFrom<csv::StringRecord> for IngredientRowImport {
    type Error = csv_extractor::Error;
    fn try_from(req: csv::StringRecord) -> Result<Self, Self::Error> {
        if req.is_empty() {
            return Err(csv_extractor::Error::InvalidRecord);
        }

        let slug = req[0].to_string();

        let fresh_for_days = match req.get(1) {
            Some(v) if !v.is_empty() => match v.parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => return Err(csv_extractor::Error::InvalidRecord),
            },
            _ => None,
        };

        let dietary_restrictions: Option<Vec<String>> = req.get(2).map(|v| {
            v.split(',')
                .filter(|e| !e.is_empty())
                .map(|e| e.trim().to_string())
                .collect()
        });

        Ok(Self {
            slug,
            fresh_for_days,
            dietary_restrictions,
        })
    }
}
