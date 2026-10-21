ALTER TABLE public.recipe DROP COLUMN "servings";
ALTER TABLE public.recipe_ingredient DROP COLUMN "servings_scaling";
DROP TYPE "servings_scaling";
