use super::*;
use httpmock::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DummyResp {
    message: String,
}

#[tokio::test]
async fn give_extra_headers_when_get_json_then_response_parsed_should_be_ok() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/hello")
            .header("X-Trace", "abc123");
        then.status(200)
            .json_body_obj(&DummyResp {
                message: "hi".into(),
            });
    });

    // give
    let client = ApiClient::new(server.base_url());

    // when
    let resp: DummyResp = client
        .get_json("/hello", Some(&[("X-Trace", "abc123")]))
        .await
        .unwrap();

    // then
    mock.assert();
    assert_eq!(resp, DummyResp { message: "hi".into() });
}

#[tokio::test]
async fn give_token_when_get_json_then_authorization_header_should_be_sent() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/secure")
            .header("authorization", "Bearer token123");
        then.status(200)
            .json_body_obj(&DummyResp {
                message: "secure-hi".into(),
            });
    });

    // give
    let client = ApiClient::new(server.base_url()).with_token("token123".into());

    // when
    let resp: DummyResp = client.get_json("/secure", None).await.unwrap();

    // then
    mock.assert();
    assert_eq!(resp.message, "secure-hi");
}

#[tokio::test]
async fn give_json_body_and_token_when_post_json_then_created_should_be_received() {
    #[derive(Serialize, Deserialize)]
    struct DummyReq<'a> {
        name: &'a str,
    }

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/items")
            .header("authorization", "Bearer token123")
            .json_body_obj(&DummyReq { name: "foo" });
        then.status(200)
            .json_body_obj(&DummyResp {
                message: "created".into(),
            });
    });

    // give
    let client = ApiClient::new(server.base_url()).with_token("token123".into());
    let body = DummyReq { name: "foo" };

    // when
    let resp: DummyResp = client
        .post_json("items", &body, None)
        .await
        .unwrap();

    // then
    mock.assert();
    assert_eq!(resp.message, "created");
}

#[tokio::test]
async fn give_form_data_when_post_form_then_fields_should_be_encoded() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/auth/login")
            .header("content-type", "application/x-www-form-urlencoded")
            .body_contains("client_id=id123")
            .body_contains("client_secret=sec456");
        then.status(200)
            .json_body_obj(&DummyResp {
                message: "ok".into(),
            });
    });

    // give
    let client = ApiClient::new(server.base_url());
    let form = &[("client_id", "id123"), ("client_secret", "sec456")];

    // when
    let resp: DummyResp = client
        .post_form("/auth/login", &form, None)
        .await
        .unwrap();

    // then
    mock.assert();
    assert_eq!(resp.message, "ok");
}

#[tokio::test]
async fn give_payload_when_put_json_then_response_updated_should_arrive() {
    #[derive(Serialize, Deserialize)]
    struct Payload {
        active: bool,
    }

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT)
            .path("/status")
            .json_body_obj(&Payload { active: true });
        then.status(200)
            .json_body_obj(&DummyResp {
                message: "updated".into(),
            });
    });

    // give
    let client = ApiClient::new(server.base_url());

    // when
    let resp: DummyResp = client
        .put_json("/status", &Payload { active: true }, None)
        .await
        .unwrap();

    // then
    mock.assert();
    assert_eq!(resp.message, "updated");
}

#[tokio::test]
async fn give_form_body_when_put_form_then_saved_should_be_returned() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT)
            .path("/profile")
            .header("content-type", "application/x-www-form-urlencoded")
            .body_contains("name=John");
        then.status(200)
            .json_body_obj(&DummyResp {
                message: "saved".into(),
            });
    });

    // give
    let client = ApiClient::new(server.base_url());
    let form = &[("name", "John")];

    // when
    let resp: DummyResp = client
        .put_form("profile", &form, None)
        .await
        .unwrap();

    // then
    mock.assert();
    assert_eq!(resp.message, "saved");
}

#[tokio::test]
async fn give_resource_when_delete_json_then_deleted_should_be_returned() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE).path("/items/42");
        then.status(200)
            .json_body_obj(&DummyResp {
                message: "deleted".into(),
            });
    });

    // give
    let client = ApiClient::new(format!("{}/", server.base_url()));

    // when
    let resp: DummyResp = client
        .delete_json("/items/42", None)
        .await
        .unwrap();

    // then
    mock.assert();
    assert_eq!(resp.message, "deleted");
}
