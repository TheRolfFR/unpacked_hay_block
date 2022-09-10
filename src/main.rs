use std::fs;

use image::{self, DynamicImage};

mod minecraft;  
use minecraft::texture::texture::*;
use minecraft::pack::{Pack};

// Add-on thumbnail
mod thumbnail;
use thumbnail::*;

use crate::constants::*;

mod constants;

fn main() {
    println!("STEP 0 : Create output directory");
    fs::create_dir_all(OUT_DIR).unwrap();

    println!("STEP 1 : Manipulate images");
    let textures = Texture::open(&TEXTURE_DEF).unwrap().manipulate(|img: &mut DynamicImage| {
        let bundle_positions = match img.width() == 32 {
            true => vec![
                (8u32, 7u32), (24u32, 23u32)
            ],
            false => vec![
                (12u32, 13u32),(15u32, 14u32),
                (44u32, 45u32),(47u32, 46u32)
            ]
        };

        for (y, y_to) in bundle_positions {
            let line = img.crop_imm(0, y, img.width(), 1);
            image::imageops::overlay(img, &line, 0, y_to as i64);
        }
    });
    textures.save(OUT_DIR).unwrap();

    println!("STEP 2 : Get bigger previews");
    let previews: Vec<DynamicImage> = textures.get_previews(PREVIEW_SIZE, PREVIEW_SIZE);

    println!("STEP 3 : Make thumbnail");
    Thumbnail::open(THUMBNAIL_BG).unwrap()
        .paste(&previews)
        .save(&(OUT_DIR.to_string() + OUT_THUMBNAIL_NAME)).unwrap();

    println!("STEP 4 : Create packs with matrix");
    Pack::new(PACK_DEF)
        .matrix_from(FIRST_VERSION, RESOLUTIONS.as_slice()).unwrap()
        .add_texture(&TEXTURE_DEF)
        .generate(|versions: &str, res: &u32| -> (String, String) {
            (
                String::from(OUT_DIR) + PACK_NAME_PREFIX + &res.to_string() + "-" + versions + ".zip",
                String::from(OUT_DIR) + &res.to_string() + ".png"
            )
        }).unwrap();
}
