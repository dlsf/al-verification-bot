use crate::anilist::self_user_view::SelfUserViewViewer;
use crate::utils::errors::GraphQLError;
use crate::Data;
use anyhow::{Error, Result};
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::{Client, StatusCode, Url};
use serde::{Deserialize, Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "resources/schema.graphql",
    query_path = "resources/query.graphql",
)]
struct SelfUserView;

pub async fn get_user_information(auth_code: &str, data: &Data) -> Result<SelfUserViewViewer> {
    let bearer_token = get_bearer_token(auth_code, data).await?;

    let client = Client::builder()
        .user_agent("graphql-rust/0.14.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", bearer_token))?,
            )).collect(),
        )
        .build()?;

    let response_body = post_graphql::<SelfUserView, _>(&client, "https://graphql.anilist.co/", self_user_view::Variables {}).await?;
    Ok(response_body.data.ok_or(GraphQLError::RequestError)?.viewer.ok_or(GraphQLError::ViewError)?)
}

#[derive(Serialize)]
struct BearerRequest {
    grant_type: String,
    client_id: u32,
    client_secret: String,
    redirect_uri: String,
    code: String
}

#[derive(Deserialize)]
struct BearerResponse {
    access_token: String
}

async fn get_bearer_token(auth_code: &str, data: &Data) -> Result<String> {
    let client = Client::builder()
        .build()?;

    let json_body = serde_json::to_string(&BearerRequest {
        code: auth_code.to_string(),
        client_id: data.client_id,
        client_secret: data.client_secret.clone(),
        grant_type: String::from("authorization_code"),
        redirect_uri: Url::parse("https://anilist.co/api/v2/oauth/pin").unwrap().to_string()
    })?;

    let response = client.post("https://anilist.co/api/v2/oauth/token")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json_body)
        .send()
        .await?;

    if response.status() == StatusCode::OK {
        let bearer_response: BearerResponse = serde_json::from_str(response.text().await?.as_str())?;
        Ok(bearer_response.access_token)
    } else {
        Err(Error::new(GraphQLError::RequestError))
    }
}
