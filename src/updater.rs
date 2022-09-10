use std::{io::Write, fs};

use image::DynamicImage;
use tempdir::TempDir;
use reqwest;

use unpacked_hay_block::{
    constants::{RESOLUTIONS_PACKS, TEXTURE_DEF},
    minecraft::texture::texture::Texture
};

fn main() -> Result<(), std::io::Error> {
    let arg_list: Vec<String> = std::env::args().collect();
    let force = arg_list.get(1)
        .map(|f| { f == "-f" || f == "--force" })
        .unwrap_or(false);

    if force {
        println!("Forced update with force flag");
    }

    print!("STEP 0: Create temp directory... ");
    std::io::stdout().flush()?;
    let tmp_dir = TempDir::new(env!("CARGO_PKG_NAME")).unwrap().into_path();

    print!("STEP 1: Loading texture... ");
    std::io::stdout().flush()?;
    let texture = Texture::open(&TEXTURE_DEF).unwrap();
    let texture_images = texture.images();
    println!("Success");

    println!("STEP 2: Loading online textures... ");
    let mut modify_list: Vec<(DynamicImage, &str)> = vec!();
    for (pack_index, pack) in RESOLUTIONS_PACKS.into_iter().enumerate() {
        let url = format!("https://api.faithfulpack.net/v2/textures/{texture_id}/url/{pack}/latest", texture_id=TEXTURE_DEF.id);

        println!("Fetching {url}...");

        let img_bytes = reqwest::blocking::get(url).unwrap()
            .bytes().unwrap();

        let online_image = image::load_from_memory(&img_bytes).unwrap();
        let current_image_def = texture_images.get(pack_index).unwrap();
        let current_image = &current_image_def.0;

        if force || online_image != *current_image {
            modify_list.push((online_image, &current_image_def.1));
        }
    }

    if modify_list.len() == 0 {
        println!("Nothing to update.");
        return Ok(());
    }

    println!("STEP 4 : Saving at {}...", tmp_dir.to_str().unwrap());
    for (image, relative_path) in modify_list {
        let final_tmp_path = tmp_dir.join(std::path::Path::new(relative_path));
        println!("Saving at {}...", final_tmp_path.display());

        // folder
        let mut final_tmp_folder = final_tmp_path.clone();
        final_tmp_folder.pop();
        fs::create_dir_all(final_tmp_folder).unwrap();
        // file
        image.save(final_tmp_path).unwrap();
    }

    // save output file
    
    println!("STEP 5 : Saving new asset file...");
    let data = tmp_dir.to_str().unwrap();
    fs::write("./out/new_assets_folder", data).expect("Failed to write update file");

    println!("done");

    Ok(())
}