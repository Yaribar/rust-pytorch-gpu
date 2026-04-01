use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    author = "Yaribar",
    about = "A Stable Diffusion image generator using diffusers-rs"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Generate {
        #[arg(short, long)]
        prompt: String,
        #[arg(short, long, default_value = "sd_output.png")]
        output: String,
        #[arg(short, long, default_value_t = 20)]
        steps: usize,
        #[arg(short, long, default_value = "data")]
        data_dir: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Generate {
            prompt,
            output,
            steps,
            data_dir,
        }) => {
            diffusers_rs::generate_image(&prompt, &output, steps, &data_dir)?;
        }
        None => {
            println!("No command given. Use --help for usage information.");
        }
    }
    Ok(())
}
