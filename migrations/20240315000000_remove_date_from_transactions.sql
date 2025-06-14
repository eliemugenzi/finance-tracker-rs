-- Remove date column from transactions table
--! Up
ALTER TABLE transactions DROP COLUMN date;
--! Down
ALTER TABLE transactions ADD COLUMN date DATE NOT NULL DEFAULT CURRENT_DATE; 