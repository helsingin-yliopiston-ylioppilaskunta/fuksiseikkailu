-- Up Migration: Update checkpoint_category enum to new options
ALTER TYPE checkpoint_category RENAME TO checkpoint_category_old;

CREATE TYPE checkpoint_category AS ENUM (
  'subject',
  'nation',
  'hobby',
  'other',
  'hyy',
  'yliopisto'
);

ALTER TABLE checkpoints 
  ALTER COLUMN category DROP DEFAULT,
  ALTER COLUMN category TYPE checkpoint_category 
    USING (
      CASE category::text
        WHEN 'academic' THEN 'subject'::checkpoint_category
        WHEN 'party' THEN 'hobby'::checkpoint_category
        WHEN 'sports' THEN 'hobby'::checkpoint_category
        WHEN 'start' THEN 'other'::checkpoint_category
        WHEN 'afterparty' THEN 'other'::checkpoint_category
        ELSE 'other'::checkpoint_category
      END
    ),
  ALTER COLUMN category SET DEFAULT 'other';

DROP TYPE checkpoint_category_old;
