CREATE TABLE tasks(
  id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  name TEXT NOT NULL,
  description TEXT DEFAULT '' NOT NULL 
);

CREATE TABLE tasks_users(
  task_id UUID REFERENCES tasks(id) NOT NULL,
  user_id UUID REFERENCES users(id) NOT NULL,
  PRIMARY KEY (task_id, user_id)
);
