ALTER TABLE public.recipe ADD "servings" SMALLINT CHECK ("servings" > 0) NOT NULL DEFAULT 1;
ALTER TABLE public.recipe ALTER COLUMN "servings" DROP DEFAULT;

CREATE TYPE servings_scaling AS ENUM('fixed', 'multiply');
ALTER TABLE public.recipe_ingredient ADD "servings_scaling" servings_scaling DEFAULT 'multiply' NOT NULL;
