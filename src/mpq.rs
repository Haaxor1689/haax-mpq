use ignore::WalkBuilder;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;
use std::{collections::HashMap, path::Path};

use stormlib::{
    Archive, CompressionFlags, CreateArchiveFlags, CreateFileFlags, CreateFileOptions,
    OpenArchiveFlags,
};

const DEFAULT_PATCHIGNORE: &str = r#"
# ignore git related stuff
.git/**
.github/**
.gitignore

# ignore all file formats that have nothing to do in patch
**/*.json
**/*.yml
**/*.yaml
**/*.exe
**/*.dll
**/*.db
**/*.csv
**/*.png
**/*.psd
**/*.txt
**/*.md
**/*.sql

# ignore special mpq files
(listfile)
(attributes)
(signature)
(user data)

.patchignore
"#;

pub async fn build(path: &str, source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut walk_builder = WalkBuilder::new(source);

    let patchignore_path = PathBuf::from(source).join(".patchignore");

    if patchignore_path.exists() {
        walk_builder.add_custom_ignore_filename(".patchignore");
        log::info!("[RUST] build_mpq: Using .patchignore for ignore rules.");
    } else {
        walk_builder.add_ignore(DEFAULT_PATCHIGNORE);
        log::warn!("[RUST] build_mpq: .patchignore not found. Using .defaultignore as fallback.");
    }

    // Read all files from the source directory recursively into hashmap of paths and mtimes
    let mut files = HashMap::new();
    for entry in walk_builder
        .add_custom_ignore_filename(".patchignore")
        .build()
        .into_iter()
    {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                log::warn!("[RUST] build_mpq: Failed to read entry: {}", e);
                continue;
            }
        };

        if !entry.path().is_file() {
            continue;
        }

        // Get relative path
        let name = entry
            .path()
            .strip_prefix(source)?
            .to_str()
            .unwrap()
            .to_string();

        // Get file metadata
        let mtime = entry
            .metadata()?
            .modified()?
            .duration_since(UNIX_EPOCH)?
            .as_secs();

        files.insert(name, mtime);
    }

    let archive = match fs::exists(path) {
        Ok(true) => {
            log::info!("[RUST] build_mpq Opening archive \"{path}\"");
            let archive = match Archive::open(path, OpenArchiveFlags::empty()) {
                Ok(guard) => guard,
                Err(e) => {
                    log::error!("[RUST] build_mpq Failed to open archive {}: {}", path, e);
                    return Err(e.into());
                }
            };

            let has_listfile = match archive.open_file("(listfile)") {
                Ok(_) => true,
                Err(e) => {
                    log::error!("[RUST] build_mpq: Missing listfile in patch \"{path}\": {e}");
                    false
                }
            };

            let has_attributes = match archive.open_file("(attributes)") {
                Ok(_) => true,
                Err(e) => {
                    log::error!("[RUST] build_mpq: Missing attributes in patch \"{path}\": {e}");
                    false
                }
            };

            if !has_listfile || !has_attributes {
                drop(archive);
                fs::remove_file(path)?;
                return Ok(());
            }

            archive
        }
        _ => {
            log::info!("[RUST] build_mpq Creating archive \"{path}\"");
            match Archive::create(
                path,
                CreateArchiveFlags::MPQ_CREATE_ARCHIVE_V1
                    | CreateArchiveFlags::MPQ_CREATE_LISTFILE
                    | CreateArchiveFlags::MPQ_CREATE_ATTRIBUTES,
                files.len() as u32,
            ) {
                Ok(guard) => guard,
                Err(e) => {
                    log::error!("[RUST] build_mpq Failed to create archive \"{path}\": {e}");
                    return Err(e.into());
                }
            }
        }
    };

    // Iterate over all files in the archive
    for search_result in archive.search(None)? {
        let name = search_result
            .cFileName
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as u8 as char)
            .collect::<String>();

        if name.eq("(listfile)") || name.eq("(attributes)") || name.eq("(signature)") {
            continue;
        }

        let file_mtime = match files.remove(&name) {
            Some(file_mtime) => file_mtime,
            None => {
                log::info!("[RUST] build_mpq Removing file from archive: {name}");
                archive.remove_file(&name)?;
                continue;
            }
        };

        let archive_mtime =
            ((search_result.dwFileTimeHi as u64) << 32) | (search_result.dwFileTimeLo as u64);

        if file_mtime == archive_mtime {
            log::info!("[RUST] build_mpq Skipping file (mtime match): {name}",);
            continue;
        }

        log::info!("[RUST] build_mpq Updating file in archive: {name}",);
        let mut buffer = Vec::new();
        File::open(&Path::new(source).join(&name))?.read(&mut buffer)?;

        archive.remove_file(&name)?;
        archive.create_file(CreateFileOptions {
            path: &name,
            data: &buffer,
            mtime: file_mtime,
            flags: CreateFileFlags::MPQ_FILE_COMPRESS,
            compression: CompressionFlags::MPQ_COMPRESSION_ZLIB,
        })?;
    }

    // Iterate over remaining files in the source directory
    for (name, file_mtime) in files {
        log::info!("[RUST] build_mpq Adding new file to archive: {name}",);
        let mut buffer = Vec::new();
        File::open(Path::new(source).join(&name))?.read_to_end(&mut buffer)?;

        archive.create_file(CreateFileOptions {
            path: &name,
            data: &buffer,
            mtime: file_mtime,
            flags: CreateFileFlags::MPQ_FILE_COMPRESS,
            compression: CompressionFlags::MPQ_COMPRESSION_ZLIB,
        })?;
    }

    archive.compact()?;
    drop(archive);

    Ok(())
}

pub async fn extract(path: &str, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let archive = match fs::exists(path) {
        Ok(true) => {
            log::info!("[RUST] extract_mpq Opening archive \"{path}\"");
            match Archive::open(path, OpenArchiveFlags::empty()) {
                Ok(guard) => guard,
                Err(e) => {
                    log::error!("[RUST] extract_mpq Failed to open archive {}: {}", path, e);
                    return Err(e.into());
                }
            }
        }
        _ => {
            log::warn!("[RUST] extract_mpq Missing archive \"{path}\"");
            return Ok(());
        }
    };

    // Iterate over all files in the archive
    for search_result in archive.search(None)? {
        let name = search_result
            .cFileName
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as u8 as char)
            .collect::<String>();

        if name.eq("(listfile)") || name.eq("(attributes)") || name.eq("(signature)") {
            continue;
        }

        // Read file content
        let mut file = match archive.open_file(name.as_str()) {
            Ok(file) => file,
            Err(e) => {
                log::error!(
                    "[RUST] extract_mpq: Failed to open \"{name}\" in patch \"{path}\": {e}"
                );
                continue;
            }
        };

        // Write buffer to file
        let target_path = Path::new(target).join(&name);
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&target_path, file.read_all()?)?;
        log::info!(
            "[RUST] extract_mpq Extracted file \"{name}\" to \"{}\"",
            target_path.display()
        );
    }

    drop(archive);

    Ok(())
}
