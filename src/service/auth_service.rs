use crate::models::AuthToken;
use crate::repository::auth_repository::AuthRepository;

pub struct AuthService<R: AuthRepository> {
    repo: R,
}

impl<R: AuthRepository> AuthService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn login(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<AuthToken, Box<dyn std::error::Error + Send + Sync>> {
        self.repo.authenticate(client_id, client_secret).await
    }
}

#[cfg(test)]
#[path = "auth_service_tests.rs"]
mod auth_service_tests;
