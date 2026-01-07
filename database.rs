use sqlx::{SqlitePool, Row};
use crate::models::user::User;
use crate::errors::AppError;

#[derive(Clone)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, AppError> {
        let pool = SqlitePool::connect(database_url)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to connect to SQLite: {}", e)))?;
        
        log::info!("Successfully connected to SQLite database");
        
        Ok(Database { pool })
    }
    
    pub async fn migrate(&self) -> Result<(), AppError> {
        // Run SQLite migration directly
        let migration_sql = include_str!("../migrations/001_create_users_table_sqlite.sql");
        
        sqlx::query(migration_sql)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to run migrations: {}", e)))?;
        
        log::info!("Database migrations completed successfully");
        Ok(())
    }
    
    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            "SELECT id, name, email, password_hash, avatar, created_at, updated_at, is_active 
             FROM users WHERE email = ?1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user: {}", e)))?;
        
        match row {
            Some(row) => {
                let id_str: String = row.get("id");
                let is_active_int: i64 = row.get("is_active");
                let created_at_str: String = row.get("created_at");
                let updated_at_str: String = row.get("updated_at");
                
                Ok(Some(User {
                    id: Some(uuid::Uuid::parse_str(&id_str).map_err(|_| AppError::DatabaseError("Invalid UUID".to_string()))?),
                    name: row.get("name"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    avatar: row.get("avatar"),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|_| AppError::DatabaseError("Invalid datetime".to_string()))?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|_| AppError::DatabaseError("Invalid datetime".to_string()))?
                        .with_timezone(&chrono::Utc),
                    is_active: is_active_int != 0,
                }))
            }
            None => Ok(None),
        }
    }
    
    pub async fn find_user_by_id(&self, id: &uuid::Uuid) -> Result<Option<User>, AppError> {
        let row = sqlx::query(
            "SELECT id, name, email, password_hash, avatar, created_at, updated_at, is_active 
             FROM users WHERE id = ?1"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("Failed to find user: {}", e)))?;
        
        match row {
            Some(row) => {
                let id_str: String = row.get("id");
                let is_active_int: i64 = row.get("is_active");
                let created_at_str: String = row.get("created_at");
                let updated_at_str: String = row.get("updated_at");
                
                Ok(Some(User {
                    id: Some(uuid::Uuid::parse_str(&id_str).map_err(|_| AppError::DatabaseError("Invalid UUID".to_string()))?),
                    name: row.get("name"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    avatar: row.get("avatar"),
                    created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|_| AppError::DatabaseError("Invalid datetime".to_string()))?
                        .with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|_| AppError::DatabaseError("Invalid datetime".to_string()))?
                        .with_timezone(&chrono::Utc),
                    is_active: is_active_int != 0,
                }))
            }
            None => Ok(None),
        }
    }
    
    pub async fn create_user(&self, user: &User) -> Result<User, AppError> {
        let user_id = uuid::Uuid::new_v4();
        let created_at_str = user.created_at.to_rfc3339();
        let updated_at_str = user.updated_at.to_rfc3339();
        let is_active_int = if user.is_active { 1 } else { 0 };
        
        sqlx::query(
            "INSERT INTO users (id, name, email, password_hash, avatar, created_at, updated_at, is_active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
        )
        .bind(user_id.to_string())
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.avatar)
        .bind(&created_at_str)
        .bind(&updated_at_str)
        .bind(is_active_int)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("UNIQUE constraint failed") {
                AppError::ValidationError("A user with this email already exists".to_string())
            } else {
                AppError::DatabaseError(format!("Failed to create user: {}", e))
            }
        })?;
        
        Ok(User {
            id: Some(user_id),
            name: user.name.clone(),
            email: user.email.clone(),
            password_hash: user.password_hash.clone(),
            avatar: user.avatar.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            is_active: user.is_active,
        })
    }
}
