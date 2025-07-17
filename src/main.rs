use anyhow::Result;
use arabic_pdf_to_text::{process_pdf, Config};
use clap::Parser;
use console;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Path to the Arabic PDF file")]
    pdf_path: String,

    #[arg(
        short,
        long,
        help = "Gemini API key (can also be set via GEMINI_API_KEY env var)"
    )]
    api_key: Option<String>,

    #[arg(
        short,
        long,
        default_value = "1048576",
        help = "Chunk size in bytes (default: 1MB)"
    )]
    chunk_size: usize,

    #[arg(short, long, help = "Output file path (default: stdout)")]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let api_key = args
        .api_key
        .or_else(|| std::env::var("GEMINI_API_KEY").ok())
        .ok_or_else(|| {
            anyhow::anyhow!("API key must be provided via --api-key or GEMINI_API_KEY env var")
        })?;

    let config = Config {
        api_key,
        chunk_size: args.chunk_size,
    };

    let start_time = std::time::Instant::now();

    let result = process_pdf(&args.pdf_path, &config).await?;

    let elapsed = start_time.elapsed();

    if let Some(output_path) = args.output {
        println!(
            "\n{} {}",
            console::style("💾").cyan(),
            console::style("Saving extracted text to file...")
                .cyan()
                .bold()
        );

        std::fs::write(&output_path, &result)?;

        println!(
            "{} {}",
            console::style("✅").green(),
            console::style(format!("Output saved to: {}", output_path))
                .green()
                .bold()
        );

        println!(
            "\n{} {}",
            console::style("⏱️").cyan(),
            console::style(format!(
                "Total processing time: {:.2} seconds",
                elapsed.as_secs_f64()
            ))
            .cyan()
        );
    } else {
        println!("\n{}", console::style("═".repeat(65)).cyan().bright());
        println!(
            "{} {}",
            console::style("📜").cyan(),
            console::style("EXTRACTED TEXT").cyan().bold().underlined()
        );
        println!("{}", console::style("═".repeat(65)).cyan().bright());
        println!("\n{}", result);
        println!("\n{}", console::style("═".repeat(65)).cyan().bright());
    }

    println!(
        "\n{} {}",
        console::style("🎯").green(),
        console::style("Mission accomplished! Have a great day! 🌟")
            .green()
            .bold()
    );

    Ok(())
}
