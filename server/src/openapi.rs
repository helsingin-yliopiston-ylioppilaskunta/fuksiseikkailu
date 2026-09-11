use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::domains::{
    areas, auth, checkpoints, news, photos, ratings, reports, scores, teams, users,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // Auth
        auth::routes::register,
        auth::routes::request_otp,
        auth::routes::verify_otp,

        // Users
        users::routes::list_users,
        users::routes::get_me,
        users::routes::get_user,
        users::routes::create_user,
        users::routes::update_me,
        users::routes::update_user,
        users::routes::delete_me,
        users::routes::delete_user,

        // Areas
        areas::routes::list_areas,
        areas::routes::get_area,
        areas::routes::create_area,
        areas::routes::update_area,
        areas::routes::delete_area,

        // Checkpoints
        checkpoints::routes::list_checkpoints,
        checkpoints::routes::get_checkpoint,
        checkpoints::routes::create_checkpoint,
        checkpoints::routes::batch_import,
        checkpoints::routes::update_checkpoint,
        checkpoints::routes::delete_checkpoint,

        // Teams
        teams::routes::list_teams,
        teams::routes::get_team,
        teams::routes::create_team,
        teams::routes::batch_import,
        teams::routes::update_team,
        teams::routes::delete_team,

        // Scores
        scores::routes::submit_score,
        scores::routes::get_leaderboard,
        scores::routes::list_by_checkpoint,
        scores::routes::list_by_team,
        scores::routes::update_score,

        // Photos
        photos::routes::list_photos,
        photos::routes::create_photo,
        photos::routes::update_photo,
        photos::routes::delete_photo,
        photos::routes::vote_photo,
        photos::routes::suggest_team,
        photos::routes::list_suggestions,

        // Ratings
        ratings::routes::rate_checkpoint,
        ratings::routes::rate_team,
        ratings::routes::get_checkpoint_ratings,
        ratings::routes::get_team_ratings,

        // Reports
        reports::routes::create_checkpoint_report,
        reports::routes::list_checkpoint_reports,
        reports::routes::create_team_report,
        reports::routes::list_team_reports,

        // News
        news::routes::list_news,
        news::routes::get_news_article,
        news::routes::create_news_article,
        news::routes::update_news_article,
        news::routes::delete_news_article,
    ),
    components(
        schemas(
            auth::models::AuthTokens,
            auth::models::RegisterPayload,
            auth::models::RequestOtpPayload,
            auth::models::VerifyOtpPayload,

            users::models::User,
            users::models::CreateUser,
            users::models::UpdateUser,
            users::models::Role,

            areas::models::Area,
            areas::models::CreateArea,
            areas::models::UpdateArea,

            checkpoints::models::PublicCheckpoint,
            checkpoints::models::Checkpoint,
            checkpoints::models::CreateCheckpoint,
            checkpoints::models::UpdateCheckpoint,
            checkpoints::models::BatchImportPayload,
            checkpoints::models::BatchImportResponse,
            checkpoints::models::CheckpointCategory,

            teams::models::Team,
            teams::models::CreateTeam,
            teams::models::UpdateTeam,
            teams::models::BatchImportTeamsPayload,
            teams::models::BatchImportTeamsResponse,

            scores::models::Score,
            scores::models::SubmitScorePayload,
            scores::models::UpdateScorePayload,
            scores::models::TeamLeaderboardEntry,

            photos::models::Photo,
            photos::models::CreatePhotoPayload,
            photos::models::UpdatePhotoPayload,
            photos::models::SubmitSuggestionPayload,
            photos::models::PhotoTeamSuggestion,

            ratings::models::CheckpointRating,
            ratings::models::TeamRating,
            ratings::models::CreateCheckpointRatingPayload,
            ratings::models::CreateTeamRatingPayload,

            reports::models::CheckpointReport,
            reports::models::TeamReport,
            reports::models::CreateCheckpointReportPayload,
            reports::models::CreateTeamReportPayload,

            news::models::NewsArticle,
            news::models::CreateNewsPayload,
            news::models::UpdateNewsPayload,
        )
    ),
    tags(
        (name = "Auth", description = "Passwordless OTP authentication & session endpoints"),
        (name = "Users", description = "User account administration endpoints"),
        (name = "Areas", description = "Geographic event sector management endpoints"),
        (name = "Checkpoints", description = "Checkpoint configuration, location, and metadata endpoints"),
        (name = "Teams", description = "Participant team administration endpoints"),
        (name = "Scores", description = "Real-time scoring and leaderboard calculation endpoints"),
        (name = "Photos", description = "Photo media uploads, public voting, and team tag suggestions"),
        (name = "Ratings", description = "Participant checkpoint feedback and organizer team spirit ratings"),
        (name = "Reports", description = "Checkpoint operational feedback and team incident reporting"),
        (name = "News", description = "Event announcements and broadcast news endpoints"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
