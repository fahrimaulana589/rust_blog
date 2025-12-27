# Testing Standards (MANDATORY)

You **MUST** follow these testing rules without exception.

### 1. Test Location & Scope
- **Integration Tests**:
    - **File**: `src/test/[feature].rs`.
    - **Scope**: End-to-end (Controller -> UseCase -> DB).
    - **Requirement**: Use `init_test_app!`.

### 2. Concurrency Control (CRITICAL)
- **Rule**: SQLite locks on concurrent writes.
- **Requirement**: annotating every test function with `#[serial]`.
    - **Import**: `use serial_test::serial;`
    - **Usage**:
      ```rust
      #[actix_web::test]
      #[serial]
      async fn test_example() { ... }
      ```
- **Penalty**: If a test fails with "Database is locked", you violated this rule.

### 3. Data Integrity & Uniqueness
- **Unique Fields**: For fields with unique constraints (username, projects.title, etc.), you **MUST** append a dynamic suffix.
    - **Pattern**: `format!("Name {}", chrono::Utc::now().timestamp_micros())`
    - **Reason**: Prevents "Duplicate entry" errors when running tests repeatedly or in parallel.

### 4. Isolation
- **One Test Per Path**: Write exactly one test function per API endpoint scenario.
    - **DO NOT** chain unrelated requests (e.g. Create -> Delete -> Update) in one test unless verifying a specific workflow.
    - **Isolation**: Each test must seed its own user/data.

### 5. Type Safety
- **Requests**: Use DTO structs (e.g., `CreateUserRequestDto`), **NEVER** `serde_json::json!`.
- **Responses**: Parse into `SuccessResponse<DTO>`.
- **Assertions**: Assert strictly on the returned DTO fields.
