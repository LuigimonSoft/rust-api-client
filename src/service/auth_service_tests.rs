use super::*;
use async_trait::async_trait;
use crate::models::AuthToken;
use std::sync::Mutex;
use std::io::{Error, ErrorKind};

struct MockAuthRepo {
    response: MockResponse,
    calls: Mutex<Vec<(String, String)>>,
}

enum MockResponse {
    Success(AuthToken),
    Failure(String),
}

impl MockAuthRepo {
    fn new(response: MockResponse) -> Self {
        Self {
            response,
            calls: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl AuthRepository for MockAuthRepo {
    async fn authenticate(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<AuthToken, Box<dyn std::error::Error + Send + Sync>> {
        self.calls
            .lock()
            .unwrap()
            .push((client_id.to_string(), client_secret.to_string()));

        match &self.response {
            MockResponse::Success(token) => Ok(token.clone()),
            MockResponse::Failure(msg) => Err(Box::new(Error::new(ErrorKind::Other, msg.clone()))),
        }
    }
}

#[tokio::test]
async fn give_valid_credentials_when_login_then_token_should_be_returned() {
    let token = AuthToken {
        access_token: "abc123".into(),
        token_type: "Bearer".into(),
        expires_in: Some(3600),
        refresh_token: Some("refresh".into()),
        scope: Some("read write".into()),
    };

    // give
    let repo = MockAuthRepo::new(MockResponse::Success(token.clone()));
    let service = AuthService::new(repo);

    // when
    let result = service.login("id123", "sec456").await.unwrap();

    // then
    assert_eq!(result.access_token, token.access_token);
    assert_eq!(result.token_type, token.token_type);
    assert_eq!(result.expires_in, token.expires_in);
    assert_eq!(result.refresh_token, token.refresh_token);
    assert_eq!(result.scope, token.scope);
}

#[tokio::test]
async fn give_invalid_credentials_when_login_then_error_should_be_propagated() {
    // give
    let repo = MockAuthRepo::new(MockResponse::Failure("invalid".into()));
    let service = AuthService::new(repo);

    // when
    let result = service.login("bad", "creds").await;

    // then
    assert!(result.is_err(), "expected error to bubble up");
}
