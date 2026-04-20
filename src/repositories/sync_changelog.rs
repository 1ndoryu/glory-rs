/* [174A-91] Repositorio de sync_changelog: replica obtenerDelta() del legacy.
 * Contrato cursor-based con detección de purge (cursor < MIN(id)) → fullSyncRequired,
 * y caso primera conexión (cursor <= 0) → fullSyncRequired con cursor inicial = MAX(id).
 * Usa query! para validar SQL en compile-time contra el schema actual. */

use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::{SyncChangelogDelta, SyncChangelogEntry, SyncChangelogTipo};

pub struct SyncChangelogRepository;

impl SyncChangelogRepository {
    pub async fn delta(
        pool: &PgPool,
        usuario_id: i32,
        cursor: i64,
        limite: i64,
    ) -> Result<SyncChangelogDelta, AppError> {
        /* Defensa en profundidad: el handler ya acota, pero mantenemos floor/ceiling. */
        let limite = limite.clamp(1, 500);

        if cursor <= 0 {
            let ultimo = Self::ultimo_cursor(pool, usuario_id).await?.unwrap_or(0);
            return Ok(SyncChangelogDelta {
                cambios: Vec::new(),
                cursor: ultimo,
                hay_mas: false,
                full_sync_required: true,
            });
        }

        let fetch_limit = limite + 1;
        let rows = sqlx::query!(
            r"SELECT id, tipo, entidad_id, metadata, created_at
              FROM sync_changelog
              WHERE usuario_id = $1 AND id > $2
              ORDER BY id ASC
              LIMIT $3",
            usuario_id,
            cursor,
            fetch_limit
        )
        .fetch_all(pool)
        .await?;

        if rows.is_empty() {
            let min = sqlx::query_scalar!(
                r"SELECT MIN(id) FROM sync_changelog WHERE usuario_id = $1",
                usuario_id
            )
            .fetch_one(pool)
            .await?;

            if let Some(min_id) = min {
                if cursor < min_id {
                    let ultimo = Self::ultimo_cursor(pool, usuario_id).await?.unwrap_or(0);
                    return Ok(SyncChangelogDelta {
                        cambios: Vec::new(),
                        cursor: ultimo,
                        hay_mas: false,
                        full_sync_required: true,
                    });
                }
            }

            return Ok(SyncChangelogDelta {
                cambios: Vec::new(),
                cursor,
                hay_mas: false,
                full_sync_required: false,
            });
        }

        let hay_mas = i64::try_from(rows.len()).unwrap_or(0) > limite;
        let usable = if hay_mas {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let take = limite as usize;
            &rows[..take]
        } else {
            &rows[..]
        };

        let cambios: Vec<SyncChangelogEntry> = usable
            .iter()
            .map(|row| SyncChangelogEntry {
                id: row.id,
                tipo: SyncChangelogTipo::from_db_str(&row.tipo)
                    .unwrap_or(SyncChangelogTipo::SampleUpdated),
                entidad_id: row.entidad_id,
                metadata: row.metadata.clone(),
                created_at: row.created_at,
            })
            .collect();

        let nuevo_cursor = cambios.last().map_or(cursor, |entry| entry.id);

        Ok(SyncChangelogDelta {
            cambios,
            cursor: nuevo_cursor,
            hay_mas,
            full_sync_required: false,
        })
    }

    async fn ultimo_cursor(pool: &PgPool, usuario_id: i32) -> Result<Option<i64>, AppError> {
        let value = sqlx::query_scalar!(
            r"SELECT MAX(id) FROM sync_changelog WHERE usuario_id = $1",
            usuario_id
        )
        .fetch_one(pool)
        .await?;
        Ok(value)
    }
}
