use image::{DynamicImage, ImageError};

#[derive(Debug)]
pub struct Thumbnail {
  im: DynamicImage
}

impl Thumbnail {
  pub fn open(bg: &str) -> Result<Self, ImageError> {
    Ok(Self {
      im: image::open(bg)?
    })
  }
  pub fn paste(&self, images: &[DynamicImage]) -> Self {
    let mut thumb = self.im.clone();
    
    let thumb_width = thumb.width();
    let thumb_height = thumb.height();

    let content_width = images.iter().fold(0, |acc, cur| acc + cur.width());

    // align items
    let spacing: i64 = ((thumb_width - content_width) / (images.len() + 1) as u32) as i64;

    let mut content_x: i64 = spacing;
    for preview in images.iter() {
      let content_y: i64 = (thumb_height - preview.height()) as i64 / 2;
      
      image::imageops::overlay(&mut thumb, preview, i64::from(content_x), i64::from(content_y));

      content_x += preview.width() as i64 + spacing;
    }

    Self {
      im: thumb
    }
  }

  pub fn save(&self, path: &str) -> Result<(), ImageError> {
    self.im.save(path)?;
    Ok(())
  }
}