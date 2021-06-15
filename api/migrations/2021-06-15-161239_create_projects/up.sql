CREATE TABLE projects(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name TEXT NOT NULL,
  description TEXT DEFAULT '' NOT NULL 
);

ALTER TABLE tasks
ADD COLUMN project_id UUID REFERENCES projects(id) NULL;
