use std::path::PathBuf;

use clap::Parser;
use clap_stdin::FileOrStdin;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Optional footer label for the QR code.")]
    pub label: Option<String>,

    #[arg(
        short,
        long,
        default_value = "40",
        help = "Optional font size for the label."
    )]
    pub font_size: f32,

    #[arg(
        short,
        long,
        help = "Write the armored content to the file at path AGE_OUTPUT."
    )]
    pub age_output: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Encrypt to the specified RECIPIENT. May be repeated."
    )]
    pub recipient: Vec<String>,

    #[arg(
        short = 'R',
        long,
        help = "Encrypt to the recipients listed at RECIPIENT_FILE. May be repeated."
    )]
    pub recipient_file: Vec<PathBuf>,

    #[arg(long, default_value = "rage", help = "Overwrite the age binary path.")]
    pub age_bin: String,

    #[arg(help = "Write the result QR code to file at path OUTPUT. Pass - for stdout")]
    pub output: PathBuf,

    #[arg(
        default_value = "-",
        help = "Path to a file to read from. Defaults to stdin"
    )]
    pub input: FileOrStdin,
}
