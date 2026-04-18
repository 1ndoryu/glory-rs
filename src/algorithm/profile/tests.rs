/* [174A-50] Tests del PerfilUsuario.
 *
 * Las queries SQL se validan en compile-time vía `sqlx::query!` contra la BD
 * local (mismo patrón que el resto del repo). Aquí sólo cubrimos la lógica
 * pura: cold-start, helpers de cache key y memoización en memoria. */

use super::{cache_key, ProfileService, UserProfile, CACHE_PREFIX};

#[test]
fn cold_start_keeps_only_user_id_and_genres() {
    let profile = UserProfile::cold_start(42, vec!["trap".into(), "house".into()]);
    assert_eq!(profile.user_id, 42);
    assert_eq!(profile.interactions, 0);
    assert!(profile.is_cold_start());
    assert!(profile.bpm_avg.is_none());
    assert!(profile.key_fav.is_none());
    assert!(profile.scale_fav.is_none());
    assert!(profile.type_fav.is_none());
    assert!(profile.favorite_creators.is_empty());
    assert_eq!(profile.declared_genres, vec!["trap", "house"]);
}

#[test]
fn cache_key_uses_legacy_prefix() {
    assert_eq!(cache_key(7), format!("{CACHE_PREFIX}7"));
    assert!(cache_key(123).starts_with("kamples_perfil_usr_"));
}

#[test]
fn user_profile_round_trips_through_json() {
    let original = UserProfile {
        user_id: 9,
        interactions: 25,
        bpm_avg: Some(128),
        key_fav: Some("F#".into()),
        scale_fav: Some("minor".into()),
        type_fav: Some("loop".into()),
        favorite_creators: vec![1, 2, 3],
        declared_genres: vec!["techno".into()],
    };
    let json = serde_json::to_string(&original).expect("serialize");
    let decoded: UserProfile = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(decoded, original);
}

#[tokio::test]
async fn invalidate_without_redis_is_noop_safe() {
    /* Sin Redis configurado, invalidate sólo borra de la cache en memoria
     * y nunca debe propagar errores aun si la entrada no existe. */
    ProfileService::invalidate(&None, 999_999)
        .await
        .expect("invalidate sin redis no debe fallar");
}
