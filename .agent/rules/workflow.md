# Workflow for New Features
1.  **Domain**: Define Entities and Repository Trait.
2.  **Infrastructure**: Implement Repository Trait.
3.  **Interface**: Define Request/Response DTOs with Validation.
4.  **Application**: Implement Use Cases using Repository Trait.
5.  **Interface**: Implement Controller to wire DTOs and Use Cases.
6.  **Wiring**: Add to `Container` in `di.rs` and `routes.rs`.
7.  **Testing**: Implement Integration Tests in `src/test/` ensuring coverage.
