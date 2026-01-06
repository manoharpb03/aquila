use crate::error::*;

use bytes::Bytes;

pub trait StorageBackend: Send + Sync + 'static + Clone {
    fn write_blob(
        &self,
        hash: &str,
        data: Bytes,
    ) -> impl Future<Output = Result<bool, StorageError>> + Send;
    fn write_manifest(
        &self,
        version: &str,
        data: Bytes,
    ) -> impl Future<Output = Result<(), StorageError>> + Send;
    fn read_file(&self, path: &str) -> impl Future<Output = Result<Bytes, StorageError>> + Send;
    fn exists(&self, path: &str) -> impl Future<Output = Result<bool, StorageError>> + Send;

    fn get_manifest_path(&self, version: &str) -> String {
        format!("manifests/{version}")
    }
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub scopes: Vec<String>,
}

pub trait AuthProvider: Send + Sync + 'static + Clone {
    fn verify(&self, token: &str) -> impl Future<Output = Result<User, AuthError>> + Send;

    /// Optional: Returns a login url to start an auth flow.
    fn get_login_url(&self) -> Option<String> {
        None
    }

    /// Optional: Exchanges an authorization code for a User identity.
    fn exchange_code(&self, _code: &str) -> impl Future<Output = Result<User, AuthError>> + Send {
        async {
            Err(AuthError::Generic(
                "Login flow not supported by this provider".into(),
            ))
        }
    }
}
