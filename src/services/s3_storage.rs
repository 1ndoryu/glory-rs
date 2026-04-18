/* [174A-27] S3Storage feature-gated.
 *
 * Implementacion de FileStorage sobre AWS S3 (o compatible: MinIO, Cloudflare R2,
 * Backblaze B2 con endpoint custom). Activa con `--features s3`.
 *
 * Configuracion via env (leida por el caller, no aqui):
 * - AWS_ACCESS_KEY_ID / AWS_SECRET_ACCESS_KEY (o IAM role en EC2/ECS)
 * - AWS_REGION
 * - S3_BUCKET (lo recibe S3Storage::new como parametro)
 * - S3_ENDPOINT_URL (opcional, para R2/MinIO)
 *
 * NOTA: Esta implementacion lee el stream completo en memoria antes de hacer PutObject.
 * Para uploads grandes (>100MB) habra que migrar a multipart upload (174A-29 lo decidira).
 */

use crate::errors::AppError;
use crate::services::storage::{validate_key, FileStorage};
use async_trait::async_trait;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use tokio::io::{AsyncRead, AsyncReadExt};

#[derive(Clone)]
pub struct S3Storage {
    client: Client,
    bucket: String,
}

impl S3Storage {
    /// Crea un cliente S3 leyendo credenciales del entorno (AWS SDK chain).
    /// `endpoint_url` opcional para S3-compatibles (R2/MinIO).
    pub async fn new(bucket: impl Into<String>, endpoint_url: Option<String>) -> Result<Self, AppError> {
        let mut loader = aws_config::defaults(aws_config::BehaviorVersion::latest());
        if let Some(ep) = endpoint_url {
            loader = loader.endpoint_url(ep);
        }
        let cfg = loader.load().await;
        let s3_cfg = aws_sdk_s3::config::Builder::from(&cfg)
            .force_path_style(true) // necesario para MinIO/R2
            .build();
        let client = Client::from_conf(s3_cfg);
        Ok(Self {
            client,
            bucket: bucket.into(),
        })
    }
}

#[async_trait]
impl FileStorage for S3Storage {
    async fn put_bytes(&self, key: &str, bytes: &[u8]) -> Result<u64, AppError> {
        validate_key(key)?;
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(ByteStream::from(bytes.to_vec()))
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "s3".into(),
                message: format!("put_object: {e}"),
            })?;
        Ok(u64::try_from(bytes.len()).unwrap_or(0))
    }

    async fn put_stream(
        &self,
        key: &str,
        reader: &mut (dyn AsyncRead + Send + Unpin),
    ) -> Result<u64, AppError> {
        validate_key(key)?;
        let mut buf = Vec::with_capacity(1024 * 1024);
        reader
            .read_to_end(&mut buf)
            .await
            .map_err(|e| AppError::Internal(format!("leer stream s3: {e}")))?;
        let len = u64::try_from(buf.len()).unwrap_or(0);
        self.put_bytes(key, &buf).await?;
        Ok(len)
    }

    async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, AppError> {
        validate_key(key)?;
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                let msg = format!("{e}");
                if msg.contains("NoSuchKey") || msg.contains("NotFound") {
                    AppError::NotFound(format!("storage: {key}"))
                } else {
                    AppError::ExternalService {
                        service: "s3".into(),
                        message: format!("get_object: {e}"),
                    }
                }
            })?;
        let bytes = resp
            .body
            .collect()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "s3".into(),
                message: format!("collect body: {e}"),
            })?;
        Ok(bytes.into_bytes().to_vec())
    }

    async fn exists(&self, key: &str) -> Result<bool, AppError> {
        validate_key(key)?;
        match self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                let msg = format!("{e}");
                if msg.contains("NotFound") || msg.contains("NoSuchKey") || msg.contains("404") {
                    Ok(false)
                } else {
                    Err(AppError::ExternalService {
                        service: "s3".into(),
                        message: format!("head_object: {e}"),
                    })
                }
            }
        }
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        validate_key(key)?;
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| AppError::ExternalService {
                service: "s3".into(),
                message: format!("delete_object: {e}"),
            })?;
        Ok(())
    }

    async fn size(&self, key: &str) -> Result<u64, AppError> {
        validate_key(key)?;
        let resp = self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                let msg = format!("{e}");
                if msg.contains("NotFound") || msg.contains("NoSuchKey") || msg.contains("404") {
                    AppError::NotFound(format!("storage: {key}"))
                } else {
                    AppError::ExternalService {
                        service: "s3".into(),
                        message: format!("head_object: {e}"),
                    }
                }
            })?;
        Ok(u64::try_from(resp.content_length().unwrap_or(0)).unwrap_or(0))
    }
}
