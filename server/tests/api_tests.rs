mod common;

use common::TestApp;
use reqwest::StatusCode;
use serde_json::json;
use server::domains::users::models::Role;
use sqlx::PgPool;

#[sqlx::test]
async fn test_auth_and_user_flow(pool: PgPool) {
    let app = TestApp::spawn(pool).await;

    // 1. Create a real admin user in the DB & get their token
    let (_admin_id, admin_token) = app.create_user_and_token(Role::Admin, None, None).await;

    // 2. Request OTP
    let resp = app
        .client
        .post(format!("{}/auth/otp/request", app.address))
        .json(&json!({ "email": "test-admin@localhost" }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 3. Register new user
    let register_resp = app
        .client
        .post(format!("{}/auth/register", app.address))
        .json(&json!({
            "email": "newuser@example.com",
            "role": "team"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(register_resp.status(), StatusCode::CREATED);

    // 4. GET /users/me with valid Admin Auth (user exists in DB)
    let me_resp = app
        .client
        .get(format!("{}/users/me", app.address))
        .bearer_auth(&admin_token)
        .send()
        .await
        .unwrap();
    assert_eq!(me_resp.status(), StatusCode::OK);
}

#[sqlx::test]
async fn test_areas_crud_flow(pool: PgPool) {
    let app = TestApp::spawn(pool).await;
    let (_admin_id, admin_token) = app.create_user_and_token(Role::Admin, None, None).await;

    let create_resp = app
        .client
        .post(format!("{}/areas", app.address))
        .bearer_auth(&admin_token)
        .json(&json!({ "name": "Kumpula Sector" }))
        .send()
        .await
        .unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let area: serde_json::Value = create_resp.json().await.unwrap();
    let area_id = area["id"].as_str().unwrap();

    let list_resp = app
        .client
        .get(format!("{}/areas", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.status(), StatusCode::OK);

    let del_resp = app
        .client
        .delete(format!("{}/areas/{}", app.address, area_id))
        .bearer_auth(&admin_token)
        .send()
        .await
        .unwrap();
    assert_eq!(del_resp.status(), StatusCode::NO_CONTENT);
}

#[sqlx::test]
async fn test_checkpoints_and_batch_import(pool: PgPool) {
    let app = TestApp::spawn(pool).await;
    let (_admin_id, admin_token) = app.create_user_and_token(Role::Admin, None, None).await;

    let batch_resp = app
        .client
        .post(format!("{}/checkpoints/batch", app.address))
        .bearer_auth(&admin_token)
        .json(&json!({
            "checkpoints": [
                { "name": "CP 1", "number": 1, "category": "subject" },
                { "name": "CP 2", "number": 2, "category": "hobby" }
            ]
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(batch_resp.status(), StatusCode::CREATED);

    let list_resp = app
        .client
        .get(format!("{}/checkpoints", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(list_resp.status(), StatusCode::OK);
}

#[sqlx::test]
async fn test_teams_scores_and_leaderboard(pool: PgPool) {
    let app = TestApp::spawn(pool).await;
    let (_admin_id, admin_token) = app.create_user_and_token(Role::Admin, None, None).await;

    // 1. Create Team
    let team_resp = app
        .client
        .post(format!("{}/teams", app.address))
        .bearer_auth(&admin_token)
        .json(&json!({ "name": "Fuksi Squad", "number": 42, "participants": 5 }))
        .send()
        .await
        .unwrap();
    assert_eq!(team_resp.status(), StatusCode::CREATED);
    let team: serde_json::Value = team_resp.json().await.unwrap();
    let team_id = team["id"].as_str().unwrap();

    // 2. Create Checkpoint
    let cp_resp = app
        .client
        .post(format!("{}/checkpoints", app.address))
        .bearer_auth(&admin_token)
        .json(&json!({ "name": "Räp-Räp Checkpoint" }))
        .send()
        .await
        .unwrap();
    let cp: serde_json::Value = cp_resp.json().await.unwrap();
    let cp_id = cp["id"].as_str().unwrap();

    // 3. Submit Score as Admin (user exists in DB satisfying foreign key)
    let score_resp = app
        .client
        .post(format!("{}/scores", app.address))
        .bearer_auth(&admin_token)
        .json(&json!({
            "team_id": team_id,
            "checkpoint_id": cp_id,
            "score": 10,
            "participants_present": 5
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(score_resp.status(), StatusCode::OK);

    // 4. GET Leaderboard
    let leaderboard_resp = app
        .client
        .get(format!("{}/scores/leaderboard", app.address))
        .send()
        .await
        .unwrap();
    assert_eq!(leaderboard_resp.status(), StatusCode::OK);
}

#[sqlx::test]
async fn test_photos_voting_and_news(pool: PgPool) {
    let app = TestApp::spawn(pool).await;
    let (_admin_id, admin_token) = app.create_user_and_token(Role::Admin, None, None).await;

    let news_resp = app
        .client
        .post(format!("{}/news", app.address))
        .bearer_auth(&admin_token)
        .json(&json!({
            "title": "Welcome to Fuksiseikkailu!",
            "content": { "blocks": ["Event starts now!"] }
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(news_resp.status(), StatusCode::CREATED);

    let photo_resp = app
        .client
        .post(format!("{}/photos", app.address))
        .bearer_auth(&admin_token)
        .json(&json!({ "s3_key": "uploads/event1.jpg" }))
        .send()
        .await
        .unwrap();
    assert_eq!(photo_resp.status(), StatusCode::CREATED);
    let photo: serde_json::Value = photo_resp.json().await.unwrap();
    let photo_id = photo["id"].as_str().unwrap();

    let vote_resp = app
        .client
        .post(format!("{}/photos/{}/vote", app.address, photo_id))
        .send()
        .await
        .unwrap();
    assert_eq!(vote_resp.status(), StatusCode::OK);
}
