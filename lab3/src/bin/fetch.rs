use graphql_client::{GraphQLQuery, Response};
use reqwest::Client;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/repository_query.graphql",
    response_derives = "Debug"
)]
pub struct RepositoryView;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                "Authorization",
                format!("Bearer {}", github_token).parse().unwrap(),
            );
            headers
        })
        .build()?;

    let variables = repository_view::Variables {
        owner: "typst".to_string(),
        name: "typst".to_string(),
    };

    let request_body = RepositoryView::build_query(variables);

    let res = client
        .post("https://api.github.com/graphql")
        .json(&request_body)
        .send()
        .await?;

    let response_body: Response<repository_view::ResponseData> = res.json().await?;

    if let Some(data) = response_body.data {
        let repo = data.repository.unwrap();
        println!("Repo: {}", repo.name);
        println!("Stars: {}", repo.stargazer_count);
        println!("Total Merged PRs: {}", repo.pull_requests.total_count);
        
        // You can now iterate over the exact types returned!
        if let Some(nodes) = repo.pull_requests.nodes {
            for pr in nodes.into_iter().flatten() {
                println!("PR: {} (+{} -{})", pr.title, pr.additions, pr.deletions);
            }
        }
    }

    Ok(())
}
