# Coding Standards

You **MUST** adhere to these coding standards.

### 1. Error Handling
- **Repositories**: Must return `diesel::QueryResult<T>`. Do NOT perform error conversion here.
- **Use Cases**: 
    - Return `Result<T, AppError>` (e.g., `BlogError`).
    - **Combined Validation**: Collect errors using `validator` struct validation + custom business logic validation. Return all errors in a single `ValidationErrors` map if possible.
- **Controllers**:
    - **Mapping**: Explicitly match error variants to HTTP responses.
        - Validation -> 400 Bad Request
        - Not Found -> 404 Not Found
        - System/Database -> 500 Internal Server Error

### 2. Dependency Injection
- Use the `crate::utils::di::Container` struct.
- **Rules**:
    - **Register**: Add new fields to the `Container` struct.
    - **Initialize**: Instantiate Use Cases in `Container::new()`.
    - **Usage**: Inject `web::Data<Container>` into controllers. DO NOT instantiate Use Cases inside controllers.

### 3. Routing
- **Location**: `src/app/drivers/routes.rs`.
- **Grouping**: Use `web::scope("/app/feature")` where appropriate to organize routes.

### 4. Safety
- **Unwrap**: You **MUST NOT** use `.unwrap()` on `Option` or `Result` in production code (Application/Interface/Infrastructure layers) unless you can mathematically prove it will never panic. Use `expect` with a descriptive message or `?` propagation.
- **Tests**: `.unwrap()` is allowed in `src/test/` for assertions.
