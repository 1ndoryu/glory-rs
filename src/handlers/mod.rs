#![allow(clippy::needless_for_each)] // Generado por utoipa OpenApi derive

mod admin;
mod auth;
mod colecciones;
mod comments;
mod downloads;
mod fcm;
mod feed;
mod health;
mod likes;
mod messages;
mod notifications;
mod posts;
mod plays;
mod social;
mod sample_catalog;
mod samples;
mod push;
mod users;
mod ws;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;

/// Define el esquema de seguridad Bearer para Swagger UI
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        /* components existe porque el derive ya registra schemas */
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        auth::register,
        auth::login,
        auth::refresh,
        auth::logout,
        auth::google_login,
        auth::google_pkce,
        feed::get_feed,
        feed::get_me_feed,
        sample_catalog::delete_sample,
        sample_catalog::get_sample,
        sample_catalog::list_samples,
        sample_catalog::random_sample,
        sample_catalog::similar_samples,
        sample_catalog::update_sample,
        samples::check_duplicate,
        samples::upload,
        plays::register_play,
        likes::create_like,
        likes::delete_like,
        messages::list_conversations,
        messages::list_messages,
        messages::send_message,
        messages::mark_read,
        messages::mark_all_read,
        messages::start_conversation,
        notifications::list_notifications,
        notifications::mark_notification_read,
        notifications::mark_all_notifications_read,
        notifications::unread_notifications_count,
        fcm::register_fcm_token,
        fcm::delete_fcm_token,
        push::get_vapid_key,
        push::subscribe_push,
        push::unsubscribe_push,
        comments::list_comments,
        comments::list_replies,
        comments::create_comment,
        comments::update_comment,
        comments::delete_comment,
        comments::like_comment,
        comments::unlike_comment,
        posts::create_post,
        posts::list_posts,
        posts::get_post,
        posts::update_post,
        posts::delete_post,
        posts::repost_post,
        posts::unrepost_post,
        social::follow_user,
        social::unfollow_user,
        social::block_user,
        social::unblock_user,
        social::my_blocks,
        downloads::register_download,
        downloads::download_limits,
        downloads::stream_download,
        ws::issue_ticket,
        ws::upgrade_connection,
        colecciones::create_coleccion,
        colecciones::list_my_colecciones,
        colecciones::get_coleccion,
        colecciones::update_coleccion,
        colecciones::delete_coleccion,
        colecciones::add_sample,
        colecciones::remove_sample,
        colecciones::list_samples,
        colecciones::merge_coleccion,
        colecciones::save_coleccion,
        colecciones::unsave_coleccion,
        colecciones::list_saved_colecciones,
        colecciones::zip::descargar_zip_coleccion,
        users::me,
        users::update_me,
        users::public_profile,
        users::block,
        users::unblock,
        users::list_blocked,
        admin::suspend,
        admin::activate,
        admin::mark_delete,
        admin::algo_timing_history,
    ),
    components(schemas(
        health::HealthResponse,
        crate::models::RegisterRequest,
        crate::models::LoginRequest,
        crate::models::RefreshRequest,
        crate::models::LogoutRequest,
        crate::models::GoogleAuthRequest,
        crate::models::GooglePkceRequest,
        crate::models::AuthResponse,
        crate::models::UserResponse,
        crate::algorithm::recommender::RankedSample,
        feed::FeedResponse,
        crate::models::SampleCreatorSummary,
        crate::models::CheckDuplicateRequest,
        crate::models::CheckDuplicateResponse,
        plays::RegisterPlayRequest,
        plays::RegisterPlayResponse,
        plays::PlayTriggered,
        likes::LikeRequest,
        likes::LikeResponse,
        messages::CreateMessageJsonRequest,
        messages::CreateMessageMultipartRequestDoc,
        messages::StartConversationRequest,
        messages::ConversationListResponse,
        messages::MessageListResponse,
        messages::ConversationMutationResponse,
        messages::MessageMutationResponse,
        notifications::NotificationListResponse,
        notifications::NotificationCountResponse,
        fcm::RegisterFcmTokenRequest,
        fcm::DeleteFcmTokenRequest,
        push::PushSubscriptionKeysRequest,
        push::SubscribePushRequest,
        push::UnsubscribePushRequest,
        push::PushVapidKeyResponse,
        crate::services::FcmTokenPlatform,
        crate::services::PushSubscriptionPlatform,
        crate::repositories::ConversationParticipantSummary,
        crate::repositories::ConversationSummary,
        crate::repositories::ConversationMessage,
        crate::repositories::DirectMessageKind,
        crate::repositories::NotificationActor,
        crate::repositories::UserNotification,
        comments::CreateCommentJsonRequest,
        comments::CreateCommentMultipartRequestDoc,
        comments::UpdateCommentRequest,
        comments::CommentLikeRequest,
        comments::CommentListResponse,
        comments::CommentRepliesResponse,
        comments::CommentMutationResponse,
        crate::repositories::CommentAuthorSummary,
        crate::repositories::CommentDetail,
        posts::CreatePostRequest,
        posts::UpdatePostRequest,
        posts::PostListResponse,
        posts::PostMutationResponse,
        posts::RepostResponse,
        posts::OkResponse,
        crate::repositories::PostAuthorSummary,
        crate::repositories::RepostedPostSummary,
        crate::repositories::PostDetail,
        social::OkResponse,
        social::BlockRequest,
        social::BlockedListResponse,
        crate::repositories::BlockedUser,
        downloads::DownloadResponse,
        downloads::DownloadLimitsResponse,
        ws::WebSocketTicketResponse,
        glory_rs::websocket::WebSocketEnvelope,
        colecciones::CreateColeccionRequest,
        colecciones::UpdateColeccionRequest,
        colecciones::AddSampleRequest,
        colecciones::OkResponse,
        colecciones::ColeccionListResponse,
        colecciones::ColeccionSamplesResponse,
        colecciones::MergeColeccionRequest,
        colecciones::MergeColeccionResponse,
        colecciones::SavedListResponse,
        crate::repositories::SavedColeccion,
        crate::repositories::Coleccion,
        crate::repositories::ColeccionSample,
        crate::models::DeleteSampleResponse,
        crate::models::ListSamplesQuery,
        crate::models::ListSamplesResponse,
        crate::models::SampleDetailResponse,
        crate::models::SampleSummary,
        crate::models::SamplesPagination,
        crate::models::SimilarSamplesQuery,
        crate::models::SimilarSamplesResponse,
        crate::models::UpdateSampleRequest,
        crate::models::UploadSampleRequestDoc,
        crate::models::UploadSampleResponse,
        crate::models::UpdateProfileRequest,
        crate::models::PublicProfileResponse,
        crate::models::PrivateProfileResponse,
        crate::models::BlockUserRequest,
        crate::models::SuspendUserRequest,
        crate::models::DeleteUserRequest,
        crate::services::algo_timing::TimingEntry,
        crate::services::algo_timing::TimingStage,
        crate::errors::ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    info(
        title = "Glory Kamples API",
        version = "0.1.0",
        description = "API Kamples — Rust + Axum + OpenAPI"
    )
)]
#[allow(clippy::needless_for_each)]
pub struct ApiDoc;

/// Crea el router principal con CORS, tracing, Swagger UI y todas las rutas.
pub fn create_router(
    pool: sqlx::PgPool,
    redis: Option<deadpool_redis::Pool>,
    config: crate::config::AppConfig,
    storage: std::sync::Arc<dyn crate::services::FileStorage>,
    push_runtime: Option<crate::services::PushDeliveryRuntime>,
    fcm_runtime: Option<crate::services::FcmDeliveryRuntime>,
    email_runtime: Option<crate::services::EmailDeliveryRuntime>,
) -> Router {
    let public_base_url = config.public_base_url.clone();
    let ws_public_url = config.ws_public_url.clone();
    let redis_url = config.redis_url.clone();
    let storage_root = config.storage_root.clone();
    let algo_planner =
        crate::algorithm::AlgoPlanner::new(crate::algorithm::AlgoPlannerConfig::legacy_defaults());
    let ws_hub = std::sync::Arc::new(glory_rs::websocket::WebSocketHub::new(
        glory_rs::websocket::HubConfig::default(),
    ));
    let ws_node_id = uuid::Uuid::new_v4();
    if redis.is_some() {
        if let Some(redis_url) = redis_url {
            crate::ws::spawn_pubsub_bridge(&redis_url, std::sync::Arc::clone(&ws_hub), ws_node_id);
        }
    }
    let state = AppState {
        pool,
        redis,
        jwt_secret: config.jwt_secret,
        ws_secret: config.ws_secret,
        google: std::sync::Arc::new(crate::services::GoogleVerifier::new(
            config.google_client_ids,
        )),
        storage,
        public_base_url,
        push_runtime: push_runtime.map(std::sync::Arc::new),
        fcm_runtime: fcm_runtime.map(std::sync::Arc::new),
        email_runtime: email_runtime.map(std::sync::Arc::new),
        ws_public_url,
        ws_ticket_ttl_secs: config.ws_ticket_ttl_secs,
        ws_hub,
        ws_node_id,
        algo_planner,
    };

    /* CORS: en desarrollo se permite todo. En producción, restringir orígenes */
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        /* [174A-6] Alias /docs → /swagger-ui/ para acceso más natural. */
        .route(
            "/docs",
            axum::routing::get(|| async { axum::response::Redirect::permanent("/swagger-ui/") }),
        )
        .nest_service("/uploads", ServeDir::new(storage_root))
        .nest("/api", api_routes())
        .layer(axum::middleware::from_fn(
            crate::middleware::request_id_middleware,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(health::routes())
        .merge(auth::routes())
        .merge(feed::routes())
        .merge(sample_catalog::routes())
        .merge(samples::routes())
        .merge(plays::routes())
        .merge(likes::routes())
        .merge(messages::routes())
        .merge(notifications::routes())
        .merge(fcm::routes())
        .merge(push::routes())
        .merge(comments::routes())
        .merge(posts::routes())
        .merge(social::routes())
        .merge(downloads::routes())
        .merge(ws::routes())
        .merge(colecciones::routes())
        .merge(users::routes())
        .merge(admin::routes())
}
