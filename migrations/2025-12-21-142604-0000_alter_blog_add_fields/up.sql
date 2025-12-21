ALTER TABLE blog ADD COLUMN slug TEXT NOT NULL DEFAULT '';
ALTER TABLE blog ADD COLUMN excerpt TEXT;
ALTER TABLE blog ADD COLUMN thumbnail TEXT;
ALTER TABLE blog ADD COLUMN status TEXT NOT NULL DEFAULT 'DRAFT';
ALTER TABLE blog ADD COLUMN published_at TIMESTAMP;
ALTER TABLE blog ADD COLUMN view_count INTEGER NOT NULL DEFAULT 0;

-- Update existing rows to have unique slug
UPDATE blog SET slug = 'slug-' || hex(randomblob(4));

-- Create index
CREATE UNIQUE INDEX idx_blog_slug ON blog(slug);
