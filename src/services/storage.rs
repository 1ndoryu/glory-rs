/* [174A-26] Trait FileStorage + impl LocalFs.
 *
 * Abstracción mínima de almacenamiento de archivos. Permite intercambiar
 * backend (LocalFs en dev / staging, S3Storage feature-gated en 174A-27) sin
 * tocar handlers ni servicios.
 *
 * Convenciones:
 * - `key` es una ruta lógica relativa (ej: "samples/2026/04/abc.mp3"). Nunca
 *   debe empezar con "/" ni contener ".." o secuencias de path traversal.
 * - El backend es responsable de crear directorios intermedios si aplica.
 * - `put_stream` consume el body completo. La validación de tamaño máximo se
 *   hace en el handler (multipart) ANTES de llamar al storage.
 */

use crate::errors::AppError;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};

#[async_trait]
pub trait FileStorage: Send + Sync {
    /// Guarda bytes leyendo desde un `AsyncRead` y devuelve la cantidad escrita.
    async fn put_stream(
        &self,
        key: &str,
        reader: &mut (dyn AsyncRead + Send + Unpin),
    ) -> Result<u64, AppError>;

    /// Devuelve los bytes completos del archivo. Solo apto para archivos pequeños.
    async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, AppError>;

    /// True si el objeto existe.
    async fn exists(&self, key: &str) -> Result<bool, AppError>;

    /// Borra el objeto. No falla si no existe.
    async fn delete(&self, key: &str) -> Result<(), AppError>;

    /// Tamaño en bytes del objeto. Error si no existe.
    async fn size(&self, key: &str) -> Result<u64, AppError>;
}

/// Valida que `key` sea una ruta relativa segura sin path traversal.
/// Comparte la lógica entre LocalFs y futuros backends.
pub fn validate_key(key: &str) -> Result<(), AppError> {
    if key.is_empty() {
        return Err(AppError::BadRequest("key vacia".into()));
    }
    if key.starts_with('/') || key.starts_with('\\') {
        return Err(AppError::BadRequest("key absoluta no permitida".into()));
    }
    for component in Path::new(key).components() {
        match component {
            std::path::Component::Normal(_) => {}
            _ => return Err(AppError::BadRequest("key con componentes invalidos".into())),
        }
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct LocalFs {
    root: PathBuf,
}

impl LocalFs {
    /// Crea una instancia y asegura que el root existe (creando si hace falta).
    pub async fn new(root: impl Into<PathBuf>) -> Result<Self, AppError> {
        let root = root.into();
        tokio::fs::create_dir_all(&root)
            .await
            .map_err(|e| AppError::Internal(format!("crear storage_root {}: {e}", root.display())))?;
        Ok(Self { root })
    }

    fn resolve(&self, key: &str) -> Result<PathBuf, AppError> {
        validate_key(key)?;
        Ok(self.root.join(key))
    }
}

#[async_trait]
impl FileStorage for LocalFs {
    async fn put_stream(
        &self,
        key: &str,
        reader: &mut (dyn AsyncRead + Send + Unpin),
    ) -> Result<u64, AppError> {
        let path = self.resolve(key)?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| AppError::Internal(format!("crear dir {}: {e}", parent.display())))?;
        }
        let mut file = tokio::fs::File::create(&path)
            .await
            .map_err(|e| AppError::Internal(format!("crear archivo {}: {e}", path.display())))?;
        let mut buf = vec![0u8; 64 * 1024];
        let mut written: u64 = 0;
        loop {
            let n = reader
                .read(&mut buf)
                .await
                .map_err(|e| AppError::Internal(format!("leer stream: {e}")))?;
            if n == 0 {
                break;
            }
            file.write_all(&buf[..n])
                .await
                .map_err(|e| AppError::Internal(format!("escribir {}: {e}", path.display())))?;
            written += u64::try_from(n).unwrap_or(0);
        }
        file.flush()
            .await
            .map_err(|e| AppError::Internal(format!("flush {}: {e}", path.display())))?;
        Ok(written)
    }

    async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, AppError> {
        let path = self.resolve(key)?;
        tokio::fs::read(&path)
            .await
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => AppError::NotFound(format!("storage: {key}")),
                _ => AppError::Internal(format!("leer {}: {e}", path.display())),
            })
    }

    async fn exists(&self, key: &str) -> Result<bool, AppError> {
        let path = self.resolve(key)?;
        match tokio::fs::metadata(&path).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(AppError::Internal(format!("metadata {}: {e}", path.display()))),
        }
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        let path = self.resolve(key)?;
        match tokio::fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(AppError::Internal(format!("borrar {}: {e}", path.display()))),
        }
    }

    async fn size(&self, key: &str) -> Result<u64, AppError> {
        let path = self.resolve(key)?;
        let meta = tokio::fs::metadata(&path)
            .await
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => AppError::NotFound(format!("storage: {key}")),
                _ => AppError::Internal(format!("metadata {}: {e}", path.display())),
            })?;
        Ok(meta.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn put_get_delete_roundtrip() {
        let tmp = std::env::temp_dir().join(format!("glory-storage-{}", uuid::Uuid::new_v4()));
        let fs = LocalFs::new(&tmp).await.unwrap();

        let data = b"hello world".to_vec();
        let mut cur = std::io::Cursor::new(data.clone());
        let n = fs.put_stream("a/b/test.bin", &mut cur).await.unwrap();
        assert_eq!(n, data.len() as u64);

        assert!(fs.exists("a/b/test.bin").await.unwrap());
        assert_eq!(fs.size("a/b/test.bin").await.unwrap(), data.len() as u64);
        assert_eq!(fs.get_bytes("a/b/test.bin").await.unwrap(), data);

        fs.delete("a/b/test.bin").await.unwrap();
        assert!(!fs.exists("a/b/test.bin").await.unwrap());

        // delete idempotente
        fs.delete("a/b/test.bin").await.unwrap();

        // cleanup
        let _ = tokio::fs::remove_dir_all(&tmp).await;
    }

    #[test]
    fn rejects_path_traversal() {
        assert!(validate_key("../etc/passwd").is_err());
        assert!(validate_key("/abs/path").is_err());
        assert!(validate_key("").is_err());
        assert!(validate_key("ok/relative/file.bin").is_ok());
    }
}
