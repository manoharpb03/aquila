use crate::jwt::JwtService;
use aquila_core::traits::{AuthProvider, StorageBackend};

#[derive(Clone)]
pub struct AppState<S: StorageBackend + Clone, A: AuthProvider + Clone> {
    pub storage: S,
    pub auth: A,
    pub jwt_service: JwtService,
}
