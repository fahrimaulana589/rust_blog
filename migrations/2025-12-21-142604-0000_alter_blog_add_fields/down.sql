-- SQLite doesn't support DROP COLUMN easily.
-- For proper down migration we'd need to recreate the table.
-- But for now, we can typically ignore or do full table rebuild.
-- Since this is dev, doing nothing or simple drop index is minimal.
DROP INDEX idx_blog_slug;
ALTER TABLE blog DROP COLUMN slug;
ALTER TABLE blog DROP COLUMN excerpt;
ALTER TABLE blog DROP COLUMN thumbnail;
ALTER TABLE blog DROP COLUMN status;
ALTER TABLE blog DROP COLUMN published_at;
ALTER TABLE blog DROP COLUMN view_count;
