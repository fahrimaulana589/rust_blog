# OpenAPI / Swagger UI Standards

### 1. Structure
- **Definition**: `src/app/drivers/openapi.rs`.
- **Derive**: Only on the `ApiDoc` struct.

### 2. Controllers
- **Path Attribute**: `#[utoipa::path(...)]` **MUST** be present on every public handler.
- **Response Types**:
    - **Success**: `body = crate::utils::success_response::SuccessResponse<FullPathToDto>`
    - **Empty**: Use `crate::utils::success_response::Empty` generic for no-data responses.
- **Tag**: Assign the correct feature tag (e.g., `tag = "Blog"`).

### 3. DTOs
- **Trait**: Derive `ToSchema`.
- **Registration**: Add DTO to `components(schemas(...))` in `openapi.rs`.

### 4. Security
- **JWT**: Add `security(("jwt" = []))` to `#[openapi(...)]` to enable the Authorize button.
