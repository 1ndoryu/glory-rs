/* [174A-91] Modelos para sync changelog (delta sync de desktop/mobile).
 * Replica el contrato legacy SyncController::delta con cursor-based pagination y
 * fullSyncRequired cuando el cursor del cliente fue purgado o es la primera vez. */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SyncChangelogTipo {
    SampleAdded,
    SampleRemoved,
    SampleUpdated,
    CollectionCreated,
    CollectionRenamed,
    CollectionDeleted,
    CollectionMerged,
}

impl SyncChangelogTipo {
    #[must_use]
    pub fn as_db_str(self) -> &'static str {
        match self {
            Self::SampleAdded => "sample_added",
            Self::SampleRemoved => "sample_removed",
            Self::SampleUpdated => "sample_updated",
            Self::CollectionCreated => "collection_created",
            Self::CollectionRenamed => "collection_renamed",
            Self::CollectionDeleted => "collection_deleted",
            Self::CollectionMerged => "collection_merged",
        }
    }

    #[must_use]
    pub fn from_db_str(value: &str) -> Option<Self> {
        match value {
            "sample_added" => Some(Self::SampleAdded),
            "sample_removed" => Some(Self::SampleRemoved),
            "sample_updated" => Some(Self::SampleUpdated),
            "collection_created" => Some(Self::CollectionCreated),
            "collection_renamed" => Some(Self::CollectionRenamed),
            "collection_deleted" => Some(Self::CollectionDeleted),
            "collection_merged" => Some(Self::CollectionMerged),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SyncChangelogEntry {
    pub id: i64,
    pub tipo: SyncChangelogTipo,
    pub entidad_id: i32,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SyncChangelogDelta {
    pub cambios: Vec<SyncChangelogEntry>,
    pub cursor: i64,
    pub hay_mas: bool,
    pub full_sync_required: bool,
}

/* [174A-91] Acepta tanto `cursor` (legacy contract) como `since` (alias del roadmap)
 * para no romper clientes existentes. `limite` se acota 1..=500 dentro del handler. */
#[derive(Debug, Clone, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct SyncChangelogQuery {
    #[serde(default)]
    pub cursor: Option<i64>,
    #[serde(default)]
    pub since: Option<i64>,
    #[serde(default)]
    pub limite: Option<i64>,
}
