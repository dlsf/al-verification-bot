use crate::anilist::self_user_view::SelfUserViewViewer;
use crate::errors::GraphQLError;
use graphql_client::{reqwest::post_graphql, GraphQLQuery};
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "resources/schema.graphql",
    query_path = "resources/query.graphql",
)]
struct SelfUserView;

pub async fn get_user_information(token: &str) -> Result<SelfUserViewViewer, anyhow::Error> {
    let client = Client::builder()
        .user_agent("graphql-rust/0.14.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", token))?,
            )).collect(),
        )
        .build()?;

    let response_body = post_graphql::<SelfUserView, _>(&client, "https://graphql.anilist.co/", self_user_view::Variables {}).await?;
    Ok(response_body.data.ok_or(GraphQLError::RequestError)?.viewer.ok_or(GraphQLError::ViewError)?)
}
