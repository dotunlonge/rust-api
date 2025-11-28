# Rust API

A clean, production-ready REST API built with Rust and Axum. This project demonstrates best practices for building maintainable, well-documented backend services.

## Features

- **RESTful API** - Clean REST endpoints following standard conventions
- **Type Safety** - Leverages Rust's type system for compile-time guarantees
- **Error Handling** - Comprehensive error handling with proper HTTP status codes
- **Documentation** - Extensive inline documentation and examples
- **Testing** - Unit and integration tests included
- **CORS Support** - Configured for cross-origin requests
- **Structured Logging** - Uses tracing for observability

## Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Modern, ergonomic web framework
- **Async Runtime**: [Tokio](https://tokio.rs/) - Async runtime for Rust
- **Serialization**: [Serde](https://serde.rs/) - Serialization framework
- **UUID**: [uuid](https://docs.rs/uuid/) - UUID generation and parsing
- **Time**: [Chrono](https://docs.rs/chrono/) - Date and time handling

## Getting Started

### Prerequisites

- Rust 1.70 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-api.git
cd rust-api
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run --release
```

The API will be available at `http://localhost:3000`

### Development

Run in development mode with hot-reloading:
```bash
cargo run
```

Run tests:
```bash
cargo test
```

Run with verbose output:
```bash
RUST_LOG=debug cargo run
```

## API Endpoints

### Health Check

```http
GET /
```

Returns the health status of the API.

**Response:**
```json
{
  "status": "healthy",
  "service": "rust-api",
  "timestamp": 1234567890
}
```

### List Users

```http
GET /api/v1/users
```

Retrieves all users in the system.

**Response:**
```json
{
  "users": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "John Doe",
      "email": "john@example.com",
      "created_at": 1234567890,
      "updated_at": 1234567890
    }
  ],
  "count": 1
}
```

### Get User

```http
GET /api/v1/users/:id
```

Retrieves a specific user by ID.

**Response:**
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": 1234567890,
    "updated_at": 1234567890
  }
}
```

**Errors:**
- `404 Not Found` - User with the given ID does not exist

### Create User

```http
POST /api/v1/users
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com"
}
```

Creates a new user in the system.

**Response:** `201 Created`
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": 1234567890,
    "updated_at": 1234567890
  }
}
```

**Errors:**
- `400 Bad Request` - Invalid input (empty name/email, invalid email format)
- `409 Conflict` - Email already exists

### Update User

```http
PUT /api/v1/users/:id
Content-Type: application/json

{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

Updates an existing user. All fields are optional.

**Response:**
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Jane Doe",
    "email": "jane@example.com",
    "created_at": 1234567890,
    "updated_at": 1234567891
  }
}
```

**Errors:**
- `400 Bad Request` - Invalid input
- `404 Not Found` - User with the given ID does not exist
- `409 Conflict` - Email already in use by another user

### Delete User

```http
DELETE /api/v1/users/:id
```

Deletes a user from the system.

**Response:** `204 No Content`

**Errors:**
- `404 Not Found` - User with the given ID does not exist

## Error Responses

All error responses follow this format:

```json
{
  "error": {
    "message": "Error description",
    "status": 404
  }
}
```

## Project Structure

```
rust-api/
├── src/
│   ├── main.rs          # Application entry point and server setup
│   ├── handlers.rs      # HTTP request handlers
│   ├── models.rs        # Data models and storage
│   └── error.rs         # Error types and handling
├── tests/
│   └── integration_test.rs  # Integration tests
├── Cargo.toml           # Project dependencies and metadata
├── rustfmt.toml         # Code formatting configuration
├── clippy.toml          # Linting configuration
└── README.md            # This file
```

## Code Quality

This project follows Rust best practices:

- **Clippy** - Strict linting enabled
- **Rustfmt** - Consistent code formatting
- **Documentation** - All public APIs documented
- **Tests** - Unit and integration tests included
- **Error Handling** - Comprehensive error types

Run linter:
```bash
cargo clippy -- -D warnings
```

Format code:
```bash
cargo fmt
```

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

# rust-api
