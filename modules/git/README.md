# Tardigrade Git Module

Git repository management module for Tardigrade-CI.

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 15+
- Docker (optional, for running PostgreSQL)

### Setup

1. **Clone and navigate**
   ```bash
   cd modules/git
   ```

2. **Configure database**
   - Create a PostgreSQL database:
     ```bash
     createdb tardigrade
     ```
   - Or use Docker:
     ```bash
     docker run --name tardigrade-db -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres
     createdb -h localhost -U postgres tardigrade
     ```

3. **Set environment variables**
   ```bash
   export DATABASE_URL=postgres://postgres:password@localhost:5432/tardigrade
   ```
   Or copy `.env.example` to `.env` and update the values.

4. **Prepare SQLx queries**
   ```bash
   cargo sqlx prepare
   ```

5. **Run migrations**
   ```bash
   cargo sqlx migrate run
   ```

6. **Build and run**
   ```bash
   cargo build
   cargo run
   ```

7. **Test**
   ```bash
   cargo test --lib
   ```

## 📚 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/repositories` | Create a new repository |
| GET | `/repositories` | List all repositories (with pagination) |
| GET | `/repositories/:id` | Get repository by ID |
| PUT | `/repositories/:id` | Update a repository |
| DELETE | `/repositories/:id` | Delete a repository |
| GET | `/health` | Health check |
| GET | `/api/info` | API information |

## 📁 Project Structure

```
modules/git/
├── Cargo.toml                 # Rust package configuration
├── .env.example               # Environment variables template
├── .cargo/
│   └── config.toml           # Cargo configuration
├── src/
│   ├── main.rs               # Entry point
│   ├── lib.rs                # Public API
│   ├── config.rs             # Configuration management
│   ├── error.rs              # Error types
│   ├── models.rs             # Data models
│   ├── repository.rs          # CRUD operations
│   ├── service.rs            # Service layer
│   ├── routes.rs             # API routes
│   └── handler/
│       ├── mod.rs            # Handler module
│       └── repository.rs     # Repository handlers
├── tests/
│   ├── unit/
│   │   └── repository.rs     # Unit tests
│   └── integration/
├── migrations/
│   └── 20260619000000_create_repositories.table.sql
└── README.md
```

## 🔧 Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://user:password@localhost:5432/tardigrade` | PostgreSQL connection URL |
| `SERVER_HOST` | `0.0.0.0` | Server host to bind to |
| `SERVER_PORT` | `3001` | Server port to bind to |
| `SERVER_ENABLE_CORS` | `true` | Enable CORS |
| `SERVER_LOG_LEVEL` | `info` | Log level |

## 🧪 Testing

### Unit Tests

Run unit tests (no database required):
```bash
cargo test --lib
```

### Integration Tests

Run integration tests (requires running database):
```bash
cargo test --test '*'
```

## 📝 Documentation

Generate documentation:
```bash
cargo doc --open
```

## ⚡ Performance

The module is optimized for:
- Fast CRUD operations with connection pooling
- Pagination support for listing repositories
- Efficient query execution with proper indexing

## 🔒 Security

- Input validation for all repository operations
- Ownership checks for update/delete operations
- Proper error handling and HTTP status codes
- UUID v4 for unique identifiers

## 📦 Dependencies

- `axum` - Web framework
- `sqlx` - PostgreSQL client
- `tokio` - Async runtime
- `serde` - Serialization
- `uuid` - Unique identifiers
- `thiserror` - Error handling
- `async-trait` - Async traits
- `chrono` - Time handling
- `tracing` - Logging

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Run clippy: `cargo clippy -- -D warnings`
6. Run fmt: `cargo fmt`
7. Submit a pull request

## 📄 License

AGPL-3.0
