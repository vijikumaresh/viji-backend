#!/bin/bash

echo "🐘 Setting up PostgreSQL Database (Simple)"
echo "=========================================="

# Database configuration
DB_NAME="loginapp"
DB_USER="pc3"  # Use current user
DB_HOST="localhost"
DB_PORT="5432"

echo "📋 Creating database and running migrations..."

# Create database (will prompt for password if needed)
createdb ${DB_NAME} 2>/dev/null || echo "Database may already exist"

# Run migrations
echo "  Running migration: 001_create_users_table.sql"
psql -d ${DB_NAME} -f migrations/001_create_users_table.sql

if [ $? -eq 0 ]; then
    echo "  ✅ Migration completed successfully"
else
    echo "  ❌ Migration failed"
    exit 1
fi

echo ""
echo "🔧 Setting up environment file..."
if [ ! -f ".env" ]; then
    cp env.example .env
    echo "📝 Created .env file"
fi

# Update .env with correct database URL (no password for current user)
sed -i "s|DATABASE_URL=.*|DATABASE_URL=postgresql://${DB_USER}@${DB_HOST}:${DB_PORT}/${DB_NAME}|g" .env

echo ""
echo "✅ Database setup complete!"
echo ""
echo "📋 Database Details:"
echo "  - Host: ${DB_HOST}"
echo "  - Port: ${DB_PORT}"
echo "  - Database: ${DB_NAME}"
echo "  - User: ${DB_USER}"
echo ""
echo "🔗 Connection String:"
echo "  postgresql://${DB_USER}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
echo ""
echo "🚀 You can now run: cargo run"
echo ""
echo "🔍 Verify setup:"
echo "  psql -d ${DB_NAME} -c '\\dt'"

