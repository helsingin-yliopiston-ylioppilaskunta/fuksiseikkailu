use crate::{config::Config, domains::users::models::Role};
use anyhow::{Context, Result};
use sqlx::PgPool;

pub async fn seed_admin_user(pool: &PgPool, config: &Config) -> Result<()> {
    let admin_exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM users 
            WHERE role = $1::user_role AND deleted_at IS NULL
        )
        "#,
        Role::Admin as Role
    )
    .fetch_one(pool)
    .await
    .context("Failed to check for existing admin user")?;

    if admin_exists.unwrap_or(false) {
        tracing::info!("Admin account already exists. Skipping seed.");
        return Ok(());
    }

    tracing::info!(
        "No admin user found. Seeding initial admin: {}",
        config.seed_admin_email
    );

    sqlx::query!(
        r#"
        INSERT INTO users (email, role)
        VALUES (LOWER($1), $2::user_role)
        "#,
        config.seed_admin_email,
        Role::Admin as Role
    )
    .execute(pool)
    .await
    .context("Failed to seed initial admin user")?;

    tracing::info!("Admin account seeded successfully.");

    Ok(())
}
