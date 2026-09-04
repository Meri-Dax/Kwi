-- Expanding recipe

CREATE TYPE recipe_status AS ENUM('draft', 'public');

ALTER TABLE public.recipe ADD "status" recipe_status DEFAULT 'draft' NOT NULL;
ALTER TABLE public.recipe ADD "prep_time" SMALLINT CHECK ("prep_time" >= 0);
ALTER TABLE public.recipe ADD "cook_time" SMALLINT CHECK ("cook_time" >= 0);
ALTER TABLE public.recipe ADD "fresh_for_hours" SMALLINT CHECK ("fresh_for_hours" >= 0);
ALTER TABLE public.recipe ADD "steps" text DEFAULT NULL;
ALTER TABLE public.recipe ADD "description" text DEFAULT NULL;

CREATE INDEX recipe_status_idx ON public.recipe ("status");
CREATE INDEX recipe_prep_time_idx ON public.recipe ("prep_time");
CREATE INDEX recipe_cook_time_idx ON public.recipe ("cook_time");
CREATE INDEX recipe_fresh_for_hours_idx ON public.recipe ("fresh_for_hours");
CREATE INDEX recipe_steps_idx ON public.recipe ("steps");
CREATE INDEX recipe_description_idx ON public.recipe ("description");

-- Expanding ingredient

ALTER TABLE public.ingredient ADD "fresh_for_days" SMALLINT DEFAULT NULL CHECK ("fresh_for_days" > 0);

CREATE INDEX ingredient_status_idx ON public.ingredient ("fresh_for_days");
