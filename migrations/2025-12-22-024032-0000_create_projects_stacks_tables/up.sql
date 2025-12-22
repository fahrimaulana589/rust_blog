-- Drop Project Stack first (foreign key)
DROP TABLE IF EXISTS project_stack;
DROP TABLE IF EXISTS stacks;
DROP TABLE IF EXISTS projects;

-- Create projects table

CREATE TABLE projects (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    nama_projek TEXT NOT NULL,
    deskripsi TEXT NOT NULL,
    status TEXT NOT NULL, -- draft, ongoing, completed
    progress INTEGER NOT NULL DEFAULT 0,
    link_demo TEXT,
    repository TEXT,
    tanggal_mulai DATE NOT NULL,
    tanggal_selesai DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create stacks table
CREATE TABLE stacks (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    nama_stack TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create pivot table
CREATE TABLE project_stack (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    stack_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (stack_id) REFERENCES stacks(id) ON DELETE CASCADE,
    UNIQUE(project_id, stack_id)
);
