mod extract;
mod process;
mod store;
use std::fs;
use std::collections::HashMap;
use log::{info, error, debug};
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
    #[structopt(short="ignore", long)]
    ignore_errors: bool,

    /// Prefetch files only (do not zip files, only save to cache)
    #[structopt(short="pre", long)]
    prefetch: bool,

    /// Pack files only (zip cached files, do not download new files)
    #[structopt(short="p", long)]
    pack: bool,

    /// Output folder for downloaded files (Default: input file's parent folder)
    #[structopt(short="o", long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// TTS Cached files path (Default: %USERPROFILE%\Documents\My Games\Tabletop Simulator)
    #[structopt(short="c", long, parse(from_os_str))]
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
    let mut cache_path = format!("{}/Documents/My Games/Tabletop Simulator", user);
    if let Some(cache) = &args.cache {
        cache_path = match cache.to_str() {
            Some(path) => path.to_string(),
            None => {
                error!("Invalid cache path: {}", cache.display());
                return Ok(());
            }
        };
    }

    // Strip trailing slashes
    cache_path = cache_path.trim_end_matches('/').trim_end_matches('\\').to_string();

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

    let cached_files: HashMap<String, String> = fs::read_dir(&audio_path)?
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
            let path_str = path.to_str()?;
            let filename = path.file_stem()?.to_string_lossy().to_string(); // remove extension from filename to help with comparison from url alone
            Some((filename, path_str.to_string()))
        })
        .collect::<HashMap<_, _>>();

    for file in args.files {
        let file = fs::canonicalize(file)?; // get full path
        let (successful_files, failed_files) = match process::process_tts_save(file.to_str().unwrap(), &cached_files, &cache_path, args.ignore_errors, args.dry_run, args.pack).await {
            Ok((successful_files, failed_files)) => (successful_files, failed_files),
            Err(err) => {
                error!("Error processing input file {}: {}", file.display(), err);
                continue
            }
        };

        info!("Preprocessing completed. {} files processed, {} files failed.", successful_files.len(), failed_files.len());
        if args.dry_run {
            info!("Dry run completed. No files downloaded.");
            continue;
        }
        if args.prefetch {
            info!("Prefetch completed. Files saved to cache.");
            continue;
        }

        let output_folder = match &args.output {
            Some(output) => output.to_str().unwrap().to_string(),
            None => {
                let parent_folder = file.parent().unwrap().to_str().unwrap();
                parent_folder.to_string()
            }
        };

        let output_path = format!("{}/{}.zip", output_folder, file.file_stem().unwrap().to_str().unwrap());

        store::pack_files(successful_files, &cache_path, &output_path).await?;
        info!("Completed backing up archive to: {}", output_path);
    }

    Ok(())
}