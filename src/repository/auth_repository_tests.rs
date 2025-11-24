use super::*;
use crate::models::AuthToken;
use httpmock::prelude::*;

#[tokio::test]
async fn give_valid_credentials_when_authenticate_then_token_should_be_returned() {
    let server = MockServer::start();
    let auth_path = "/auth/login";

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(auth_path)
            .header("content-type", "application/x-www-form-urlencoded")
            .body_contains("client_id=my_id")
            .body_contains("client_secret=my_secret");
        then.status(200).json_body_obj(&AuthToken {
            access_token: "abc123".into(),
            token_type: "Bearer".into(),
            expires_in: Some(3600),
            refresh_token: Some("refresh".into()),
            scope: Some("read write".into()),
        });
    });

    // give
    let repo = RestAuthRepository::new(&server.base_url(), auth_path);

    // when
    let token = repo
        .authenticate("my_id", "my_secret")
        .await
        .expect("token expected");

    // then
    mock.assert();
    assert_eq!(token.access_token, "abc123");
    assert_eq!(token.token_type, "Bearer");
    assert_eq!(token.expires_in, Some(3600));
    assert_eq!(token.refresh_token.as_deref(), Some("refresh"));
    assert_eq!(token.scope.as_deref(), Some("read write"));
}

#[tokio::test]
async fn give_invalid_credentials_when_authenticate_then_error_should_be_propagated() {
    let server = MockServer::start();
    let auth_path = "/auth/login";

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(auth_path)
            .body_contains("client_id=bad")
            .body_contains("client_secret=wrong");
        then.status(401).json_body_obj(&serde_json::json!({
            "error": "invalid_client"
        }));
    });

    // give
    let repo = RestAuthRepository::new(&server.base_url(), auth_path);

    // when
    let result = repo.authenticate("bad", "wrong").await;

    // then
    mock.assert();
    assert!(result.is_err(), "expected authentication failure");
}
