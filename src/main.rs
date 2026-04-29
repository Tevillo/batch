use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Batch rename files in the current directory to <pattern>-<N> with padded numbering."
)]
struct Args {
    /// New name prefix; files become <pattern>-<N>[.<ext>].
    #[arg(short, long)]
    pattern: String,

    /// Print planned renames without modifying anything.
    #[arg(short = 'n', long)]
    dry_run: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut files: Vec<PathBuf> = fs::read_dir(".")?
        .filter_map(|res| res.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect();

    if files.is_empty() {
        println!("No files to rename.");
        return Ok(());
    }

    files.sort_by(|a, b| {
        natord::compare(
            a.file_name().unwrap_or_default().to_string_lossy().as_ref(),
            b.file_name().unwrap_or_default().to_string_lossy().as_ref(),
        )
    });

    let width = files.len().to_string().len();
    let prefix = &args.pattern;

    for (i, src) in files.iter().enumerate() {
        let n = i + 1;
        let ext = match src.extension() {
            Some(ext) => ext.to_str().unwrap_or_default(),
            _ => "",
        };
        let new_name = format!("{prefix}-{n:0width$}.{ext}");
        if args.dry_run {
            println!("{:?} -> {new_name}", src);
        } else {
            let _ = std::fs::rename(src, new_name);
        }
    }

    println!("Done.");
    Ok(())
}
