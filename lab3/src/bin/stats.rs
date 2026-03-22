use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// --- СТРУКТУРИ ДЛЯ ПАРСИНГУ ЗБЕРЕЖЕНИХ JSON ---

#[derive(Deserialize)]
struct PrAuthor {
    login: String,
}

#[derive(Deserialize)]
struct PrComment {
    author: Option<PrAuthor>,
    body: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

#[derive(Deserialize)]
struct PrCommit {
    #[serde(rename = "committedDate")]
    committed_date: String,
}

#[derive(Deserialize)]
struct PullRequest {
    additions: i64,
    deletions: i64,
    #[serde(rename = "changedFiles")]
    changed_files: i64,
    body: String,
    author: Option<PrAuthor>,
    #[serde(rename = "createdAt")]
    created_at: String,
    #[serde(rename = "mergedAt")]
    merged_at: Option<String>,
    comments: Vec<PrComment>,
    commits: Vec<PrCommit>,
}

// --- СТРУКТУРИ ДЛЯ ЗБЕРЕЖЕННЯ РЕЗУЛЬТАТІВ (ГРАФІКІВ) ---

#[derive(Serialize, Default)]
struct PlotData {
    size_vs_density_unfiltered: XYData<i64, f64>,
    size_vs_density_filtered: XYData<i64, f64>,
    files_vs_comments: XYData<i64, usize>,
    body_len_vs_comment_count: XYData<usize, usize>,
    body_len_vs_total_comment_len: XYData<usize, usize>,
    ping_pong_syndrome: XYData<i64, f64>, // cycles (x), days (y)
}

#[derive(Serialize, Default)]
struct XYData<X, Y> {
    x: Vec<X>,
    y: Vec<Y>,
}

#[derive(PartialEq, Eq)]
enum EventType {
    Commit,
    Comment,
}

struct Event {
    time: DateTime<FixedOffset>,
    kind: EventType,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_dir = Path::new("build");
    if !build_dir.exists() {
        println!("Папка 'build' не знайдена. Спочатку запустіть скрипт завантаження.");
        return Ok(());
    }

    let mut all_plot_data: HashMap<String, PlotData> = HashMap::new();

    // Проходимося по всіх папках (репозиторіях) у build/
    for entry in fs::read_dir(build_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let repo_name = entry.file_name().to_string_lossy().into_owned();
            print!("Обробка репозиторію: {} ... ", repo_name);

            let mut repo_data = PlotData::default();
            let mut pr_count = 0;

            // Проходимося по всіх JSON файлах у папці репозиторію
            for file_entry in fs::read_dir(&path)? {
                let file_entry = file_entry?;
                let file_path = file_entry.path();
                let file_name = file_entry.file_name().to_string_lossy().into_owned();

                if file_name.starts_with("pr_view_") && file_name.ends_with(".json") {
                    let file_content = fs::read_to_string(&file_path)?;
                    
                    // Парсимо JSON
                    let pr: PullRequest = match serde_json::from_str(&file_content) {
                        Ok(data) => data,
                        Err(_) => continue, // Пропускаємо пошкоджені файли
                    };

                    pr_count += 1;

                    // Базові метрики
                    let size = pr.additions + pr.deletions;
                    let comment_count = pr.comments.len();
                    let body_len = pr.body.chars().count();
                    let total_comment_len: usize = pr.comments.iter().map(|c| c.body.chars().count()).sum();

                    // 1 & 2: Розмір vs Щільність
                    if size > 0 {
                        let density = comment_count as f64 / size as f64;
                        repo_data.size_vs_density_unfiltered.x.push(size);
                        repo_data.size_vs_density_unfiltered.y.push(density);

                        if size <= 1000 {
                            repo_data.size_vs_density_filtered.x.push(size);
                            repo_data.size_vs_density_filtered.y.push(density);
                        }
                    }

                    // 3: Файли vs Коментарі
                    if pr.changed_files < 100 {
                        repo_data.files_vs_comments.x.push(pr.changed_files);
                        repo_data.files_vs_comments.y.push(comment_count);
                    }

                    // 4 & 5: Довжина опису vs Дискусії
                    if body_len < 4000 {
                        repo_data.body_len_vs_comment_count.x.push(body_len);
                        repo_data.body_len_vs_comment_count.y.push(comment_count);

                        repo_data.body_len_vs_total_comment_len.x.push(body_len);
                        repo_data.body_len_vs_total_comment_len.y.push(total_comment_len);
                    }

                    // 6: Синдром Пінг-Понгу
                    if let (Some(merged_at_str), Some(pr_author)) = (&pr.merged_at, &pr.author) {
                        let mut events: Vec<Event> = Vec::new();

                        // Парсимо коміти
                        for c in &pr.commits {
                            if let Ok(dt) = DateTime::parse_from_rfc3339(&c.committed_date) {
                                events.push(Event { time: dt, kind: EventType::Commit });
                            }
                        }

                        // Парсимо коментарі (тільки від ІНШИХ людей)
                        for c in &pr.comments {
                            if let Some(c_author) = &c.author {
                                if c_author.login != pr_author.login {
                                    if let Ok(dt) = DateTime::parse_from_rfc3339(&c.created_at) {
                                        events.push(Event { time: dt, kind: EventType::Comment });
                                    }
                                }
                            }
                        }

                        // Сортуємо хронологічно
                        events.sort_by_key(|e| e.time);

                        // Шукаємо перший коментар
                        if let Some(first_comment_idx) = events.iter().position(|e| e.kind == EventType::Comment) {
                            let mut ping_pongs = 0;
                            let mut prev_kind = &events[first_comment_idx].kind;

                            for e in &events[first_comment_idx + 1..] {
                                if e.kind != *prev_kind {
                                    ping_pongs += 1;
                                    prev_kind = &e.kind;
                                }
                            }

                            let cycles = ping_pongs / 2;

                            if let (Ok(created_dt), Ok(merged_dt)) = (
                                DateTime::parse_from_rfc3339(&pr.created_at),
                                DateTime::parse_from_rfc3339(merged_at_str),
                            ) {
                                let duration = merged_dt.signed_duration_since(created_dt);
                                let days = duration.num_seconds() as f64 / 86400.0;

                                if days <= 150.0 {
                                    repo_data.ping_pong_syndrome.x.push(cycles);
                                    repo_data.ping_pong_syndrome.y.push(days);
                                }
                            }
                        }
                    }
                }
            }

            println!("Оброблено {} ПР.", pr_count);
            if pr_count > 0 {
                all_plot_data.insert(repo_name, repo_data);
            }
        }
    }

    // Зберігаємо результати
    let output_path = build_dir.join("plot_data.json");
    let json_output = serde_json::to_string_pretty(&all_plot_data)?;
    fs::write(&output_path, json_output)?;

    println!("\n✅ Всі розрахунки завершено! Дані збережено у: {:?}", output_path);

    Ok(())
}
