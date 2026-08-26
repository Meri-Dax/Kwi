CREATE TABLE ingredient (
    "id"        uuid PRIMARY KEY    DEFAULT gen_random_uuid() NOT NULL,
    "slug"      VARCHAR(255) NOT NULL UNIQUE
);
