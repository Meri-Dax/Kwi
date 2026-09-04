ALTER TABLE public.recipe DROP COLUMN "status";
ALTER TABLE public.recipe DROP COLUMN "prep_time";
ALTER TABLE public.recipe DROP COLUMN "cook_time";
ALTER TABLE public.recipe DROP COLUMN "fresh_for_hours";
ALTER TABLE public.recipe DROP COLUMN "steps";
ALTER TABLE public.recipe DROP COLUMN "description";
ALTER TABLE public.ingredient DROP COLUMN "fresh_for_days";

DROP TYPE "recipe_status";
