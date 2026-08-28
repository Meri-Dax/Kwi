CREATE TABLE dietary_restriction (
    "id"        uuid PRIMARY KEY    DEFAULT gen_random_uuid() NOT NULL,
    "slug"      VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE ingredient_dietary_restriction (
    "id"                        uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "ingredient_id"             uuid NOT NULL REFERENCES ingredient("id") ON DELETE CASCADE,
    "dietary_restriction_id"    uuid NOT NULL REFERENCES dietary_restriction("id") ON DELETE CASCADE,
    UNIQUE ("ingredient_id", "dietary_restriction_id")
);
