use std::{fs::{File, OpenOptions}, vec, io::{Write, Read}};

use zip::{result::ZipError, ZipWriter, write::FileOptions};

use super::texture::texture::TextureDefinition;
use super::version::MinecraftVersion;

#[derive(Debug, Clone)]
pub struct PackDefinition<'a> {
  pub comment: &'a str,
  pub description: &'a str
}

#[derive(Debug, Clone)]
pub struct PackMatrix<'a> {
  definition: &'a PackDefinition<'a>,
  pack_versions: Vec<u8>,
  resolutions: Vec<u32>,
  textures: Vec<&'a TextureDefinition<'a>>
}

#[inline]
fn get_file_bytes(path: &str) -> Vec<u8> {
  let mut buf = Vec::new();
  let mut f = OpenOptions::new().read(true).open(path)
      .unwrap();
  f.read_to_end(&mut buf)
      .unwrap();

  buf
}

impl<'a> PackMatrix<'a> {
  pub fn add_texture(&mut self, texture_path: &'a TextureDefinition) -> &mut Self {
    self.textures.push(texture_path);
    return self;
  }

  pub fn generate<F>(&self, func: F) -> Result<(), ZipError>
    where F: Fn(&str, &u32) -> (String, String) {
    
    let file_options = Default::default();

    for (res_i, resolution) in self.resolutions.iter().enumerate() {
      for pack_version in self.pack_versions.iter() {
        let (path, pack_png) = func(MinecraftVersion::version_to_str(pack_version.clone()), resolution);
        let file = File::create(&path).unwrap();
        let mut writer = ZipWriter::new(file);

        // pack textures
        for texture in self.textures.iter() {
          let bytes = get_file_bytes(texture.paths[res_i].1);
          let texture_bytes = bytes.as_slice();

          let path: &str = texture.version_paths.iter().filter(|(m_ver, _)| {
            MinecraftVersion::new(*m_ver).pack_version() <= Some(*pack_version)
          }).last().unwrap().1;
          writer.start_file(path, file_options)?;
          writer.write(texture_bytes)?;
        }

        // pack MCMETA
        writer.set_comment(self.definition.comment);
        writer.start_file("pack.mcmeta", file_options)?;
        writer.write(format!(r#"{{
    "pack": {{
      "pack_format": {},
      "description": "{}"
    }}
  }}"#, pack_version, self.definition.description).as_bytes())?;
        
        // pack PNG
        let bytes = get_file_bytes(&pack_png);
        let pack_image_bytes = bytes.as_slice();
        writer.start_file("pack.png", FileOptions::default())?;
        writer.write(pack_image_bytes)?;

        writer.finish()?;
      }
    }
    Ok(())
  }
}

pub struct Pack<'a> {
  def: &'a PackDefinition<'a>
}

impl<'a> Pack<'a> {
  pub fn new(def: &'a PackDefinition) -> Self {
    Self {
      def
    }
  }

  pub fn matrix_from(&self, start_version: &str, res: &[u32]) -> Result<PackMatrix, &str> {
    let from = MinecraftVersion::new(start_version).pack_version();
    let to = MinecraftVersion::max_version().pack_version();

    match (from, to) {
        (None, _) => Err("Could not parse from version"),
        (Some(_), None) => Err("Could not parse max version"),
        (Some(f), Some(t)) if f > t => Err("From version is higher than maximum version"),
        (Some(f), Some(t)) => Ok(PackMatrix {
          definition: &self.def,
          pack_versions: (f..=t).into_iter().collect(),
          resolutions: res.to_vec(),
          textures: vec![]
        })
    }
  }
}