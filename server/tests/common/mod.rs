use reqwest::Client;
use server::{
    app,
    config::Config,
    domains::{auth::jwt, users::models::Role},
};
use sqlx::PgPool;
use std::net::SocketAddr;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub client: Client,
    pub pool: PgPool,
    pub config: Config,
}

impl TestApp {
    pub async fn spawn(pool: PgPool) -> Self {
        let config = Config {
            database_url: "postgres://postgres:postgres@localhost/fuksiseikkailu_test".to_string(),
            bind_addr: "127.0.0.1:0".parse().unwrap(),
            max_db_connections: 5,
            jwt_secret: "integration-test-secret-key-123456789".to_string(),
            jwt_expiration_seconds: 3600,
            seed_admin_email: "test-admin@localhost".to_string(),
            seed_admin_password: "unused".to_string(),
        };

        let router = app(pool.clone(), config.clone());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            axum::serve(
                listener,
                router.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
        });

        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .unwrap();

        Self {
            address: format!("http://127.0.0.1:{port}"),
            client,
            pool,
            config,
        }
    }

    /// Generates a valid Bearer JWT for specified user role and optional IDs
    pub fn create_token(
        &self,
        role: Role,
        checkpoint_id: Option<Uuid>,
        team_id: Option<Uuid>,
    ) -> String {
        jwt::encode_jwt(
            Uuid::new_v4(),
            role,
            checkpoint_id,
            team_id,
            &self.config.jwt_secret,
            self.config.jwt_expiration_seconds,
        )
        .unwrap()
    }

    /// Inserts a real user into the database and returns a valid Bearer JWT bound to their ID
    pub async fn create_user_and_token(
        &self,
        role: Role,
        checkpoint_id: Option<Uuid>,
        team_id: Option<Uuid>,
    ) -> (Uuid, String) {
        let email = format!("test-{}@example.com", Uuid::new_v4());
        let user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (email, role, checkpoint_id, team_id)
            VALUES ($1, $2::user_role, $3, $4)
            RETURNING id
            "#,
            email,
            role as Role,
            checkpoint_id,
            team_id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap();

        let token = jwt::encode_jwt(
            user_id,
            role,
            checkpoint_id,
            team_id,
            &self.config.jwt_secret,
            self.config.jwt_expiration_seconds,
        )
        .unwrap();

        (user_id, token)
    }
}
