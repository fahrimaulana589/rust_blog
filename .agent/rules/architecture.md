# Architecture

The project follows a strict **Clean Architecture** pattern. You **MUST** ensure code is placed in the correct layer and dependencies point inwards (or use Dependency Injection).

### Directory Structure
```
src/
  app/
    features/
      [feature_name]/
        interface/       # [Presentation] Controllers, DTOs
        application/     # [Business Logic] Use Cases
        domain/          # [Enterprise Logic] Entities, Repository Traits
        infrastructure/  # [Data Access] Repository Implementations
    drivers/             # Framework drivers (routes, middlewares)
  utils/                 # Shared utilities
  config/                # App configuration
  schema.rs              # Diesel schema
```

### Layer Responsibilities & Rules

#### 1. Interface Layer (`interface/`)
**Goal**: Handle HTTP/User interaction.
- **Controllers**:
    - **MUST** be named `controller.rs` (singular).
    - **MUST** return `impl Responder`.
    - **DO NOT** contain business logic.
    - **DO NOT** access the database directly.
    - **MUST** accept `web::Data<Container>` and call Use Cases.
    - **Response Format**: 
        - Success: `crate::utils::success_response::SuccessResponse<T>`.
        - Error: `crate::utils::error_response::ErrorResponse`.
- **DTOs**:
    - **MUST** be defined in `dto.rs` (or `dto/mod.rs`).
    - **Create/Update Responses**: **MUST** return the full entity data (e.g., `CategoryResponseDto`), **NEVER** just a string or empty body.
    - **Pagination**: List endpoints **MUST** use `PaginationRequestDto` and return `PaginatedResponseDto`.
    - **Validation**: Use `validator` derive macros. Controller **MUST** call `.validate()` before use.

#### 2. Application Layer (`application/`)
**Goal**: Application-specific business rules.
- **Use Cases**:
    - **Structure**: `struct Execute { repository: Arc<dyn Trait> }`.
    - **Method**: `async fn execute(&self, ...)`.
    - **Validation**:
        - Perform complex validation here (uniqueness, business rules).
        - **MUST** fail if invalid.
    - **Output**: Return `Result<ResponseDto, AppError>`.
    - **Dependency**: **MUST** depend on `domain::repository::RepositoryTrait`, **NEVER** on `infrastructure::RepositoryImpl`.

#### 3. Domain Layer (`domain/`)
**Goal**: Enterprise business logic and contracts.
- **Entities**: Diesel structs.
- **Repository Traits**: Define abstract DB operations.
    - **Return Type**: `diesel::QueryResult<T>` or `diesel::QueryResult<Option<T>>`.
    - **DO NOT** depend on actix, serde, or other infra details if possible.

#### 4. Infrastructure Layer (`infrastructure/`)
**Goal**: Implement interfaces defined in Domain.
- **Repositories**:
    - **Implement**: `domain::RepositoryTrait`.
    - **Dependencies**: `DbPool`.
    - **Logic**: **DO NOT** put business logic here. Only SQL/Diesel queries.
