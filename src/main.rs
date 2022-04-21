use std::io::{Write, Read};
use std::fs::{File, OpenOptions};

use image::{self, DynamicImage};
use image::imageops::FilterType;
use zip::ZipWriter;
use zip::write::FileOptions;

mod pack_creator;

const IMG_IN: [(&str, [[u32;2];2]);2] = [
    ("32", [[ 6u32,  8u32], [22u32, 24u32]]),
    ("64", [[12u32, 15u32], [44u32, 47u32]])
];

const THUMBNAIL_BG: &str = "./assets/thumbnail_background.png";

const PREVIEW_SIZE: u32 = 430;
const SPACING: u32 = 100;

fn main() {
    let mut preview_vector = Vec::new();
    
    for input in IMG_IN {
        let res = input.0;

        println!("Opening {}x version...", res);
        let mut img = image::open(format!("./assets/{}.png", res)).unwrap();
    
        for bundle in input.1 {
            for (index, y) in bundle.into_iter().enumerate() {
                println!("Changing bundle {}", index);

                let y_to = y + 1 - (index as u32) * 2;
    
                let line = img.crop_imm(0, y, img.width(), 1);
                image::imageops::overlay(&mut img, &line, 0, y_to as i64);
            }
        }
    
        // original
        println!("Saving {}x file...", res);
        let imgpath = format!("./out/{}.png", res);
        img.save(&imgpath).unwrap();
        // preview
        println!("Creating {}x preview...", res);
        let preview = img.resize(
            PREVIEW_SIZE, 
            PREVIEW_SIZE,
            FilterType::Nearest
        );
        
        preview_vector.push(preview);

        //* encode image
        let mut buf = Vec::new();
        let mut f = OpenOptions::new().read(true).open(&imgpath)
            .unwrap();
        f.read_to_end(&mut buf)
            .unwrap();
        let slice = buf.as_slice();

        pack_creator::create_packs(
            format!("./out/unpacked-hay-block-{}", res).as_str(),
            "Unpacked hay blocks freed by TheRolf",
            |w: &mut ZipWriter<File>,path: &str | -> Result<(), zip::result::ZipError> {
                w.start_file(path, FileOptions::default())?;
                w.write(slice)?;
                w.start_file("pack.png", FileOptions::default())?;
                w.write(slice)?;
                Ok(())
            }
        ).unwrap();
    }

    let mut thumb = image::open(THUMBNAIL_BG).unwrap();
    let thumb_width = thumb.width();
    let thumb_height = thumb.height();

    // align items
    let total_content_width = (PREVIEW_SIZE  + SPACING) * preview_vector.len() as u32 - SPACING;
    let mut content_x = (thumb_width - total_content_width) / 2;
    let content_y = (thumb_height - PREVIEW_SIZE) / 2;

    for (index, preview) in preview_vector.into_iter().enumerate() {
        println!("Copying {}x preview onto thumbnail...", IMG_IN[index].0);

        image::imageops::overlay(&mut thumb, &preview, i64::from(content_x), i64::from(content_y));
        content_x += PREVIEW_SIZE  + SPACING;
    }

    println!("Saving thumbnail...");
    thumb.save("./out/thumbnail.png").unwrap();
}
