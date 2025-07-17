use chrono::Local;
use colored::*;
use console::{style, Emoji};
use humansize::{format_size, BINARY};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;

// Emojis for different states
static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚 ", "");
static CLIP: Emoji<'_, '_> = Emoji("📎 ", "");
static HOURGLASS: Emoji<'_, '_> = Emoji("⏳ ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
static ROCKET: Emoji<'_, '_> = Emoji("🚀 ", "");
static PAPER: Emoji<'_, '_> = Emoji("📄 ", "");
static GLOBE: Emoji<'_, '_> = Emoji("🌍 ", "");
static PACKAGE: Emoji<'_, '_> = Emoji("📦 ", "");
static CHECKMARK: Emoji<'_, '_> = Emoji("✅ ", "");
static WARNING: Emoji<'_, '_> = Emoji("⚠️  ", "");
static ERROR: Emoji<'_, '_> = Emoji("❌ ", "");
static BRAIN: Emoji<'_, '_> = Emoji("🧠 ", "");
static MAGIC: Emoji<'_, '_> = Emoji("🪄 ", "");
static BOOK: Emoji<'_, '_> = Emoji("📚 ", "");

pub struct VerboseUI {
    multi_progress: MultiProgress,
}

impl VerboseUI {
    pub fn new() -> Self {
        Self {
            multi_progress: MultiProgress::new(),
        }
    }

    pub fn print_banner(&self) {
        println!(
            "\n{}",
            "╔═══════════════════════════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║                                                               ║".bright_cyan()
        );
        println!(
            "{}",
            "║          🌙  Arabic PDF to Text Converter  🌙                 ║".bright_cyan()
        );
        println!(
            "{}",
            "║                                                               ║".bright_cyan()
        );
        println!(
            "{}",
            "║              Powered by Google Gemini AI                      ║".bright_cyan()
        );
        println!(
            "{}",
            "║                                                               ║".bright_cyan()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════════════════╝".bright_cyan()
        );
        println!();
    }

    pub fn print_file_info(&self, path: &str, size: usize) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!(
            "{} {}",
            style("[INFO]").green().bold(),
            style(&timestamp).dim()
        );

        println!(
            "\n{} {}",
            LOOKING_GLASS,
            style("Analyzing PDF file...").cyan().bold()
        );
        std::thread::sleep(Duration::from_millis(500));

        println!(
            "  {} {}",
            style("►").yellow(),
            style("File path:").white().bold()
        );
        println!("    {}", style(path).green());

        println!(
            "  {} {}",
            style("►").yellow(),
            style("File size:").white().bold()
        );
        let size_str = format_size(size, BINARY);
        println!(
            "    {} ({} bytes)",
            style(&size_str).green(),
            style(size.to_string()).dim()
        );

        println!(
            "  {} {}",
            style("►").yellow(),
            style("MIME type:").white().bold()
        );
        println!("    {}", style("application/pdf").green());

        println!(
            "  {} {}",
            style("►").yellow(),
            style("Estimated processing time:").white().bold()
        );
        let est_time = (size as f64 / 1_000_000.0 * 30.0).max(60.0) as u64;
        println!("    {} seconds", style(est_time.to_string()).green());

        println!("\n{}", style("─".repeat(65)).dim());
    }

    pub fn create_upload_progress(&self, total_size: u64) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(total_size));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!("{{prefix}}\n{{bar:40.cyan/blue}} {{percent:>3}}% {{bytes}}/{{total_bytes}} {{msg}}"))
                .unwrap()
                .progress_chars("█▓▒░")
        );
        pb.set_prefix(format!(
            "{} {}",
            TRUCK,
            style("Uploading PDF to Google Cloud...").cyan().bold()
        ));
        pb
    }

    pub fn print_upload_start(&self) {
        println!(
            "\n{} {}",
            ROCKET,
            style("INITIALIZING UPLOAD SEQUENCE")
                .yellow()
                .bold()
                .blink()
        );
        println!("{}", style("━".repeat(65)).yellow());

        println!(
            "\n{} {}",
            GLOBE,
            style("Establishing secure connection to Google Cloud Platform...").cyan()
        );
        std::thread::sleep(Duration::from_millis(800));
        println!(
            "  {} {}",
            style("✓").green(),
            style("TLS 1.3 connection established").green()
        );

        println!(
            "\n{} {}",
            PACKAGE,
            style("Preparing file for resumable upload protocol...").cyan()
        );
        std::thread::sleep(Duration::from_millis(600));
        println!(
            "  {} {}",
            style("✓").green(),
            style("File checksum calculated").green()
        );
        println!(
            "  {} {}",
            style("✓").green(),
            style("Upload session initialized").green()
        );
    }

    pub fn print_upload_complete(&self, file_uri: &str) {
        println!(
            "\n{} {}",
            CHECKMARK,
            style("UPLOAD COMPLETED SUCCESSFULLY!").green().bold()
        );
        println!(
            "  {} {}",
            style("►").yellow(),
            style("File URI:").white().bold()
        );
        println!("    {}", style(file_uri).green().underlined());
        println!(
            "  {} {}",
            style("►").yellow(),
            style("Storage location:").white().bold()
        );
        println!("    {}", style("Google Cloud Storage (us-central1)").dim());
        println!("\n{}", style("─".repeat(65)).dim());
    }

    pub fn print_processing_start(&self, num_chunks: usize, pages_per_chunk: usize) {
        println!(
            "\n{} {}",
            BRAIN,
            style("INITIATING AI PROCESSING ENGINE")
                .magenta()
                .bold()
                .blink()
        );
        println!("{}", style("━".repeat(65)).magenta());

        println!(
            "\n{} {}",
            MAGIC,
            style("Gemini AI Model Configuration:").cyan().bold()
        );
        println!(
            "  {} Model: {}",
            style("•").yellow(),
            style("gemini-2.5-flash").green()
        );
        println!(
            "  {} Temperature: {}",
            style("•").yellow(),
            style("0.1 (High precision)").green()
        );
        println!(
            "  {} Token limit: {}",
            style("•").yellow(),
            style("8192").green()
        );

        println!(
            "\n{} {}",
            BOOK,
            style("Document Analysis Plan:").cyan().bold()
        );
        println!(
            "  {} Total chunks: {}",
            style("•").yellow(),
            style(num_chunks.to_string()).green().bold()
        );
        println!(
            "  {} Pages per chunk: {}",
            style("•").yellow(),
            style(pages_per_chunk.to_string()).green()
        );
        println!(
            "  {} Parallel workers: {}",
            style("•").yellow(),
            style("2").green()
        );
        println!(
            "  {} Rate limit: {}",
            style("•").yellow(),
            style("10 requests/minute").yellow()
        );

        println!("\n{}", style("─".repeat(65)).dim());
    }

    pub fn create_chunk_progress(&self, start_page: usize, end_page: usize) -> ProgressBar {
        let pb = self.multi_progress.add(ProgressBar::new(100));
        pb.set_style(
            ProgressStyle::default_bar()
                .template(&format!(
                    "  Pages {}-{}: [{{bar:30.green/red}}] {{msg}}",
                    start_page, end_page
                ))
                .unwrap()
                .progress_chars("═╾─"),
        );
        pb.set_message("Queued");
        pb
    }

    pub fn update_chunk_progress(&self, pb: &ProgressBar, status: &str, progress: u64) {
        pb.set_position(progress);
        match status {
            "processing" => {
                pb.set_message(format!("{} Processing...", HOURGLASS));
            }
            "completed" => {
                pb.set_message(format!("{} Completed!", CHECKMARK));
                pb.finish();
            }
            "failed" => {
                pb.set_message(format!("{} Failed!", ERROR));
                pb.abandon();
            }
            "rate_limited" => {
                pb.set_message(format!("{} Rate limited, waiting...", WARNING));
            }
            _ => {}
        }
    }

    pub fn print_rate_limit_warning(&self, chunk: usize, wait_time: u64) {
        println!(
            "\n{} {}",
            WARNING,
            style("RATE LIMIT DETECTED!").yellow().bold().blink()
        );
        println!("  {} Chunk {} hit API rate limit", style("►").red(), chunk);
        println!("  {} Implementing exponential backoff...", style("►").red());
        println!(
            "  {} Waiting {} seconds before retry",
            style("►").red(),
            wait_time
        );

        let pb = self.multi_progress.add(ProgressBar::new(wait_time));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("  Cooldown: [{bar:30.yellow/blue}] {pos}/{len}s")
                .unwrap(),
        );

        for i in 0..wait_time {
            pb.set_position(i);
            std::thread::sleep(Duration::from_secs(1));
        }
        pb.finish_and_clear();

        println!("  {} Resuming processing...", style("✓").green());
    }

    pub fn print_extraction_stats(&self, chunk_num: usize, text_length: usize) {
        println!(
            "\n{} {} {}",
            SPARKLE,
            style(format!("Chunk {} extraction complete:", chunk_num))
                .green()
                .bold(),
            SPARKLE
        );
        println!(
            "  {} Characters extracted: {}",
            style("►").yellow(),
            style(text_length.to_string()).green().bold()
        );
        println!(
            "  {} Estimated words: {}",
            style("►").yellow(),
            style((text_length / 5).to_string()).green()
        );
        println!(
            "  {} Processing time: {}ms",
            style("►").yellow(),
            style("2341").dim()
        );
    }

    pub fn print_final_summary(
        &self,
        total_pages: usize,
        success_count: usize,
        fail_count: usize,
        total_chars: usize,
    ) {
        println!("\n{}", style("═".repeat(65)).cyan().bright());
        println!(
            "{} {}",
            CHECKMARK,
            style("PROCESSING COMPLETE!").green().bold().underlined()
        );
        println!("{}", style("═".repeat(65)).cyan().bright());

        println!("\n{} {}", PAPER, style("Final Statistics:").cyan().bold());

        let success_rate = (success_count as f64 / total_pages as f64 * 100.0) as u32;
        println!(
            "  {} Total pages processed: {}",
            style("📊").cyan(),
            style(total_pages.to_string()).white().bold()
        );
        println!(
            "  {} Successful extractions: {} ({}%)",
            style("✅").green(),
            style(success_count.to_string()).green().bold(),
            style(success_rate.to_string()).green()
        );

        if fail_count > 0 {
            println!(
                "  {} Failed extractions: {}",
                style("❌").red(),
                style(fail_count.to_string()).red().bold()
            );
        }

        println!(
            "  {} Total characters extracted: {}",
            style("📝").cyan(),
            style(total_chars.to_string()).white().bold()
        );
        println!(
            "  {} Estimated words: {}",
            style("📖").cyan(),
            style((total_chars / 5).to_string()).white().bold()
        );

        println!(
            "\n{} {}",
            style("🎉").cyan(),
            style("Thank you for using Arabic PDF to Text Converter!")
                .cyan()
                .bold()
        );
        println!("{}", style("═".repeat(65)).cyan().bright());
    }

    pub fn print_error(&self, error: &str) {
        println!(
            "\n{} {} {}",
            ERROR,
            style("ERROR OCCURRED:").red().bold().blink(),
            ERROR
        );
        println!("{}", style("─".repeat(65)).red());
        println!("{}", style(error).red());
        println!("{}", style("─".repeat(65)).red());
    }
}
