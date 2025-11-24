use reqwest::{Client};
use serde::{de::DeserializeOwned, Serialize};

pub struct ApiClient {
    base_url: String,
    http: Client,
    token: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: Client::new(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    fn build_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    fn apply_headers<'a>(
        &'a self,
        mut req: reqwest::RequestBuilder,
        extra_headers: Option<&[(&str, &str)]>,
    ) -> reqwest::RequestBuilder {
        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        if let Some(headers) = extra_headers {
            for (k, v) in headers {
                req = req.header(*k, *v);
            }
        }

        req
    }

    // -----------------------
    //        GET JSON
    // -----------------------
    pub async fn get_json<TResp>(
        &self,
        path: &str,
        extra_headers: Option<&[(&str, &str)]>,
    ) -> Result<TResp, Box<dyn std::error::Error + Send + Sync>>
    where
        TResp: DeserializeOwned,
    {
        let url = self.build_url(path);
        let req = self.http.get(&url);
        let req = self.apply_headers(req, extra_headers);

        let resp = req.send().await?.error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }

    // -----------------------
    //   POST application/json
    // -----------------------
    pub async fn post_json<TBody, TResp>(
        &self,
        path: &str,
        body: &TBody,
        extra_headers: Option<&[(&str, &str)]>,
    ) -> Result<TResp, Box<dyn std::error::Error + Send + Sync>>
    where
        TBody: Serialize + ?Sized,
        TResp: DeserializeOwned,
    {
        let url = self.build_url(path);
        let req = self.http.post(&url).json(body);
        let req = self.apply_headers(req, extra_headers);

        let resp = req.send().await?.error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }

    // --------------------------------------
    //   POST application/x-www-form-urlencoded
    // --------------------------------------
    pub async fn post_form<TForm, TResp>(
        &self,
        path: &str,
        form: &TForm,
        extra_headers: Option<&[(&str, &str)]>,
    ) -> Result<TResp, Box<dyn std::error::Error + Send + Sync>>
    where
        TForm: Serialize + ?Sized,
        TResp: DeserializeOwned,
    {
        let url = self.build_url(path);
        let req = self.http.post(&url).form(form);
        let req = self.apply_headers(req, extra_headers);

        let resp = req.send().await?.error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }

    // -----------------------
    //   PUT application/json
    // -----------------------
    pub async fn put_json<TBody, TResp>(
        &self,
        path: &str,
        body: &TBody,
        extra_headers: Option<&[(&str, &str)]>,
    ) -> Result<TResp, Box<dyn std::error::Error + Send + Sync>>
    where
        TBody: Serialize + ?Sized,
        TResp: DeserializeOwned,
    {
        let url = self.build_url(path);
        let req = self.http.put(&url).json(body);
        let req = self.apply_headers(req, extra_headers);

        let resp = req.send().await?.error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }

    // --------------------------------------
    //   PUT application/x-www-form-urlencoded
    // --------------------------------------
    pub async fn put_form<TForm, TResp>(
        &self,
        path: &str,
        form: &TForm,
        extra_headers: Option<&[(&str, &str)]>,
    ) -> Result<TResp, Box<dyn std::error::Error + Send + Sync>>
    where
        TForm: Serialize + ?Sized,
        TResp: DeserializeOwned,
    {
        let url = self.build_url(path);
        let req = self.http.put(&url).form(form);
        let req = self.apply_headers(req, extra_headers);

        let resp = req.send().await?.error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }

    // -----------------------
    //         DELETE
    // -----------------------
    pub async fn delete_json<TResp>(
        &self,
        path: &str,
        extra_headers: Option<&[(&str, &str)]>,
    ) -> Result<TResp, Box<dyn std::error::Error + Send + Sync>>
    where
        TResp: DeserializeOwned,
    {
        let url = self.build_url(path);
        let req = self.http.delete(&url);
        let req = self.apply_headers(req, extra_headers);

        let resp = req.send().await?.error_for_status()?;
        Ok(resp.json::<TResp>().await?)
    }
}
#[cfg(test)]
#[path = "./api_client_tests.rs"]
mod api_client_tests;
