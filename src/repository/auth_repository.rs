use crate::api::ApiClient;
use crate::models::AuthToken;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait AuthRepository {
    async fn authenticate(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<AuthToken, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct RestAuthRepository {
    client: ApiClient,
    auth_path: String,
}

impl RestAuthRepository {
    pub fn new(base_url: &str, auth_path: &str) -> Self {
        Self {
            client: ApiClient::new(base_url),
            auth_path: auth_path.to_string(),
        }
    }
}

#[async_trait]
impl AuthRepository for RestAuthRepository {
    async fn authenticate(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<AuthToken, Box<dyn std::error::Error + Send + Sync>> {
        #[derive(Serialize)]
        struct AuthForm<'a> {
            client_id: &'a str,
            client_secret: &'a str,
        }

        let form = AuthForm {
            client_id,
            client_secret,
        };

        let token: AuthToken = self
            .client
            .post_form(&self.auth_path, &form, None)
            .await?;

        Ok(token)
    }
}

#[cfg(test)]
#[path = "auth_repository_tests.rs"]
mod auth_repository_tests;
