-- Logistics

CREATE TABLE recipe_logistics (
    "id"                        uuid PRIMARY KEY    DEFAULT gen_random_uuid() NOT NULL,
    "slug"                      VARCHAR(255) NOT NULL UNIQUE,
    "date_created"              timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "date_updated"              timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE recipe_recipe_logistics_xref (
    "id"                        uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "recipe_id"                 uuid NOT NULL REFERENCES recipe("id") ON DELETE CASCADE,
    "recipe_logistics_id"       uuid NOT NULL REFERENCES recipe_logistics("id") ON DELETE CASCADE,
    UNIQUE ("recipe_id", "recipe_logistics_id")
);

-- Equipment

CREATE TABLE recipe_equipment (
    "id"                        uuid PRIMARY KEY    DEFAULT gen_random_uuid() NOT NULL,
    "slug"                      VARCHAR(255) NOT NULL UNIQUE,
    "date_created"              timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "date_updated"              timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE recipe_recipe_equipment_xref (
    "id"                        uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "recipe_id"                 uuid NOT NULL REFERENCES recipe("id") ON DELETE CASCADE,
    "recipe_equipment_id"       uuid NOT NULL REFERENCES recipe_equipment("id") ON DELETE CASCADE,
    UNIQUE ("recipe_id", "recipe_equipment_id")
);

-- Course

CREATE TABLE recipe_course (
    "id"                        uuid PRIMARY KEY    DEFAULT gen_random_uuid() NOT NULL,
    "slug"                      VARCHAR(255) NOT NULL UNIQUE,
    "date_created"              timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "date_updated"              timestamptz DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE recipe_recipe_course_xref (
    "id"                        uuid PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    "recipe_id"                 uuid NOT NULL REFERENCES recipe("id") ON DELETE CASCADE,
    "recipe_course_id"          uuid NOT NULL REFERENCES recipe_course("id") ON DELETE CASCADE,
    UNIQUE ("recipe_id", "recipe_course_id")
);
