use std::path::Path;

use image::{DynamicImage, ImageError, ImageResult, imageops::FilterType};

#[derive(Debug)]
pub struct TextureDefinition<'a> {
  pub name: &'a str,
  pub paths: &'a[(&'a str, &'a str)],
  pub version_paths: &'a[(&'a str, &'a str)],
  pub id: i32
}

#[derive(Debug, Default, Clone)]
pub struct Texture {
  name: String,
  images: Vec<(DynamicImage, String)>
}

impl Texture {
  pub fn open(def: &TextureDefinition) -> Result<Self, ImageError> {
    let mut res = Self::default();

    res.name = def.name.to_string();

    for (path,_) in def.paths.iter() {
      res.images.push((image::open(*path)?, path.to_string()));
    }

    Ok(res)
  }

  pub fn images(&self) -> &Vec<(DynamicImage, String)>{
    return self.images.as_ref();
  }

  pub fn manipulate<F>(&self, func: F) -> Self where F: Fn(&mut DynamicImage) -> () {
    let mut result = self.clone();

    for (image,_) in result.images.iter_mut() {
      func(image);
    }

    result
  }

  pub fn save(&self, out_dir: &str) -> ImageResult<()> {
    for (image, path) in self.images.iter() {
      image.save(String::from(out_dir) + Path::new(path.as_str()).file_name().unwrap().to_str().unwrap())?;
    }
    Ok(())
  }

  pub fn get_previews(self, preview_width: u32, preview_height: u32) -> Vec<DynamicImage> {
    self.images.iter()
      .map(|(e,_)| e.resize(
        preview_width, 
        preview_height,
        FilterType::Nearest
    )).collect()
  }
}