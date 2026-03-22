use graphql_client::{GraphQLQuery};
use serde::Serialize;
use tokio::fs;

type DateTime = String;

const REPOSITORIES: &[(&str, &str)] = &[
    ("rust-lang", "rust"),
    ("facebook", "react"),
    ("tiangolo", "fastapi"),
    ("typst", "typst"),

    ("vuejs", "vue"),
    ("angular", "angular"),
    ("flutter", "flutter"),
    ("vercel", "next.js"),

    ("django", "django"),
    ("spring-projects", "spring-boot"),
    ("laravel", "laravel"),
    ("rails", "rails"),

    ("python", "cpython"),
    ("golang", "go"),
    ("microsoft", "TypeScript"),
    ("nodejs", "node"),

    ("kubernetes", "kubernetes"),
    ("moby", "moby"),
    ("hashicorp", "terraform"),
    ("ansible", "ansible"),
    ("prometheus", "prometheus"),

    ("tensorflow", "tensorflow"),
    ("pytorch", "pytorch"),
    ("huggingface", "transformers"),
    ("scikit-learn", "scikit-learn"),

    ("microsoft", "vscode"),
    ("Homebrew", "brew"),
    ("vitejs", "vite"),];

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/pr_query.graphql",
    response_derives = "Debug, Serialize"
)]
pub struct PrQuery;

#[derive(Serialize)]
struct GhPullRequest {
    number: i64,
    title: String,
    body: String,
    author: Option<GhAuthor>,
    additions: i64,
    deletions: i64,
    #[serde(rename = "changedFiles")]
    changed_files: i64,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "mergedAt")]
    merged_at: Option<String>,
    #[serde(rename = "closedAt")]
    closed_at: Option<String>,
    #[serde(rename = "reviewDecision")]
    review_decision: Option<String>,
    labels: Vec<GhLabel>,
    comments: Vec<GhComment>,
    commits: Vec<GhCommit>,
    reviews: Vec<GhReview>,
}

#[derive(Serialize)]
struct GhAuthor {
    login: String,
}

#[derive(Serialize)]
struct GhLabel {
    name: String,
}

#[derive(Serialize)]
struct GhComment {
    author: Option<GhAuthor>,
    body: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Serialize)]
struct GhCommit {
    #[serde(rename = "committedDate")]
    committed_date: String,
    #[serde(rename = "messageHeadline")]
    message_headline: String,
}

#[derive(Serialize)]
struct GhReview {
    author: Option<GhAuthor>,
    state: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let github_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        format!("Bearer {}", github_token).parse()?,
    );

    let client = reqwest::Client::builder()
        .user_agent("graphql-rust/0.13.0")
        .default_headers(headers)
        .build()?;

    fs::create_dir_all("build").await?;

    for &(owner, name) in REPOSITORIES {
        let repo_dir = format!("build/{}_{}", owner, name);
        fs::create_dir_all(&repo_dir).await?;

        let max_prs_per_repo = 5000; 

        // 1. COUNT EXISTING FILES
        let mut fetched_count = 0;
        if let Ok(mut entries) = fs::read_dir(&repo_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.starts_with("pr_view_") && file_name.ends_with(".json") {
                    fetched_count += 1;
                }
            }
        }

        if fetched_count >= max_prs_per_repo {
            println!("⏭️ Skipping {}/{} - already fetched {} PRs", owner, name, fetched_count);
            continue;
        }

        println!("Fetching {}/{} (Already have: {})...", owner, name, fetched_count);

        // 2. READ SAVED CURSOR (to resume pagination)
        let cursor_file = format!("{}/.cursor", repo_dir);
        let mut cursor: Option<String> = None;
        if let Ok(saved_cursor) = fs::read_to_string(&cursor_file).await {
            let trimmed = saved_cursor.trim();
            if !trimmed.is_empty() {
                cursor = Some(trimmed.to_string());
                println!("  -> Resuming from saved cursor...");
            }
        }

        let mut has_next_page = true;

        while has_next_page && fetched_count < max_prs_per_repo {
            let variables = pr_query::Variables {
                owner: owner.to_string(),
                name: name.to_string(),
                cursor: cursor.clone(),
            };

            let request_body = PrQuery::build_query(variables);

            let res = client
                .post("https://api.github.com/graphql")
                .json(&request_body)
                .send()
                .await?;

            let status = res.status();
            
            if !status.is_success() {
                let error_text = res.text().await?;
                println!("❌ API Error! Status: {}", status);
                println!("📄 Body: {}", error_text);
                
                if status == reqwest::StatusCode::FORBIDDEN {
                    println!("⏳ Rate limited. Sleeping for 60 seconds...");
                    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                    continue;
                } else {
                    println!("🛑 Critical error. Stopping.");
                    break; // Move to the next repository instead of crashing the whole script
                }
            }

            let response_body: graphql_client::Response<pr_query::ResponseData> = res.json().await?;

            if let Some(data) = response_body.data {
                if let Some(repo) = data.repository {
                    if let Some(prs) = repo.pull_requests.nodes {
                        for pr in prs.into_iter().flatten() {
                            
                            if fetched_count >= max_prs_per_repo {
                                break;
                            }

                            let file_path = format!("{}/pr_view_{}.json", repo_dir, pr.number);
                            
                            // 3. ONLY WRITE IF IT DOESN'T EXIST 
                            // (in case a crash happened mid-page and some files were written but cursor wasn't updated)
                            if !std::path::Path::new(&file_path).exists() {
                                let gh_pr = GhPullRequest {
                                    number: pr.number,
                                    title: pr.title.clone(),
                                    body: pr.body.clone(),
                                    author: pr.author.map(|a| GhAuthor { login: a.login }),
                                    additions: pr.additions,
                                    deletions: pr.deletions,
                                    changed_files: pr.changed_files,
                                    created_at: pr.created_at.clone(),
                                    merged_at: pr.merged_at.clone(),
                                    closed_at: pr.closed_at.clone(),
                                    review_decision: pr.review_decision.map(|d| format!("{:?}", d)),
                                    labels: pr.labels.and_then(|l| l.nodes).unwrap_or_default().into_iter().flatten().map(|l| GhLabel { name: l.name }).collect(),
                                    reviews: pr.reviews.and_then(|r| r.nodes).unwrap_or_default().into_iter().flatten().map(|r| GhReview {
                                        author: r.author.map(|a| GhAuthor { login: a.login }),
                                        state: format!("{:?}", r.state),
                                        created_at: r.created_at,
                                    }).collect(),
                                    comments: pr.comments.nodes.unwrap_or_default().into_iter().flatten().map(|c| GhComment {
                                        author: c.author.map(|a| GhAuthor { login: a.login }),
                                        body: c.body,
                                        created_at: c.created_at,
                                    }).collect(),
                                    commits: pr.commits.nodes.unwrap_or_default().into_iter().flatten().map(|c| GhCommit {
                                        committed_date: c.commit.committed_date,
                                        message_headline: c.commit.message_headline,
                                    }).collect(),
                                };

                                let json_data = serde_json::to_string_pretty(&gh_pr)?;
                                fs::write(&file_path, json_data).await?;
                                
                                fetched_count += 1;
                            }
                        }
                    }

                    println!("  -> Fetched {} / {} PRs for {}/{}", fetched_count, max_prs_per_repo, owner, name);

                    has_next_page = repo.pull_requests.page_info.has_next_page;
                    cursor = repo.pull_requests.page_info.end_cursor;

                    // 4. SAVE THE CURSOR STATE
                    // After successfully processing a page, write the cursor to disk
                    if let Some(ref c) = cursor {
                        fs::write(&cursor_file, c).await?;
                    }

                } else {
                    break;
                }
            } else {
                break;
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await; 
        }
        println!("✅ Finished {}/{}! Total PRs saved: {}", owner, name, fetched_count);
    }

    Ok(())
}
