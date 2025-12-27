ALTER TABLE projects ADD COLUMN slug TEXT NOT NULL DEFAULT '';
UPDATE projects SET slug = 'slug-' || hex(randomblob(4));
CREATE UNIQUE INDEX projects_slug_unique ON projects(slug);
