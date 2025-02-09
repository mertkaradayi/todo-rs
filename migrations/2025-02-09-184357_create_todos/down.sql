-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS todos;
DROP TRIGGER update_todo_updated_at;