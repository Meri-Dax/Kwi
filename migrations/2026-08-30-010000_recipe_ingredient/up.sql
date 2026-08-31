CREATE TYPE ingredient_unit AS ENUM('unit', 'milliliter', 'gram');

CREATE TABLE recipe_ingredient (
    "id"                        uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "recipe_id"                 uuid NOT NULL REFERENCES recipe("id") ON DELETE CASCADE,
    "ingredient_id"             uuid NOT NULL REFERENCES ingredient("id") ON DELETE CASCADE,
    "qty"                       INTEGER NOT NULL CHECK ("qty" > 0),
    "unit"                      ingredient_unit NOT NULL,
    UNIQUE ("recipe_id", "ingredient_id")
);
