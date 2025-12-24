
Dokumentasi lengkap endpoint API untuk project MyBlog. Dokumen ini mencakup semua endpoint yang terdaftar di aplikasi (auth, home, profile, blog, projects, stacks, portfolios).

Base URL: `http://localhost:8080`

Swagger UI: `http://localhost:8080/swagger-ui/`
OpenAPI JSON: `http://localhost:8080/api-docs/openapi.json`

## Format respon umum

Success Response
```json
{
  "message": "Operation successful",
  "data": { /* optional */ }
}
```

Paginated Response (contoh)
```json
{
  "message": "Data fetched successfully",
  "data": {
    "items": [ /* array of resource objects */ ],
    "meta": { "page": 1, "per_page": 10, "total_pages": 1, "total_items": 10 }
  }
}
```

Error / Validation response
```json
{
  "message": "Validation Error",
  "errors": { "field": "error message" }
}
```

Catatan: Endpoint yang berada di bawah scope `/app` dilindungi oleh middleware `Auth` dan membutuhkan header:
`Authorization: Bearer <JWT_TOKEN>`

---

## 1. Root & Home

### GET /
Description: Root health / welcome endpoint (public)

Response (200 OK)
```json
{ "message": "Hello, world!" }
```

### GET /app/count
Description: Increment/return a counter (protected)
Response (200 OK)
```json
{ "message": "42" }
```

### GET /app/send-email
Description: Trigger sending a test email (protected)
Response (200 OK)
```json
{ "message": "Email sent successfully" }
```

---

## 2. Authentication (Public)

### POST /login
Request body:
```json
{ "username": "admin", "password": "password" }
```
Success (200): contoh response
```json
{
  "message": "Login successful",
  "data": {
    "username": "admin",
    "email": "admin@example.com",
    "token": "eyJ..."
  }
}
```

### POST /logout
Description: Logs out the user by clearing the auth cookie.
Success (200):
```json
{ "message": "Logout successful" }
```
Response Header: `Set-Cookie: auth_token=; Max-Age=0; HttpOnly; Path=/`

### POST /forgot-password
Request body:
```json
{ "email": "user@example.com" }
```
Success (200):
```json
{ "message": "Password reset email sent" }
```

### POST /reset-password
Request body:
```json
{ "token": "token_from_email", "new_password": "newpass" }
```
Success (200):
```json
{ "message": "Password reset successful" }
```

### GET /app/islogin
Description: Check if user is authenticated (protected).
Headers: `Authorization: Bearer <token>` or `Cookie: auth_token=<token>`
Success (200):
```json
{ "message": "User is logged in" }
```
Error (401): Unauthorized

---

## 3. Profile (Protected)
Headers: `Authorization: Bearer <token>`

### GET /app/profile
Response (200): Jika profile ada, `data` berisi object profile; jika belum dibuat, `data` adalah null.

Contoh Profile object
```json
{
  "full_name": "Fahri Maulana",
  "headline": "Senior Rust Developer",
  "summary": "Experienced developer...",
  "role": "Backend Engineer",
  "location": "Jakarta, Indonesia",
  "profile_image": "https://...",
  "availability": "Open for work",
  "years_of_experience": 5,
  "resume_url": "https://...",
  "email": "fahri@example.com",
  "work_philosophy": "Clean Code",
  "timezone": "Asia/Jakarta",
  "specializations": ["Rust", "System Design"],
  "tech_focus": ["Actix Web", "Diesel"],
  "languages": [ { "name": "Indonesian", "level": "Native" }, { "name": "English", "level": "Professional" } ]
}
```

### POST /app/profile
Request body: same shape as contoh Profile object (fields validated as in code)

Response (200): success message with the saved profile object in `data`.

---

## 4. Blog (Protected)
Headers: `Authorization: Bearer <token>`

Semua endpoint blog ada di bawah `/app`.

### Categories

- GET /app/categories?page={page}&per_page={per_page}
  - Response: paginated list of categories
  - Contoh satu item:
```json
{
  "id": 1,
  "name": "Rust",
  "created_at": "2025-01-01T10:00:00Z",
  "updated_at": "2025-01-01T10:00:00Z"
}
```

- POST /app/categories
  - Request example:
```json
{ "name": "Tutorial" }
```
  - Success: 201 Created, contoh response:
```json
{ "message": "Category created successfully", "data": { "id": 2, "name": "Tutorial", "created_at": "...", "updated_at": "..." } }
```

- GET /app/categories/{id}
  - Path param: id (int)
  - Success: category object

- PUT /app/categories/{id}
  - Request example:
```json
{ "name": "Updated Name" }
```

- DELETE /app/categories/{id}
  - Success: empty success response

### Tags

- GET /app/tags?page={page}&per_page={per_page}
  - Response: paginated list of tags
  - Item example:
```json
{ "id": 1, "name": "Rust", "created_at": "...", "updated_at": "..." }
```

- POST /app/tags
  - Request example:
```json
{ "name": "Rust" }
```

- GET /app/tags/{id}
- PUT /app/tags/{id}
  - Request example:
```json
{ "name": "New Tag" }
```
- DELETE /app/tags/{id}

### Blogs

- GET /app/blogs?page={page}&per_page={per_page}
  - Response: paginated list of blog objects
  - Blog item example:
```json
{
  "id": 1,
  "title": "Belajar Rust Dasar",
  "slug": "belajar-rust-dasar",
  "content": "Isi artikel...",
  "excerpt": "Singkat cerita...",
  "thumbnail": "url",
  "status": "DRAFT",
  "view_count": 10,
  "category": { "id": 1, "name": "Rust" },
  "tags": [ { "id": 1, "name": "Rust" } ],
  "created_at": "2025-01-01T10:00:00Z",
  "updated_at": "2025-01-01T10:00:00Z",
  "published_at": null
}
```

- POST /app/blogs
  - Request example:
```json
{
  "title": "Belajar Rust Dasar",
  "content": "Isi artikel...",
  "category_id": 1,
  "tag_ids": [1,2],
  "excerpt": "...",
  "thumbnail": "url",
  "status": "DRAFT"
}
```

- GET /app/blogs/{id}
  - Success: blog object (see example above)

- PUT /app/blogs/{id}
  - Request: same fields as create but all optional

- DELETE /app/blogs/{id}

---

## 5. Projects & Stacks (Protected)
Headers: `Authorization: Bearer <token>`

### Projects

- GET /app/projects?page={page}&per_page={per_page}
  - Response: paginated list of projects
  - Project item example:
```json
{
  "id": 1,
  "nama_projek": "My Portfolio Website",
  "deskripsi": "Website personal...",
  "status": "ongoing",
  "progress": 50,
  "link_demo": "https://...",
  "repository": "https://github.com/...",
  "tanggal_mulai": "2025-01-01",
  "tanggal_selesai": null,
  "stacks": [ { "id": 1, "nama_stack": "Rust" } ],
  "created_at": "2025-01-01T10:00:00Z",
  "updated_at": "2025-01-01T10:00:00Z"
}
```

- POST /app/projects
  - Request example:
```json
{
  "nama_projek": "My Portfolio Website",
  "deskripsi": "...",
  "status": "ongoing",
  "progress": 50,
  "link_demo": "https://...",
  "repository": "https://github.com/...",
  "tanggal_mulai": "2025-01-01",
  "tanggal_selesai": null,
  "stack_ids": [1,3]
}
```

- GET /app/projects/{id}
- PUT /app/projects/{id}
  - Request: partial fields allowed (see create example)
- DELETE /app/projects/{id}

### Stacks

- GET /app/stacks
- POST /app/stacks
  - Request example:
```json
{ "nama_stack": "Rust" }
```
- GET /app/stacks/{id}
- PUT /app/stacks/{id}
  - Request example (same as create)
- DELETE /app/stacks/{id}

---

## 6. Portfolios (Protected)
Headers: `Authorization: Bearer <token>`

- GET /app/portfolios?page={page}&per_page={per_page}
  - Response: paginated list of portfolios
  - Portfolio item example:
```json
{
  "id": 1,
  "judul": "Fitur Dark Mode",
  "deskripsi": "...",
  "is_active": true,
  "created_at": "2025-01-01T10:00:00Z",
  "updated_at": "2025-01-01T10:00:00Z",
  "project": { "id": 1, "nama_projek": "My Portfolio Website" }
}
```

- POST /app/portfolios
  - Request example:
```json
{ "project_id": 1, "judul": "Feature X", "deskripsi": "...", "is_active": true }
```

- GET /app/portfolios/{id}
- PUT /app/portfolios/{id}
  - Request example: same fields as create (all optional for update)
- DELETE /app/portfolios/{id}

---

## Error handling notes
- Validation errors return 400 with `errors` map detailing field errors.
- Not found returns 404 with message (e.g., "Category not found").
- Server errors return 500 with message string.

## Quick tips
- Use the Swagger UI (`/swagger-ui/`) to explore request/response schemas generated from the code.
- Protected endpoints require `Authorization: Bearer <token>` where token is obtained from `/login` response.

---

Jika Anda ingin, saya bisa:
- Menambahkan contoh curl untuk tiap endpoint.
- Mengekspor file OpenAPI JSON/YAML yang dapat diimpor ke Postman (app sudah expose `/api-docs/openapi.json`).
- Menambahkan tabel ringkas endpoint (method + path + auth) di bagian atas.

  }
}
```
