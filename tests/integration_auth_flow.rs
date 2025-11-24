use httpmock::prelude::*;
use rust_api_client::api::ApiClient;
use rust_api_client::models::AuthToken;
use rust_api_client::repository::auth_repository::RestAuthRepository;
use rust_api_client::service::auth_service::AuthService;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn give_valid_credentials_when_login_e2e_then_token_should_be_returned() {
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

    let repo = RestAuthRepository::new(&server.base_url(), auth_path);
    let service = AuthService::new(repo);

    let token = service
        .login("my_id", "my_secret")
        .await
        .expect("token expected");

    mock.assert();
    assert_eq!(token.access_token, "abc123");
    assert_eq!(token.token_type, "Bearer");
    assert_eq!(token.expires_in, Some(3600));
    assert_eq!(token.refresh_token.as_deref(), Some("refresh"));
    assert_eq!(token.scope.as_deref(), Some("read write"));
}

#[tokio::test]
async fn give_invalid_credentials_when_login_e2e_then_error_should_be_propagated() {
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

    let repo = RestAuthRepository::new(&server.base_url(), auth_path);
    let service = AuthService::new(repo);

    let result = service.login("bad", "wrong").await;

    mock.assert();
    assert!(result.is_err(), "expected authentication failure");
}

#[tokio::test]
async fn give_crud_endpoints_when_calling_api_client_e2e_then_responses_should_match() {
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Item {
        id: u32,
        name: String,
    }

    let server = MockServer::start();

    let get_mock = server.mock(|when, then| {
        when.method(GET).path("/items/1");
        then.status(200).json_body_obj(&Item {
            id: 1,
            name: "one".into(),
        });
    });

    let post_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/items")
            .json_body_obj(&serde_json::json!({ "name": "two" }));
        then.status(200).json_body_obj(&Item {
            id: 2,
            name: "two".into(),
        });
    });

    let put_mock = server.mock(|when, then| {
        when.method(PUT)
            .path("/items/2")
            .json_body_obj(&serde_json::json!({ "name": "two-updated" }));
        then.status(200).json_body_obj(&Item {
            id: 2,
            name: "two-updated".into(),
        });
    });

    let delete_mock = server.mock(|when, then| {
        when.method(DELETE).path("/items/2");
        then.status(200).json_body_obj(&serde_json::json!({
            "deleted": true
        }));
    });

    // give
    let client = ApiClient::new(server.base_url());

    // when
    let fetched: Item = client.get_json("/items/1", None).await.unwrap();
    let created: Item = client
        .post_json("/items", &serde_json::json!({ "name": "two" }), None)
        .await
        .unwrap();
    let updated: Item = client
        .put_json("/items/2", &serde_json::json!({ "name": "two-updated" }), None)
        .await
        .unwrap();
    let deleted: serde_json::Value = client.delete_json("/items/2", None).await.unwrap();

    // then
    get_mock.assert();
    post_mock.assert();
    put_mock.assert();
    delete_mock.assert();
    assert_eq!(
        fetched,
        Item {
            id: 1,
            name: "one".into()
        }
    );
    assert_eq!(
        created,
        Item {
            id: 2,
            name: "two".into()
        }
    );
    assert_eq!(
        updated,
        Item {
            id: 2,
            name: "two-updated".into()
        }
    );
    assert_eq!(deleted["deleted"], serde_json::json!(true));
}
