use chrono::{DateTime, Utc};
use eyre::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    pub async fn create_user(&self, login: &str) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let id_str = id.to_string();
        let naive_now = now.naive_utc();

        sqlx::query!(
            "INSERT INTO users (id, login, created_at, last_active_at) VALUES (?1, ?2, ?3, ?4)",
            id_str,
            login,
            naive_now,
            naive_now
        )
        .execute(&self.pool)
        .await?;

        Ok(User {
            id,
            login: login.to_string(),
            created_at: now,
            last_active_at: now,
        })
    }

    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>> {
        let user_id_str = user_id.to_string();
        let row = sqlx::query!(
            "SELECT id, login, created_at, last_active_at FROM users WHERE id = ?1",
            user_id_str
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: Uuid::parse_str(&row.id)?,
                login: row.login,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                last_active_at: DateTime::from_naive_utc_and_offset(row.last_active_at, Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_by_login(&self, login: &str) -> Result<Option<User>> {
        let row = sqlx::query!(
            "SELECT id, login, created_at, last_active_at FROM users WHERE login = ?1",
            login
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            Ok(Some(User {
                id: Uuid::parse_str(&row.id)?,
                login: row.login,
                created_at: DateTime::from_naive_utc_and_offset(row.created_at, Utc),
                last_active_at: DateTime::from_naive_utc_and_offset(row.last_active_at, Utc),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn update_user_last_active(&self, user_id: &Uuid) -> Result<()> {
        let now = Utc::now();
        let user_id_str = user_id.to_string();
        let naive_now = now.naive_utc();

        sqlx::query!(
            "UPDATE users SET last_active_at = ?1 WHERE id = ?2",
            naive_now,
            user_id_str
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_or_create_user(&self, login: &str) -> Result<User> {
        if let Some(user) = self.get_user_by_login(login).await? {
            self.update_user_last_active(&user.id).await?;
            Ok(User {
                last_active_at: Utc::now(),
                ..user
            })
        } else {
            self.create_user(login).await
        }
    }
}
