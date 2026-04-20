#![allow(clippy::needless_for_each)] // Generado por utoipa OpenApi derive
/* sentinel-disable-file limite-lineas — registro central de rutas y schemas OpenAPI */

mod admin;
mod articles;
mod auth;
mod colecciones;
mod comments;
mod connect;
mod dashboard;
mod downloads;
mod fcm;
mod feed;
mod free_codes;
mod health;
mod likes;
mod messages;
mod metrics;
mod music;
mod notifications;
mod payments;
mod plays;
mod posts;
mod push;
mod reports;
mod search;
mod sample_catalog;
mod samples;
mod social;
mod sync;
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
        articles::list_articles,
        articles::list_categories,
        articles::get_article,
        articles::list_my_articles,
        articles::create_article,
        articles::update_article,
        articles::delete_article,
        articles::toggle_like_article,
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
        free_codes::generate_free_code,
        free_codes::verify_free_code,
        free_codes::claim_free_code,
        free_codes::invalidate_free_code,
        connect::create_connect_onboarding,
        connect::get_connect_status,
        connect::create_connect_dashboard_link,
        connect::get_connect_balance,
        dashboard::stats,
        dashboard::top_samples,
        dashboard::transactions,
        dashboard::income_series,
        sync::get_changelog,
        payments::list_plans,
        payments::create_subscription_checkout,
        payments::create_sample_checkout,
        payments::webhook::payment_webhook,
        payments::create_billing_portal,
        reports::report_generic,
        reports::report_user_legacy,
        reports::report_platform_error,
        reports::report_legal,
        reports::list_pending_legal_reports,
        reports::report_comment,
        reports::report_post,
        search::global_search,
        search::legacy_quick_search,
        music::public::list_songs,
        music::public::search_songs,
        music::public::top_songs,
        music::public::get_song,
        music::public::get_song_chain,
        music::public::top_artists,
        music::public::get_artist,
        music::public::get_relation,
        music::public::get_relation_by_sample,
        music::public::relation_stats,
        music::mutations::link_sample_to_relation,
        music::mutations::unlink_sample_from_relation,
        music::mutations::verify_relation,
        music::admin::create_artist,
        music::admin::update_artist,
        music::admin::delete_artist,
        music::admin::create_song,
        music::admin::update_song,
        music::admin::delete_song,
        music::admin::create_relation,
        music::admin::update_relation,
        music::admin::delete_relation,
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
        admin::summary,
        admin::activity,
        admin::list_users,
        admin::update_user_legacy,
        admin::suspend_user_legacy,
        admin::unsuspend_user_legacy,
        admin::mark_delete_legacy,
        admin::cancel_delete_legacy,
        admin::list_scrapers,
        admin::list_extraction_queue,
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
        crate::models::CreateArticleMultipartRequestDoc,
        crate::models::UpdateArticleRequest,
        crate::models::ArticleListData,
        crate::models::ArticleListResponse,
        crate::models::ArticleResponse,
        crate::models::ArticleCategoriesResponse,
        crate::models::DeleteArticleData,
        crate::models::DeleteArticleResponse,
        crate::models::ToggleArticleLikeResponse,
        crate::algorithm::recommender::RankedSample,
        feed::FeedResponse,
        crate::repositories::ArticleAuthorSummary,
        crate::repositories::ArticleCategoryCount,
        crate::repositories::ArticleDetail,
        crate::repositories::ArticleEmbed,
        crate::repositories::ArticleSummary,
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
        crate::models::ClaimFreeCodeRequest,
        crate::models::ClaimFreeCodeResponse,
        crate::models::CreateSubscriptionCheckoutRequest,
        crate::models::CreateSampleCheckoutRequest,
        crate::models::CreatorDashboardIncomePeriod,
        crate::models::CreatorDashboardIncomePoint,
        crate::models::CreatorDashboardSampleStat,
        crate::models::CreatorDashboardStats,
        crate::models::CreatorDashboardTransaction,
        crate::models::CreatorDashboardTransactionType,
        crate::models::SyncChangelogDelta,
        crate::models::SyncChangelogEntry,
        crate::models::SyncChangelogTipo,
        crate::models::DownloadGrantRequest,
        crate::models::CreatorConnectBalance,
        crate::models::CreatorConnectState,
        crate::models::CreatorConnectStatus,
        crate::models::FreeCodeTargetType,
        crate::models::GenerateFreeCodeRequest,
        crate::models::GenerateFreeCodeResponse,
        crate::models::InvalidateFreeCodeResponse,
        crate::domain::KamplesPlanId,
        crate::models::PaymentPlanPeriod,
        crate::models::PaymentPlanPublic,
        crate::models::PaymentPlansResponse,
        crate::models::PaymentRedirectResponse,
        crate::models::PaymentWebhookResponse,
        crate::models::VerifyFreeCodeResponse,
        crate::models::AdminLegalReportItem,
        crate::models::AdminLegalReportsResponse,
        crate::models::CreateGenericReportRequest,
        crate::models::CreateLegalReportRequest,
        crate::models::CreatePlatformErrorReportRequest,
        crate::models::CreateReportReasonRequest,
        crate::models::CreateScopedReportRequest,
        crate::models::ErrorReportResponse,
        crate::models::GenericReportType,
        crate::models::LegalReportDetails,
        crate::models::LegalReportResponse,
        crate::models::LegalReportType,
        crate::models::LegalRightType,
        crate::models::ReportResponse,
        crate::models::GlobalSearchQuery,
        crate::models::GlobalSearchResponse,
        crate::models::LegacyQuickSearchCollectionResult,
        crate::models::LegacyQuickSearchQuery,
        crate::models::LegacyQuickSearchRelationResult,
        crate::models::LegacyQuickSearchRelationSide,
        crate::models::LegacyQuickSearchResponse,
        crate::models::LegacyQuickSearchSampleCreator,
        crate::models::LegacyQuickSearchSampleResult,
        crate::models::LegacyQuickSearchSongResult,
        crate::models::LegacyQuickSearchTodoItem,
        crate::models::LegacyQuickSearchUserResult,
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
        crate::models::AdminSummaryStats,
        crate::models::AdminActivityPoint,
        crate::models::AdminActivityResponse,
        crate::models::AdminUsersQuery,
        crate::models::AdminUsersResponse,
        crate::models::AdminUserListItem,
        crate::models::AdminUserUpdateRequest,
        crate::models::AdminUserSuspendRequest,
        crate::models::AdminUserDeleteRequest,
        crate::models::AdminOkResponse,
        crate::models::AdminActivityQuery,
        crate::models::AdminScrapersQuery,
        crate::models::AdminScrapersResponse,
        crate::models::AdminScraperItem,
        crate::models::AdminExtractionQueueQuery,
        crate::models::AdminExtractionQueueResponse,
        crate::models::AdminExtractionQueueItem,
        crate::models::MusicArtistRole,
        crate::models::SampleRelationType,
        crate::models::SampleRelationElementType,
        crate::models::SampleRelationSource,
        crate::models::RelationSampleSide,
        crate::models::MusicArtist,
        crate::models::MusicSong,
        crate::models::SongArtistLink,
        crate::models::SampleRelationSummary,
        crate::models::SampleRelationDetail,
        crate::models::MusicPagination,
        crate::models::MusicSongsResponse,
        crate::models::SongListResponse,
        crate::models::MusicArtistsResponse,
        crate::models::SongDetailResponse,
        crate::models::ArtistStats,
        crate::models::ArtistDetailResponse,
        crate::models::RelationTypeCount,
        crate::models::RelationStatsResponse,
        crate::models::SampleRelationLookupResponse,
        crate::models::RelationChainNode,
        crate::models::RelationChainResponse,
        crate::models::MusicMutationResponse,
        crate::models::RelationVerificationResponse,
        crate::models::ListSongsQuery,
        crate::models::SearchSongsQuery,
        crate::models::LimitQuery,
        crate::models::RelationChainQuery,
        crate::models::CreateArtistRequest,
        crate::models::UpdateArtistRequest,
        crate::models::SongArtistInput,
        crate::models::CreateSongRequest,
        crate::models::UpdateSongRequest,
        crate::models::CreateRelationRequest,
        crate::models::UpdateRelationRequest,
        crate::models::SampleLinkRequest,
        crate::models::VerifyRelationRequest,
        crate::models::SearchCollectionOwnerSummary,
        crate::models::SearchCollectionResult,
        crate::models::SearchSampleResult,
        crate::models::SearchSongResult,
        crate::models::SearchType,
        crate::models::SearchUserResult,
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

#[derive(Clone)]
pub struct AppRuntimes {
    pub push: Option<crate::services::PushDeliveryRuntime>,
    pub fcm: Option<crate::services::FcmDeliveryRuntime>,
    pub email: Option<crate::services::EmailDeliveryRuntime>,
    pub stripe: Option<crate::services::StripeRuntime>,
}

/// Crea el router principal con CORS, tracing, Swagger UI y todas las rutas.
pub fn create_router(
    pool: sqlx::PgPool,
    redis: Option<deadpool_redis::Pool>,
    config: crate::config::AppConfig,
    storage: std::sync::Arc<dyn crate::services::FileStorage>,
    runtimes: AppRuntimes,
    algo_planner: std::sync::Arc<crate::algorithm::AlgoPlanner>,
) -> Router {
    let public_base_url = config.public_base_url.clone();
    let ws_public_url = config.ws_public_url.clone();
    let redis_url = config.redis_url.clone();
    let storage_root = config.storage_root.clone();
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
        push_runtime: runtimes.push.map(std::sync::Arc::new),
        fcm_runtime: runtimes.fcm.map(std::sync::Arc::new),
        email_runtime: runtimes.email.map(std::sync::Arc::new),
        stripe_runtime: runtimes.stripe.map(std::sync::Arc::new),
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
        .merge(articles::routes())
        .merge(feed::routes())
        .merge(sample_catalog::routes())
        .merge(samples::routes())
        .merge(plays::routes())
        .merge(likes::routes())
        .merge(messages::routes())
        .merge(notifications::routes())
        .merge(free_codes::routes())
        .merge(connect::routes())
        .merge(dashboard::routes())
        .merge(sync::routes())
        .merge(payments::routes())
        .merge(reports::routes())
        .merge(search::routes())
        .merge(music::routes())
        .merge(fcm::routes())
        .merge(push::routes())
        .merge(comments::routes())
        .merge(posts::routes())
        .merge(social::routes())
        .merge(downloads::routes())
        .merge(ws::routes())
        .merge(colecciones::routes())
        .merge(users::routes())
        .merge(metrics::routes())
        .merge(admin::routes())
}
