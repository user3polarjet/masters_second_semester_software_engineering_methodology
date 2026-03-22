use plotters::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// --- СТРУКТУРИ ДЛЯ ПАРСИНГУ JSON ---
#[derive(Deserialize)]
struct PlotData {
    size_vs_density_unfiltered: XYData<i64, f64>,
    size_vs_density_filtered: XYData<i64, f64>,
    files_vs_comments: XYData<i64, usize>,
    body_len_vs_comment_count: XYData<usize, usize>,
    body_len_vs_total_comment_len: XYData<usize, usize>,
    ping_pong_syndrome: XYData<i64, f64>,
}

#[derive(Deserialize)]
struct XYData<X, Y> {
    x: Vec<X>,
    y: Vec<Y>,
}

// --- МАТЕМАТИКА: Кореляція Пірсона ---
fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    if n == 0.0 { return 0.0; }

    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_x_sq: f64 = x.iter().map(|&v| v * v).sum();
    let sum_y_sq: f64 = y.iter().map(|&v| v * v).sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(&a, &b)| a * b).sum();

    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x_sq - sum_x * sum_x) * (n * sum_y_sq - sum_y * sum_y)).sqrt();

    if denominator == 0.0 { 0.0 } else { numerator / denominator }
}

// --- ЛОГІКА МАЛЮВАННЯ ГРАФІКІВ ---
fn draw_scatter_plot(
    filepath: &Path,
    title: &str,
    x_label: &str,
    y_label: &str,
    x_data: &[f64],
    y_data: &[f64],
    color: RGBColor,
) -> Result<(), Box<dyn std::error::Error>> {
    if x_data.is_empty() { return Ok(()); }

    // Знаходимо мінімуми та максимуми з відступом 5% для красивих осей
    let min_x = x_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_x = x_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let min_y = y_data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_y = y_data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let pad_x = (max_x - min_x) * 0.05;
    let pad_y = (max_y - min_y) * 0.05;

    // ЗМІНА: Використовуємо BitMapBackend замість SVGBackend для PNG
    let root = BitMapBackend::new(&filepath, (1024, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 24).into_font())
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(
            (min_x - pad_x)..(max_x + pad_x),
            (min_y - pad_y)..(max_y + pad_y),
        )?;

    chart.configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .light_line_style(&WHITE.mix(0.0)) // Ховаємо дрібну сітку для чистоти
        .draw()?;

    // Малюємо крапки з напівпрозорістю (alpha = 0.4)
    chart.draw_series(x_data.iter().zip(y_data.iter()).map(|(&x, &y)| {
        Circle::new((x, y), 3, color.mix(0.4).filled())
    }))?;

    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_dir = Path::new("build");
    let plots_dir = build_dir.join("plots");
    fs::create_dir_all(&plots_dir)?;

    let data_file = build_dir.join("plot_data.json");
    if !data_file.exists() {
        println!("Файл plot_data.json не знайдено!");
        return Ok(());
    }

    println!("Завантаження даних з JSON...");
    let file_content = fs::read_to_string(&data_file)?;
    let all_plot_data: HashMap<String, PlotData> = serde_json::from_str(&file_content)?;

    // Глобальні масиви для всіх графіків (об'єднуємо всі репозиторії)
    let mut g1_x = Vec::new(); let mut g1_y = Vec::new();
    let mut g2_x = Vec::new(); let mut g2_y = Vec::new();
    let mut g3_x = Vec::new(); let mut g3_y = Vec::new();
    let mut g4_x = Vec::new(); let mut g4_y = Vec::new();
    let mut g5_x = Vec::new(); let mut g5_y = Vec::new();
    let mut g6_x = Vec::new(); let mut g6_y = Vec::new();

    for data in all_plot_data.values() {
        g1_x.extend(data.size_vs_density_unfiltered.x.iter().map(|&v| v as f64));
        g1_y.extend(&data.size_vs_density_unfiltered.y);

        g2_x.extend(data.size_vs_density_filtered.x.iter().map(|&v| v as f64));
        g2_y.extend(&data.size_vs_density_filtered.y);

        g3_x.extend(data.files_vs_comments.x.iter().map(|&v| v as f64));
        g3_y.extend(data.files_vs_comments.y.iter().map(|&v| v as f64));

        g4_x.extend(data.body_len_vs_comment_count.x.iter().map(|&v| v as f64));
        g4_y.extend(data.body_len_vs_comment_count.y.iter().map(|&v| v as f64));

        g5_x.extend(data.body_len_vs_total_comment_len.x.iter().map(|&v| v as f64));
        g5_y.extend(data.body_len_vs_total_comment_len.y.iter().map(|&v| v as f64));

        g6_x.extend(data.ping_pong_syndrome.x.iter().map(|&v| v as f64));
        g6_y.extend(&data.ping_pong_syndrome.y);
    }

    println!("\nАналіз та побудова графіків...");

    // ЗМІНА: Змінено розширення файлів з .svg на .png
    // 1
    draw_scatter_plot(&plots_dir.join("1_size_vs_density_unfiltered.png"), 
        "Розмір PR vs Щільність коментарів (Усі дані)", "Розмір PR (рядки)", "Щільність коментарів", &g1_x, &g1_y, BLUE)?;

    // 2
    let corr2 = pearson_correlation(&g2_x, &g2_y);
    println!("Кореляція (Розмір vs Щільність, до 1000 рядків): {:.4}", corr2);
    draw_scatter_plot(&plots_dir.join("2_size_vs_density_filtered.png"), 
        "Розмір PR (до 1000) vs Щільність коментарів", "Розмір PR (рядки)", "Щільність коментарів", &g2_x, &g2_y, BLUE)?;

    // 3
    let corr3 = pearson_correlation(&g3_x, &g3_y);
    println!("Кореляція (Файли vs Коментарі): {:.4}", corr3);
    let orange = RGBColor(255, 165, 0);
    draw_scatter_plot(&plots_dir.join("3_files_vs_comments.png"), 
        "Змінені файли vs Кількість коментарів", "Кількість змінених файлів", "Загальна кількість коментарів", &g3_x, &g3_y, orange)?;

    // 4
    draw_scatter_plot(&plots_dir.join("4_body_len_vs_comment_count.png"), 
        "Довжина опису PR vs Кількість коментарів", "Довжина опису PR (символи)", "Кількість коментарів", &g4_x, &g4_y, orange)?;

    // 5
    draw_scatter_plot(&plots_dir.join("5_body_len_vs_total_comment_len.png"), 
        "Довжина опису PR vs Загальний обсяг дискусії", "Довжина опису PR (символи)", "Сумарна довжина коментарів", &g5_x, &g5_y, orange)?;

    // 6
    let purple = RGBColor(128, 0, 128);
    draw_scatter_plot(&plots_dir.join("6_ping_pong_syndrome.png"), 
        "Синдром Пінг-Понгу: Цикли vs Час до злиття", "Цикли Пінг-Понгу (Коментар-Коміт)", "Час до злиття (дні)", &g6_x, &g6_y, purple)?;

    println!("\n✅ Графіки успішно збережено у форматі PNG у папку {:?}", plots_dir);

    Ok(())
}