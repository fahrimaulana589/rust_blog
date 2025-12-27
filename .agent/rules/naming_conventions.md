# Naming Conventions

You **MUST** follow these naming conventions strictly.

### 1. Files & Modules
- **Feature Modules**: `snake_case` (e.g., `blog`, `auth`).
- **Controller File**: `src/app/features/[feature]/interface/controller.rs` (**SINGULAR**).
- **DTO File**: `src/app/features/[feature]/interface/dto.rs`.
- **Use Case Directories**:
    - `[entity]_usecase` if module has multiple entities (e.g., `stack_usecase`, `project_usecase`).
    - `usecase` if module has only one main entity.

### 2. Structs & Classes
- **Request DTO**: `[Action][Entity]RequestDto` (e.g., `CreateCategoryRequestDto`, `UpdateBlogRequestDto`).
- **Response DTO**: `[Entity]ResponseDto` (e.g., `CategoryResponseDto`).
- **Repository Trait**: `[Entity]Repository` (e.g., `BlogRepository`).
- **Repository Impl**: `[Entity]RepositoryImpl` (e.g., `BlogRepositoryImpl`).

### 3. Functions
- **Tests**: `test_[action]_[entity]_[scenario]` (e.g., `test_create_project_duplicate_name`).
- **Use Case Method**: `execute`.
