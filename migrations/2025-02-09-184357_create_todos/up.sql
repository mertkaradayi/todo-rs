-- Your SQL goes here
CREATE TABLE todos (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- Create a trigger to update the updated_at field whenever the row is updated
CREATE TRIGGER update_todo_updated_at
AFTER
UPDATE ON todos FOR EACH ROW BEGIN
UPDATE todos
SET updated_at = CURRENT_TIMESTAMP
WHERE id = OLD.id;
END;