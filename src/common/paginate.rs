use diesel::{pg::Pg, prelude::*, query_builder::*, sql_types::BigInt};
use diesel_async::{AsyncPgConnection, methods::LoadQuery};
use serde::{Deserialize, Deserializer, Serialize};

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}

const DEFAULT_PER_PAGE: i64 = 10;

#[derive(Debug, Clone, Copy, QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    per_page: i64,
    offset: i64,
}

#[derive(Serialize, Debug, Clone)]
pub struct List<T> {
    pub page: u32,
    pub max_page: u32,
    pub list: Vec<T>,
}

impl<T> List<T> {
    pub fn empty() -> Self {
        Self {
            page: 1,
            max_page: 1,
            list: Vec::new(),
        }
    }
}

impl<T> Paginated<T> {
    pub fn per_page(self, per_page: i64) -> Self {
        Paginated {
            per_page,
            offset: (self.page - 1) * per_page,
            ..self
        }
    }

    pub async fn load_and_count_pages<'a, U>(self, conn: &mut AsyncPgConnection) -> QueryResult<List<U>>
    where
        T: 'a,
        U: Send + 'a,
        Self: LoadQuery<'a, AsyncPgConnection, (U, i64)>,
    {
        let per_page = self.per_page;
        let page = self.page;
        let results = diesel_async::RunQueryDsl::load::<(U, i64)>(self, conn).await?;
        let total = results.first().map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        let result = List {
            list: records,
            max_page: u32::try_from(total_pages).expect("Invalid max page"),
            page: u32::try_from(page).expect("Invalid page"),
        };
        Ok(result)
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<BigInt, _>(&self.offset)?;
        Ok(())
    }
}

pub fn deserialize_opt_page<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i64> = Option::deserialize(deserializer)?;

    match value {
        Some(v) if v < 1 => Err(serde::de::Error::custom(format!("`page` must be >= 1, got {v}"))),
        other => Ok(other),
    }
}
