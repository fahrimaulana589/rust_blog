# API Documentation

Berikut adalah dokumentasi lengkap endpoint API, termasuk format Request Body, Response Body Sukses, dan Response Gagal (Error).

Base URL: `http://localhost:8080` (sesuaikan dengan config)

## Common Response Formats

### Success Response
```json
{
  "message": "Operation successful",
  "data": { ... } // Optional
}
```

### Paginated Response
```json
{
  "message": "Data fetched successfully",
  "data": {
    "items": [ ... ],
    "meta": {
      "page": 1,
      "per_page": 10,
      "total_pages": 5,
      "total_items": 50
    }
  }
}
```

### Generic Error Response
```json
{
  "message": "Unauthorized access",
  "errors": null
}
```

### Validation Error Response
```json
{
  "message": "Validation Error",
  "errors": {
    "field_name": "Error description"
  }
}
```

---

## 1. Authentication

### Login
**POST** `/login`
*Access: Public*

**Request Body:**
```json
{
  "username": "admin",
  "password": "password123"
}
```

**Response (Success - 200 OK):**
```json
{
  "message": "Login successful",
  "data": {
    "username": "admin",
    "email": "admin@example.com",
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  }
}
```

**Response (Fail - 400 Bad Request):**
```json
{
  "message": "Validation Error",
  "errors": {
    "username": "Username is required"
  }
}
```

**Response (Fail - 401 Unauthorized):**
```json
{
  "message": "Invalid credentials",
  "errors": null
}
```

### Forgot Password
**POST** `/forgot-password`
*Access: Public*

**Request Body:**
```json
{
  "email": "user@example.com"
}
```

**Response (Success - 200 OK):**
```json
{
  "message": "Email sent successfully"
}
```

**Response (Fail - 400 Bad Request):**
```json
{
  "message": "Validation Error",
  "errors": {
    "email": "Email is invalid"
  }
}
```

### Reset Password
**POST** `/reset-password`
*Access: Public*

**Request Body:**
```json
{
  "token": "reset_token_from_email",
  "new_password": "newSecurePassword123!"
}
```

**Response (Success - 200 OK):**
```json
{
  "message": "Password reset successfully"
}
```

---

## 2. Profile (Protected)
*Headers: `Authorization: Bearer <token>`*

### Get Profile
**GET** `/app/profile`
*Access: Protected*

**Response (Success - 200 OK):**
```json
{
  "message": "Profile fetched successfully",
  "data": {
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
    "languages": [
      { "name": "Indonesian", "level": "Native" },
      { "name": "English", "level": "Professional" }
    ]
  }
}
```
*Note: Jika profile belum dibuat, `data` akan `null`, tapi status tetap 200 OK.*

**Response (Fail - 401 Unauthorized):**
```json
{
  "message": "Unauthorized",
  "errors": null
}
```

### Upsert Profile
**POST** `/app/profile`
*Access: Protected*

**Request Body:**
```json
{
  "full_name": "Fahri Maulana",
  "headline": "Senior Rust Developer",
  "summary": "Updated summary...",
  "role": "Backend Engineer",
  "location": "Jakarta",
  "profile_image": "url",
  "availability": "Full-time",
  "years_of_experience": 5,
  "resume_url": "url",
  "email": "valid@email.com",
  "work_philosophy": "KISS",
  "timezone": "UTC+7",
  "specializations": ["Rust", "Go"],
  "tech_focus": ["WebAssembly"],
  "languages": [
      { "name": "English", "level": "Fluent" }
  ]
}
```

**Response (Success - 200 OK):**
```json
{
  "message": "Profile upserted successfully",
  "data": { ... } // Sama seperti Get Profile
}
```

**Response (Fail - 400 Validation Error):**
```json
{
  "message": "Validation Error",
  "errors": {
    "email": "Invalid email format",
    "full_name": "Full name is required"
  }
}
```

---

## 3. Blog (Protected)
*Headers: `Authorization: Bearer <token>`*

### Categories

#### Get Categories
**GET** `/app/categories?page=1&per_page=10`

**Response (Success - 200 OK):**
```json
{
  "message": "Categories fetched successfully",
  "data": {
    "items": [
      {
        "id": 1,
        "name": "Rust",
        "created_at": "2025-01-01T10:00:00",
        "updated_at": "2025-01-01T10:00:00"
      }
    ],
    "meta": { "page": 1, "per_page": 10, "total_pages": 1, "total_items": 1 }
  }
}
```

#### Create Category
**POST** `/app/categories`

**Request Body:**
```json
{
  "name": "Tutorial"
}
```

**Response (Success - 201 Created):**
```json
{
  "message": "Category created successfully",
  "data": { "id": 2, "name": "Tutorial", ... }
}
```

**Response (Fail - 400 Validation):**
```json
{
  "message": "Validation Error",
  "errors": { "name": "Name is required" }
}
```

#### Update Category
**PUT** `/app/categories/{id}`

**Response (Fail - 404 Not Found):**
```json
{
  "message": "Category not found",
  "errors": null
}
```

---

### Blogs

#### Create Blog
**POST** `/app/blogs`

**Request Body:**
```json
{
  "title": "Belajar Rust Dasar",
  "content": "Isi artikel...",
  "category_id": 1,
  "tag_ids": [1, 2],
  "excerpt": "Singkat cerita...",
  "thumbnail": "image_url",
  "status": "DRAFT"
}
```

**Response (Success - 200 OK):**
```json
{
  "message": "Blog created successfully",
  "data": {
    "id": 1,
    "title": "Belajar Rust Dasar",
    "slug": "belajar-rust-dasar",
    "status": "DRAFT",
    ...
  }
}
```

**Response (Fail - 400 Validation):**
```json
{
  "message": "Validation Error",
  "errors": {
    "title": "Title is required",
    "content": "Content is required"
  }
}
```

---

## 4. Projects & Portfolio (Protected)
*Headers: `Authorization: Bearer <token>`*

### Projects

#### Create Project
**POST** `/app/projects`

**Request Body:**
```json
{
  "nama_projek": "My Portfolio Website",
  "deskripsi": "Website personal...",
  "status": "ongoing",
  "progress": 50,
  "link_demo": "https://...",
  "repository": "https://github.com/...",
  "tanggal_mulai": "2025-01-01",
  "tanggal_selesai": null,
  "stack_ids": [1, 3]
}
```

**Response (Success - 200 OK):**
```json
{
  "message": "Project created successfully",
  "data": {
    "id": 1,
    "nama_projek": "My Portfolio Website",
    "status": "ongoing",
    ...
  }
}
```

### Portfolios

#### Get Portfolios
**GET** `/app/portfolios`

**Response (Success - 200 OK):**
```json
{
  "message": "Data fetched successfully",
  "data": {
    "items": [
      {
        "id": 1,
        "judul": "Fitur Dark Mode",
        "project": { "id": 1, "nama_projek": "My Portfolio", ... }
        ...
      }
    ],
    "meta": { ... }
  }
}
```
