#!/bin/bash

# Blog API Setup Script
# Complete setup and installation for the blog API demo

set -e  # Exit on any error

echo "Blog API Setup"
echo "=============="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_step() {
    echo -e "${PURPLE}[STEP]${NC} $1"
}

print_info() {
    echo -e "${CYAN}[INFO]${NC} $1"
}

# Check if required tools are installed
check_requirements() {
    print_step "Checking system requirements..."
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Check for PostgreSQL
    if ! command -v psql &> /dev/null; then
        print_error "PostgreSQL is not installed. Please install PostgreSQL first:"
        echo "  macOS: brew install postgresql"
        echo "  Ubuntu: sudo apt-get install postgresql postgresql-contrib"
        echo "  CentOS: sudo yum install postgresql postgresql-server"
        exit 1
    fi
    
    # Check for Diesel CLI
    if ! command -v diesel &> /dev/null; then
        print_warning "Diesel CLI not found. Installing..."
        cargo install diesel_cli --no-default-features --features postgres
    fi
    
    # Check for jq (for JSON formatting)
    if ! command -v jq &> /dev/null; then
        print_warning "jq not found. Installing..."
        if [[ "$OSTYPE" == "darwin"* ]]; then
            brew install jq
        elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
            sudo apt-get install jq
        fi
    fi
    
    print_success "All requirements satisfied!"
}

# Setup database
setup_database() {
    print_step "Setting up database..."
    
    # Check if PostgreSQL is running
    if ! pg_isready -q; then
        print_error "PostgreSQL is not running. Please start PostgreSQL first:"
        echo "  macOS: brew services start postgresql"
        echo "  Ubuntu: sudo systemctl start postgresql"
        exit 1
    fi
    
    # Create database if it doesn't exist
    if ! psql -lqt | cut -d \| -f 1 | grep -qw blog_db; then
        print_status "Creating database 'blog_db'..."
        createdb blog_db
        print_success "Database created successfully!"
    else
        print_success "Database 'blog_db' already exists!"
    fi
    
    # Set up environment variables
    if [ ! -f .env ]; then
        print_status "Creating .env file..."
        echo "DATABASE_URL=postgres://localhost/blog_db" > .env
        print_success ".env file created!"
    fi
    
    # Run migrations
    print_status "Running database migrations..."
    diesel migration run
    print_success "Migrations completed!"
}

# Build the project
build_project() {
    print_step "Building the project..."
    cargo build
    print_success "Project built successfully!"
}

# Test the setup
test_setup() {
    print_step "Testing setup..."
    
    # Test database connection
    if psql -d blog_db -c "SELECT 1;" > /dev/null 2>&1; then
        print_success "Database connection working"
    else
        print_error "Database connection failed"
        exit 1
    fi
    
    # Test if tables exist
    TABLE_COUNT=$(psql -d blog_db -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public';" | tr -d ' ')
    if [ "$TABLE_COUNT" -ge 3 ]; then
        print_success "Database tables created ($TABLE_COUNT tables found)"
    else
        print_error "Database tables not found"
        exit 1
    fi
    
    # Test if project compiles
    if cargo check > /dev/null 2>&1; then
        print_success "Project compiles successfully"
    else
        print_error "Project compilation failed"
        exit 1
    fi
}

# Start the server
start_server() {
    print_step "Starting the server..."
    
    # Kill any existing server process
    pkill -f "blog_apis" || true
    
    # Start server in background
    cargo run &
    SERVER_PID=$!
    
    # Wait for server to start
    print_status "Waiting for server to start..."
    sleep 5
    
    # Check if server is running
    if curl -s http://127.0.0.1:8000/api/posts > /dev/null; then
        print_success "Server is running on http://127.0.0.1:8000"
        echo $SERVER_PID > .server_pid
        return 0
    else
        print_error "Failed to start server"
        return 1
    fi
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    
    # Kill server if running
    if [ -f .server_pid ]; then
        SERVER_PID=$(cat .server_pid)
        kill $SERVER_PID 2>/dev/null || true
        rm -f .server_pid
    fi
    
    # Kill any remaining blog_apis processes
    pkill -f "blog_apis" 2>/dev/null || true
}

# Main execution
main() {
    # Set up cleanup on script exit
    trap cleanup EXIT
    
    echo ""
    print_info "Starting Blog API Setup..."
    echo ""
    
    # Check requirements
    check_requirements
    echo ""
    
    # Setup database
    setup_database
    echo ""
    
    # Build project
    build_project
    echo ""
    
    # Test setup
    test_setup
    echo ""
    
    # Start server
    if start_server; then
        print_success "Setup completed successfully!"
        echo ""
        echo "Blog API is ready!"
        echo "Server running on: http://127.0.0.1:8000"
        echo "API Endpoints:"
        echo "   POST /api/users - Create user"
        echo "   POST /api/posts - Create post with tags"
        echo "   GET  /api/posts - List posts with pagination and search"
        echo ""
        echo "Run './demo.sh' to start the demo"
        echo ""
        echo "Press Ctrl+C to stop the server"
        
        # Keep server running
        wait
    else
        print_error "Setup failed"
        exit 1
    fi
}

# Run main function
main "$@"
