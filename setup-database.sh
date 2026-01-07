#!/bin/bash

echo "🐘 Setting up PostgreSQL Database"
echo "================================="

# Database configuration
DB_NAME="loginapp"
DB_USER="postgres"
DB_PASSWORD="password"
DB_HOST="localhost"
DB_PORT="5432"

# Check if PostgreSQL is running
if ! sudo systemctl is-active --quiet postgresql; then
    echo "🔧 Starting PostgreSQL service..."
    sudo systemctl start postgresql
    sudo systemctl enable postgresql
fi

echo "👤 Setting up database and user..."

# Create database and configure user
sudo -u postgres psql << EOF
-- Create database if it doesn't exist
SELECT 'CREATE DATABASE ${DB_NAME}'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = '${DB_NAME}')\gexec

-- Set password for postgres user
ALTER USER ${DB_USER} PASSWORD '${DB_PASSWORD}';

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE ${DB_NAME} TO ${DB_USER};

\q
EOF

echo "📋 Running migrations..."

# Run each migration file
for migration in migrations/*.sql; do
    if [ -f "$migration" ]; then
        echo "  Running $(basename "$migration")..."
        PGPASSWORD=${DB_PASSWORD} psql -h ${DB_HOST} -p ${DB_PORT} -U ${DB_USER} -d ${DB_NAME} -f "$migration"
        
        if [ $? -eq 0 ]; then
            echo "  ✅ $(basename "$migration") completed successfully"
        else
            echo "  ❌ $(basename "$migration") failed"
            exit 1
        fi
    fi
done

echo ""
echo "🔧 Setting up environment file..."
if [ ! -f ".env" ]; then
    cp env.example .env
    echo "📝 Created .env file"
fi

# Update .env with correct database URL
sed -i "s|DATABASE_URL=.*|DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}|g" .env

echo ""
echo "✅ Database setup complete!"
echo ""
echo "📋 Database Details:"
echo "  - Host: ${DB_HOST}"
echo "  - Port: ${DB_PORT}"
echo "  - Database: ${DB_NAME}"
echo "  - User: ${DB_USER}"
echo "  - Password: ${DB_PASSWORD}"
echo ""
echo "🔗 Connection String:"
echo "  postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
echo ""
echo "🚀 You can now run: cargo run"
echo ""
echo "🔍 Verify setup:"
echo "  psql -U ${DB_USER} -d ${DB_NAME} -c '\\dt'"

