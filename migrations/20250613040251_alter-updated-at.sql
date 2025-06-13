-- Add migration script here
--! Up
ALTER TABLE transactions ALTER COLUMN updated_at SET NOT NULL;
ALTER TABLE transactions ALTER COLUMN updated_at SET DEFAULT NOW();
ALTER TABLE users ALTER COLUMN updated_at SET NOT NULL;
ALTER TABLE users ALTER COLUMN updated_at SET DEFAULT NOW();
--! Down
ALTER TABLE transactions ALTER COLUMN updated_at DROP NOT NULL;
ALTER TABLE transactions ALTER COLUMN updated_at DROP DEFAULT;
ALTER TABLE users ALTER COLUMN updated_at DROP NOT NULL;
ALTER TABLE users ALTER COLUMN updated_at DROP DEFAULT;

