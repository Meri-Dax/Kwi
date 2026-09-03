ALTER TABLE public.recipe ADD date_created timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL;
ALTER TABLE public.recipe ADD date_updated timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL;

CREATE INDEX recipe_date_created_idx ON public.recipe (date_created);
CREATE INDEX recipe_date_updated_idx ON public.recipe (date_updated);
