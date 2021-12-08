CREATE TABLE projects (
    id uuid PRIMARY KEY,
    name text NOT NULL,
    updated_at timestamptz NOT NULL
)