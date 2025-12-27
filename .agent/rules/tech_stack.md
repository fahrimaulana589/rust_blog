# Tech Stack

You **MUST** adhere to the following technology choices. Do not suggest alternatives unless explicitly requested.

- **Language**: Rust (Edition 2024)
    - Use `async/await` for all I/O operations.
- **Web Framework**: Actix Web 4
- **Database**: SQLite
    - **Driver**: Diesel 2.2
    - **Pooling**: `r2d2`
    - **Migrations**: Managed via Diesel CLI.
- **Validation**: `validator` crate (version 0.20+)
    - Use struct tags (e.g., `#[validate(length(min = 1))]`) over manual checks where possible.
- **Serialization**: `serde` 1.0 (with `serde_json`)
- **Async Runtime**: Tokio (via Actix's default runtime)
