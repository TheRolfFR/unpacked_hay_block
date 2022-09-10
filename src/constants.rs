
use crate::minecraft::texture::texture::*;
use crate::minecraft::pack::PackDefinition;

pub const FIRST_VERSION: &str = "1.6.1";
pub const RESOLUTIONS: [u32; 2] = [32, 64];
pub const RESOLUTIONS_PACKS: [&str; 2] = ["faithful_32x", "faithful_64x"];

pub const PACK_NAME_PREFIX: &str = "unpacked-hay-block-";

pub const TEXTURE_DEF: TextureDefinition = TextureDefinition {
    name: "hay_block",
    paths: [
        ("./assets/32.png", "./out/32.png"),
        ("./assets/64.png", "./out/64.png")
    ].as_slice(),
    version_paths: [
        (FIRST_VERSION, "assets/minecraft/textures/blocks/hay_block_side.png"),
        ("1.13", "assets/minecraft/textures/block/hay_block_side.png")
    ].as_slice(),
    id: 391
};

pub const THUMBNAIL_BG: &str = "./assets/thumbnail_background.png";
pub const PREVIEW_SIZE: u32 = 430u32;
pub const OUT_DIR: &str = "./out/";
pub const OUT_THUMBNAIL_NAME: &str = "thumbnail.png";

pub const PACK_DEF: &PackDefinition = &PackDefinition {
    comment: "Pack by TheRolf",
    description: "Free your hay bales! 32x and 64x resource pack add-on"
};