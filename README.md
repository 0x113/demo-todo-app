# Todo App RS

A simple Rust-based todo application built with Actix Web and PostgreSQL. This project serves as a demonstration of basic CRUD operations and modern Rust web development practices, featuring both a REST API and a web UI.

⚠️ **Note**: This is a simple demonstration project and is not intended for production use.

## Features

- 🌐 Modern web UI for managing todos
- ✅ Create new todo items
- 📋 List all todo items  
- ✅ Mark todo items as completed
- 🗑️ Delete todo items
- 🗄️ PostgreSQL database persistence
- 🐳 Docker containerization
- 📊 Structured logging with tracing

## Tech Stack

- **Language**: Rust 2024 Edition
- **Web Framework**: Actix Web 4.x
- **Template Engine**: Tera
- **Database**: PostgreSQL with SQLx
- **Serialization**: Serde
- **Async Runtime**: Tokio
- **Logging**: Tracing + Tracing Subscriber
- **Containerization**: Docker

## Project Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── templates/           # Tera HTML templates
│   └── index.html       # Main UI template
├── common/              # Shared utilities and state
└── todo/                # Todo module
    ├── handlers.rs      # API request handlers
    ├── web_handlers.rs  # Web UI handlers
    ├── service.rs       # Business logic
    ├── repository.rs    # Database operations
    └── model.rs         # Data models
```

## Prerequisites

- Rust 1.70+ (with Cargo)
- PostgreSQL 12+
- Docker & Docker Compose (optional)

## Environment Variables

The application requires the following environment variables:

```bash
HOST=127.0.0.1
PORT=8080
DB_HOST=localhost
DB_PORT=5432
DB_USER=your_db_user
DB_PASSWORD=your_db_password
DB_NAME=todo_app
```

## Quick Start

### Option 1: Local Development

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd todo-app-rs
   ```

2. **Set up PostgreSQL database**
   ```bash
   # Create database (adjust connection details as needed)
   psql -h localhost -U postgres -c "CREATE DATABASE todo_app;"
   ```

3. **Set environment variables**
   ```bash
   export HOST=127.0.0.1
   export PORT=8080
   export DB_HOST=localhost
   export DB_PORT=5432
   export DB_USER=your_db_user
   export DB_PASSWORD=your_db_password
   export DB_NAME=todo_app
   ```

4. **Run the application**
   ```bash
   cargo run
   ```

5. **Access the application**
   - **Web UI**: http://localhost:8080
   - **REST API**: http://localhost:8080/api/v1

### Option 2: Docker

```bash
# Build the Docker image
docker build -t todo-app-rs .

# Run with environment variables
docker run -p 8080:8080 \
  -e HOST=0.0.0.0 \
  -e PORT=8080 \
  -e DB_HOST=your_db_host \
  -e DB_PORT=5432 \
  -e DB_USER=your_db_user \
  -e DB_PASSWORD=your_db_password \
  -e DB_NAME=todo_app \
  todo-app-rs
```

## Web UI

The application provides a modern web interface accessible at the root URL:

- **URL**: http://localhost:8080
- **Features**: 
  - Add new todos with a simple form
  - View all todos in a clean list
  - Mark todos as completed
  - Delete todos
  - Real-time updates after each action

## API Documentation

The application also exposes a REST API with the following endpoints:

### Base URL
```
http://localhost:8080/api/v1
```

### Endpoints

#### 1. Create a new todo
- **POST** `/tasks`
- **Body**:
  ```json
  {
    "title": "Learn Rust"
  }
  ```
- **Response**: `200 OK` (empty body)

#### 2. List all todos
- **GET** `/tasks`
- **Response**:
  ```json
  [
    {
      "id": 1,
      "title": "Learn Rust",
      "completed": false
    }
  ]
  ```

#### 3. Mark todo as completed
- **POST** `/tasks/{id}/done`
- **Response**: `200 OK` (empty body)

#### 4. Delete a todo
- **DELETE** `/tasks/{id}`
- **Response**: `200 OK` (empty body)

## Usage Examples

### Using cURL

```bash
# Create a new todo
curl -X POST http://localhost:8080/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust"}'

# Get all todos
curl http://localhost:8080/api/v1/tasks

# Mark todo as done (assuming ID is 1)
curl -X POST http://localhost:8080/api/v1/tasks/1/done

# Delete a todo (assuming ID is 1)
curl -X DELETE http://localhost:8080/api/v1/tasks/1
```

## Database Schema

The application automatically creates the following table on startup:

```sql
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
```

## Development

### Building
```bash
cargo build
```

### Running tests (if available)
```bash
cargo test
```

### Code formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## License

This project is intended for demonstration purposes only.

## Contributing

This is a demo project. Feel free to fork and experiment with it for learning purposes.
