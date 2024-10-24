mod extract;
mod process;
use std::fs;
use std::collections::HashSet;
use log::{info, error};
use log4rs::{config::RawConfig, init_raw_config};
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "tts-archiver")]
pub struct Args {
    #[structopt(short="v", long)]
    verbose: bool,

    /// Dry run. Only does a head request to check if file exists and does not download, processing is still simulated.
    #[structopt(short="dry", long)]
    dry_run: bool,

    /// Ignore errors (continue processing even if errors occur)
    #[structopt(short="i", long)]
    ignore_errors: bool,

    /// Output folder for downloaded files (Default: input file's parent folder)
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// TTS Cached files path (Default: %USERPROFILE%\Documents\My Games\Tabletop Simulator)
    #[structopt(short, long, parse(from_os_str))]
    cache: Option<PathBuf>,

    /// Tabletop Simulator save/workshop JSON file/s to process
    #[structopt(name = "TTS_SAVE_JSON(s)", parse(from_os_str))]
    files: Vec<PathBuf>,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log4rs_config = include_str!("log4rs.yml");
    let raw_config: RawConfig = serde_yml::from_str(log4rs_config)?;
    init_raw_config(raw_config)?;

    info!("Starting up");

    let args = Args::from_args();

    // set base cache path to {USERPROFILE}\Documents\My Games\Tabletop Simulator
    let user = std::env::var("USERPROFILE")?;
    let mut cache_path = format!("{}/Documents/My Games/Tabletop Simulator/", user);
    if let Some(cache) = &args.cache {
        cache_path = match cache.to_str() {
            Some(path) => path.to_string(),
            None => {
                error!("Invalid cache path: {}", cache.display());
                return Ok(());
            }
        };
    }

    if !PathBuf::from(&cache_path).exists() {
        error!("Cache path does not exist: {}", cache_path);
        return Ok(());
    }

    let audio_path = &format!("{}/Mods/Audio/", cache_path);
    let image_path = &format!("{}/Mods/Images/", cache_path);
    let pdf_path = &format!("{}/Mods/PDF/", cache_path);
    let model_path = &format!("{}/Mods/Models/", cache_path);
    let workshop_path = &format!("{}/Mods/Workshop/", cache_path);
    let image_raw_path = &format!("{}/Mods/Images Raw/", cache_path);
    let model_raw_path = &format!("{}/Mods/Models Raw/", cache_path);
    let assetbundles_path = &format!("{}/Mods/Assetbundles/", cache_path);
    let text_path = &format!("{}/Mods/Text/", cache_path);

    // Create folders if they don't exist
    fs::create_dir_all(&audio_path)?;
    fs::create_dir_all(&image_path)?;
    fs::create_dir_all(&pdf_path)?;
    fs::create_dir_all(&model_path)?;
    fs::create_dir_all(&workshop_path)?;
    fs::create_dir_all(&image_raw_path)?;
    fs::create_dir_all(&model_raw_path)?;
    fs::create_dir_all(&assetbundles_path)?;
    fs::create_dir_all(&text_path)?;

    let cached_files = fs::read_dir(&audio_path)?
        .chain(fs::read_dir(&image_path)?)
        .chain(fs::read_dir(&pdf_path)?)
        .chain(fs::read_dir(&workshop_path)?)
        .chain(fs::read_dir(&model_path)?)
        .chain(fs::read_dir(&image_raw_path)?)
        .chain(fs::read_dir(&model_raw_path)?)
        .chain(fs::read_dir(&assetbundles_path)?)
        .chain(fs::read_dir(&text_path)?)
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let filename = path.file_stem()?.to_string_lossy().to_string(); // remove extension from filename to help with comparison from url alone
            Some(filename)
        })
        .collect::<HashSet<_>>();

    for file in args.files {
        process::process_tts_save(file.to_str().unwrap(), &cached_files, &cache_path, args.ignore_errors, args.dry_run).await?;
    }

    Ok(())
}