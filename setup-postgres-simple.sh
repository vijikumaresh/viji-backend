#!/bin/bash

echo "🐘 PostgreSQL Setup (No Docker Required)"
echo "========================================"

# Install PostgreSQL if not already installed
if ! command -v psql &> /dev/null; then
    echo "📦 Installing PostgreSQL..."
    sudo apt update
    sudo apt install -y postgresql postgresql-contrib
    
    echo "🔧 Starting PostgreSQL service..."
    sudo systemctl start postgresql
    sudo systemctl enable postgresql
else
    echo "✅ PostgreSQL is already installed"
fi

# Ensure PostgreSQL is running
if ! sudo systemctl is-active --quiet postgresql; then
    echo "🔧 Starting PostgreSQL service..."
    sudo systemctl start postgresql
fi

echo "👤 Setting up database and user..."

# Create database and user
sudo -u postgres psql << 'EOF'
-- Create database if it doesn't exist
SELECT 'CREATE DATABASE loginapp'
WHERE NOT EXISTS (SELECT FROM pg_database WHERE datname = 'loginapp')\gexec

-- Set password for postgres user
ALTER USER postgres PASSWORD 'password';

-- Grant all privileges
GRANT ALL PRIVILEGES ON DATABASE loginapp TO postgres;

-- Show connection info
\l loginapp
\q
EOF

echo "🔧 Setting up environment..."
if [ ! -f ".env" ]; then
    cp env.example .env
    echo "📝 Created .env file from template"
fi

echo ""
echo "✅ PostgreSQL setup complete!"
echo ""
echo "📋 Database Details:"
echo "  - Host: localhost"
echo "  - Port: 5432"
echo "  - Database: loginapp"
echo "  - User: postgres"
echo "  - Password: password"
echo ""
echo "🔗 Connection String:"
echo "  postgresql://postgres:password@localhost/loginapp"
echo ""
echo "🚀 You can now run: cargo run"
echo ""
echo "🔍 Useful commands:"
echo "  - Connect to database: psql -U postgres -d loginapp"
echo "  - Check PostgreSQL status: sudo systemctl status postgresql"
echo "  - View logs: sudo journalctl -u postgresql"
