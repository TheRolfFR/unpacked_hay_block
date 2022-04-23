use std::fs;

use image::{self, DynamicImage};

mod minecraft;  
use minecraft::texture::texture::*;
use minecraft::pack::{Pack, PackDefinition};

// Add-on thumbnail
mod thumbnail;
use thumbnail::*;

const FIRST_VERSION: &str = "1.6.1";
const RESOLUTIONS: [u32; 2] = [32, 64];

const PACK_NAME_PREFIX: &str = "unpacked-hay-block-";

const TEXTURE_DEF: TextureDefinition = TextureDefinition {
    name: "hay_block",
    paths: [
        ("./assets/32.png", "./out/32.png"),
        ("./assets/64.png", "./out/64.png")
    ].as_slice(),
    version_paths: [
        (FIRST_VERSION, "assets/minecraft/textures/blocks/hay_block_side.png"),
        ("1.13", "assets/minecraft/textures/block/hay_block_side.png")
    ].as_slice()
};

const THUMBNAIL_BG: &str = "./assets/thumbnail_background.png";
const PREVIEW_SIZE: u32 = 430u32;
const OUT_DIR: &str = "./out/";
const OUT_THUMBNAIL_NAME: &str = "thumbnail.png";

const PACK_DEF: &PackDefinition = &PackDefinition {
    comment: "Pack by TheRolf",
    description: "Free your hay bales! 32x and 64x resource pack add-on"
};

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
