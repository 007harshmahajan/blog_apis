# Blog API - Rust Backend with Diesel ORM

A production-ready blog API backend built with Rust, Rocket, Diesel ORM, and PostgreSQL. This implementation demonstrates advanced Diesel ORM usage for complex scenarios including array aggregation, LEFT JOINs, and efficient pagination.

## Features

### Subtask 1: Core API Implementation
- Create User API: POST `/api/users`
- Create Post API: POST `/api/posts` 
- List Posts API: GET `/api/posts` with pagination and search
- Database Schema: Users and Posts tables with proper relationships
- Pagination: Complete metadata with current_page, per_page, from, to, total_pages, total_docs

### Subtask 2: Tags System with Array Aggregation
- Many-to-Many Relationship: Posts ↔ Tags via junction table
- Array Aggregation: Single query with PostgreSQL `ARRAY_AGG` for tag collection
- Create Posts with Tags: API accepts tags array in request
- Efficient Queries: No N+1 problems, single query architecture
- Diesel Integration: Minimal raw SQL, maximum Diesel ORM usage

### Subtask 3: Enhanced User Information
- Created By Structure: Rich user information in post responses
- LEFT JOIN Implementation: Handles system posts (NULL created_by) gracefully
- Single Query Architecture: All data retrieved in one efficient query
- System Posts Support: NULL created_by for system-generated content

## Technology Stack

- Language: Rust 1.70+
- Framework: Rocket 0.5
- ORM: Diesel 2.1 with PostgreSQL
- Database: PostgreSQL 12+
- Serialization: Serde + Serde JSON
- UUID: UUID v4 for primary keys
- Chrono: DateTime handling with UTC timestamps

## Prerequisites

### System Requirements
- Rust: Latest stable version (1.70+)
- PostgreSQL: 12+ with client libraries
- Diesel CLI: For database migrations
- jq: For JSON formatting (optional, for demo)

### Installation Commands

#### macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
brew install postgresql
brew services start postgresql

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Install jq (for demo)
brew install jq
```

#### Ubuntu/Debian
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib libpq-dev

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Install jq (for demo)
sudo apt-get install jq
```

## Quick Start

### Automated Setup (Recommended)
```bash
# Clone the repository
git clone <repository-url>
cd blog_apis

# Setup and start the server
./setup.sh

# In another terminal, run the comprehensive demo
./demo.sh
```

### Manual Setup
```bash
# 1. Create database
createdb blog_db

# 2. Set environment variables
echo "DATABASE_URL=postgres://localhost/blog_db" > .env

# 3. Run migrations
diesel migration run

# 4. Build and run
cargo run
```

## API Documentation

### Base URL
```
http://127.0.0.1:8000/api
```

### 1. Create User
**POST** `/api/users`

**Request Body:**
```json
{
  "username": "john_doe",
  "first_name": "John",
  "last_name": "Doe"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "uuid-here",
    "username": "john_doe",
    "first_name": "John",
    "last_name": "Doe",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

### 2. Create Post with Tags
**POST** `/api/posts`

**Request Body:**
```json
{
  "title": "My First Blog Post",
  "body": "This is the content of my first blog post...",
  "created_by": "user-uuid-here",
  "tags": ["rust", "programming", "backend"]
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "post-uuid-here",
    "title": "My First Blog Post",
    "body": "This is the content of my first blog post...",
    "created_by": "user-uuid-here",
    "created_at": "2024-01-01T00:00:00Z"
  }
}
```

### 3. List Posts with Pagination and Search
**GET** `/api/posts?page=1&limit=10&search=rust`

**Query Parameters:**
- `page` (optional): Page number (default: 1)
- `limit` (optional): Items per page (default: 10)
- `search` (optional): Search term across title, body, tags, user fields

**Response:**
```json
{
  "success": true,
  "data": {
    "records": [
      {
        "id": "post-uuid-here",
        "title": "My First Blog Post",
        "body": "This is the content...",
        "created_by": {
          "user_id": "user-uuid-here",
          "username": "john_doe",
          "first_name": "John",
          "last_name": "Doe"
        },
        "created_at": "2024-01-01T00:00:00Z",
        "tags": ["rust", "programming", "backend"]
      },
      {
        "id": "system-post-uuid",
        "title": "System Generated Post",
        "body": "This post was created by the system...",
        "created_by": null,
        "created_at": "2024-01-01T00:00:00Z",
        "tags": ["system", "announcement"]
      }
    ],
    "meta": {
      "current_page": 1,
      "per_page": 10,
      "from": 1,
      "to": 10,
      "total_pages": 5,
      "total_docs": 56
    }
  }
}
```

## Database Schema

### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR NOT NULL UNIQUE,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Posts Table
```sql
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    created_by UUID REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Posts_Tags Table (Junction Table)
```sql
CREATE TABLE posts_tags (
    fk_post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag VARCHAR NOT NULL,
    PRIMARY KEY (fk_post_id, tag)
);
```

## Project Structure

```
blog_apis/
├── src/
│   ├── main.rs          # Application entry point
│   ├── models.rs        # Data models and structs
│   ├── schema.rs        # Database schema (auto-generated)
│   ├── db.rs           # Database connection setup
│   ├── repository.rs   # Database operations layer
│   └── handlers.rs     # API endpoint handlers
├── migrations/          # Database migration files
│   ├── 2024_01_01_000001_create_users_table/
│   ├── 2024_01_01_000002_create_posts_table/
│   └── 2024_01_01_000003_create_posts_tags_table/
├── setup.sh            # Complete setup and installation
├── demo.sh             # Comprehensive demo script
├── Cargo.toml          # Rust dependencies
├── Rocket.toml         # Rocket configuration
└── README.md           # This file
```

## Testing & Demo

### Run Comprehensive Demo
```bash
# Setup and start server
./setup.sh

# In another terminal, run demo
./demo.sh
```

### Manual Testing
```bash
# Start server
cargo run

# In another terminal, run tests
./demo.sh
```

### Individual API Tests
```bash
# Create user
curl -X POST "http://127.0.0.1:8000/api/users" \
  -H "Content-Type: application/json" \
  -d '{"username": "testuser", "first_name": "Test", "last_name": "User"}'

# Create post with tags
curl -X POST "http://127.0.0.1:8000/api/posts" \
  -H "Content-Type: application/json" \
  -d '{"title": "Test Post", "body": "Test content", "created_by": "USER_ID_HERE", "tags": ["rust", "test"]}'

# List posts with pagination
curl "http://127.0.0.1:8000/api/posts?page=1&limit=5"

# Search posts
curl "http://127.0.0.1:8000/api/posts?search=rust"
```

## Development

### Database Operations
```bash
# Create new migration
diesel migration generate migration_name

# Run migrations
diesel migration run

# Revert last migration
diesel migration revert

# Reset database
diesel database reset

# Show database schema
diesel print-schema > src/schema.rs
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests
cargo test

# Check for unused dependencies
cargo check
```

## Advanced Features

### Array Aggregation Implementation
The implementation uses PostgreSQL's `ARRAY_AGG` function within a single Diesel query:

```sql
COALESCE(ARRAY_AGG(DISTINCT pt.tag) FILTER (WHERE pt.tag IS NOT NULL), '{}') as tags
```

### LEFT JOIN for User Information
Handles optional user relationships for system posts:

```sql
LEFT JOIN users u ON p.created_by = u.id
```

### Efficient Search
Multi-field search across posts, users, and tags:

```sql
WHERE (search_term IS NULL OR 
       p.title ILIKE search_term OR 
       p.body ILIKE search_term OR 
       u.username ILIKE search_term OR 
       pt.tag ILIKE search_term)
```

## Performance Optimizations

- Single Query Architecture: All data retrieved in one query
- Proper Indexing: Optimized indexes for posts_tags relationships
- Connection Pooling: Diesel r2d2 for efficient connection management
- Efficient Pagination: Count and data queries optimized separately
- No N+1 Problems: Array aggregation prevents multiple queries

## Troubleshooting

### Common Issues

**1. PostgreSQL Connection Error**
```bash
# Check if PostgreSQL is running
pg_isready

# Start PostgreSQL service
brew services start postgresql  # macOS
sudo systemctl start postgresql  # Linux
```

**2. Diesel CLI Not Found**
```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

**3. Database Migration Errors**
```bash
# Reset database and run migrations
diesel database reset
diesel migration run
```

**4. Port Already in Use**
```bash
# Kill existing processes
pkill -f "blog_apis"
# Or change port in Rocket.toml
```

## License

This project is licensed under the MIT License.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## Support

For questions or issues, please open an issue in the repository.
