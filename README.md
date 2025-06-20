# Todo App RS

A simple Rust-based todo application built with Actix Web and PostgreSQL.

⚠️ **Important**: This is a **demonstration project** created for learning purposes. It's not intended for production use and serves as an example of building a simple web application with Rust, Actix Web, and PostgreSQL, including containerization and Kubernetes deployment.

## Features

- 🌐 UI for managing todos
- ✅ Create new todo items
- 📋 List all todo items  
- ✅ Mark todo items as completed
- 🗑️ Delete todo items
- 🗄️ PostgreSQL database persistence
- 🐳 Docker containerization
- ☸️ Kubernetes deployment ready
- 📊 Structured logging with tracing

## Tech Stack

- **Language**: Rust 2024 Edition
- **Web Framework**: Actix Web 4.x
- **Template Engine**: Tera
- **Database**: PostgreSQL with SQLx
- **Serialization**: Serde
- **Async Runtime**: Tokio
- **Logging**: Tracing
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
- Kubernetes cluster (optional, for K8s deployment)

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

### Option 3: Kubernetes Deployment

The project includes complete Kubernetes manifests for deployment in a cluster.

#### Prerequisites for Kubernetes deployment:
- Access to a Kubernetes cluster (local or cloud)
- `kubectl` configured to access your cluster
- Docker image built and available to your cluster

#### Deploy to Kubernetes:

1. **Build and load the Docker image** (for local clusters like minikube/kind):
   ```bash
   # Build the image
   docker build -t todo-app-rs:latest .
   
   # For minikube
   minikube image load todo-app-rs:latest
   
   # For kind
   kind load docker-image todo-app-rs:latest
   ```

2. **Update database credentials** in `kubernetes/postgres-secret.yaml`:
   ```bash
   # Edit the secret file with your database credentials
   # Note: Values should be base64 encoded
   kubectl create secret generic postgres-secret \
     --from-literal=POSTGRES_USER=your_db_user \
     --from-literal=POSTGRES_PASSWORD=your_db_password \
     --dry-run=client -o yaml > kubernetes/postgres-secret.yaml
   ```

3. **Deploy using Kustomize**:
   ```bash
   # Apply all Kubernetes resources
   kubectl apply -k kubernetes/
   ```

4. **Verify deployment**:
   ```bash
   # Check if pods are running
   kubectl get pods -n todo-app
   
   # Check services
   kubectl get services -n todo-app
   ```

5. **Access the application**:
   ```bash
   # Port forward to access locally
   kubectl port-forward -n todo-app service/todo-app-service 8080:80
   
   # Or get the external IP (if using LoadBalancer)
   kubectl get service todo-app-service -n todo-app
   ```

#### Kubernetes Resources Included:
- **Namespace**: `todo-app` namespace for resource isolation
- **PostgreSQL**: Complete PostgreSQL deployment with persistent volume
- **Todo App**: Application deployment with 2 replicas
- **ConfigMap**: Environment configuration
- **Secret**: Database credentials
- **Services**: ClusterIP services for internal communication
- **PVC**: Persistent volume claim for PostgreSQL data

#### Cleanup:
```bash
# Remove all resources
kubectl delete -k kubernetes/
```

**Note**: The Kubernetes configuration is set up for demonstration purposes with `imagePullPolicy: Never` for local development. For production deployments, you would need to:
- Push the Docker image to a container registry
- Update the image reference in the deployment
- Configure appropriate resource limits and requests
- Set up proper secrets management
- Configure ingress controllers for external access

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

This is a **demonstration project** created for educational purposes. Feel free to fork and experiment with it for learning about:
- Rust web development with Actix Web
- PostgreSQL integration with SQLx
- Docker containerization
- Kubernetes deployment patterns
- REST API design
- HTML templating with Tera

Pull requests and improvements are welcome for educational enhancements!
