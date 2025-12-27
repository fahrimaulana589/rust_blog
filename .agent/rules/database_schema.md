# Database Schema Reference
The project uses the following Diesel schema tables:
- `blog` (id, title, slug, content, category_id, status, ...): Unique `slug`.
- `blog_tags` (blog_id, tag_id): Many-to-Many link.
- `categories` (id, name): Master data for blog categorization.
- `tags` (id, name): Master data for blog tagging.
- `projects` (id, nama_projek, status, ...): Portfolio projects.
- `portfolios` (id, project_id, ...): Portfolio details linked to projects.
- `stacks` (id, nama_stack): Tech stacks.
- `project_stack` (id, project_id, stack_id): Link projects to stacks.
- `profiles` (id, full_name, role, ...): User profile information.
- `profile_languages`, `profile_specializations`, `profile_tech_focus`: Profile details.
- `users` (id, username, email, password): Authentication.
- `counts`: Utility table.
