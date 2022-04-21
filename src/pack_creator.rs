use zip;
use zip::result::ZipResult;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use zip::write::{FileOptions, ZipWriter};

#[path = "mc_version.rs"]
mod minecraft_version;
use minecraft_version::version_to_str;

const PATHS: [([u8; 4], &str); 2] = [
  ([1, 1, 2, 3],   "assets/minecraft/textures/blocks/hay_block_side.png"),
  ([4, 5, 6, 7], "assets/minecraft/textures/block/hay_block_side.png")
];

fn path_from_format(format: u8) -> &'static str {
  for duo in PATHS {
    if duo.0.contains(&format) {
      return duo.1;
    }
  }

  return "";
}

pub fn create_packs<F>(out_file: &str, pack_desc: &str, func: F)-> ZipResult<()>
  where F: Fn(&mut ZipWriter<File>, &str) -> ZipResult<()> {
  for path in PATHS {
    for format in path.0 {
      let filepath = path_from_format(format);

      let versions = version_to_str(format);
      let zip_name = format!("{}-{}.zip", out_file, versions);
      
      let path = Path::new(&zip_name);

      let file = File::create(&path).unwrap();
      let mut writer = ZipWriter::new(file);

      writer.set_comment("comment");
      writer.start_file("pack.mcmeta", FileOptions::default())?;
      writer.write(format!(r#"{{
  "pack": {{
    "pack_format": {},
    "description": "{}"
  }}
}}"#, format, pack_desc).as_bytes())?;

      func(&mut writer, filepath)?;

      writer.finish()?;
    }
  }

  Ok(())
}