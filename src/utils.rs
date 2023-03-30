use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use image::{GenericImageView, ImageError, imageops::FilterType};


pub fn list_files(folder_path: &str) {
    let path = Path::new(folder_path);
    if !path.is_dir() {
        eprintln!("Error: '{}' is not a directory.", folder_path);
        return;
    }

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let ext = entry.path().extension().and_then(|s| s.to_str());
            match ext {
                Some("png") | Some("jpg") | Some("webp") => handle_image(&entry),
                Some("mp4") | Some("mkv") | Some("avi") | Some("webm") => handle_video(&entry),
                Some("mp3") | Some("wav") | Some("flac") | Some("ogg") => handle_audio(&entry),
                _ => println!("Other: {}", entry.path().display()),
            }
        }
    }
}

// ** Image
pub fn handle_image(entry: &DirEntry) {
    match resize_image(entry.path()) {
        Ok(_) => println!("Resized: {}", entry.path().display()),
        Err(e) => eprintln!("Error resizing {}: {}", entry.path().display(), e),
    }
}

pub fn resize_image(path: &Path) -> Result<(), ImageError> {
    let img = image::open(path)?;
    let (width, height) = img.dimensions();
    let aspect_ratio = width as f64 / height as f64;
    let sizes = [800, 1280, 1400, 1920];

    let max_size = *sizes
        .iter()
        .filter(|&&size| size < std::cmp::max(width, height))
        .max()
        .unwrap_or(&800);

    if max_size >= std::cmp::max(width, height) {
        return Ok(()); // Do not resize if the image is smaller than the smallest size.
    }

    let (new_width, new_height) = if width > height {
        (max_size, (max_size as f64 / aspect_ratio).round() as u32)
    } else {
        ((max_size as f64 * aspect_ratio).round() as u32, max_size)
    };

    let resized_image = img.resize(new_width, new_height, FilterType::Lanczos3);
    let mut output_path = PathBuf::from(path);
    let file_stem = output_path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    output_path.set_file_name(format!("{}-resized", file_stem));
    output_path.set_extension(path.extension().unwrap_or_default());
    resized_image.save(output_path)?;
    Ok(())
}


// ** Video
pub fn handle_video(entry: &DirEntry) {
    println!("Video: {}", entry.path().display());
}

// ** Audio
pub fn handle_audio(entry: &DirEntry) {
    println!("Audio: {}", entry.path().display());
}