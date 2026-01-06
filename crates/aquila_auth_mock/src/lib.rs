use aquila_core::prelude::*;

#[derive(Clone)]
pub struct AllowAllAuth;

impl AuthProvider for AllowAllAuth {
    async fn verify(&self, _token: &str) -> Result<User, AuthError> {
        Ok(User {
            id: "dev_user".to_string(),
            scopes: vec!["admin".to_string(), "read".to_string(), "write".to_string()],
        })
    }
}
