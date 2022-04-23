use std::cmp::Ordering;

const MIN: &str = "1.6.1";
const MAX: &str = "1.19";

const FORMATS: [(&str, u8);7] = [
  ("1.6.1-1.8.9",   1),
  ("1.9-1.10.2",    2),
  ("1.11-1.12.2",   3),
  ("1.13-1.14.4",   4),
  ("1.15-1.16.1",   5),
  ("1.16.2-1.16.4", 6),
  ("1.17-1.19",     7)
];

#[derive(Debug)]
pub struct MinecraftVersion(String);

impl PartialEq for MinecraftVersion {
  fn eq(&self, other: &Self) -> bool   {
    self.partial_cmp(other) == Some(Ordering::Equal)
  }
}
impl Eq for MinecraftVersion {}

impl PartialOrd for MinecraftVersion {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let self_arr = self.to_array();
    let other_arr = other.to_array();

    let mut ordering = Ordering::Equal;
    let mut i = 0usize;

    while ordering == Ordering::Equal && i < 3 {
      ordering = self_arr[i].partial_cmp(&other_arr[i]).unwrap();
      i+= 1;
    }

    Some(ordering)
  }
}

impl Ord for MinecraftVersion {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl MinecraftVersion {
  pub fn min_version() -> MinecraftVersion { MinecraftVersion::new(MIN) }
  pub fn max_version() -> MinecraftVersion { MinecraftVersion::new(MAX) }

  pub fn new(ver: &str) -> Self {
    Self(ver.to_string())
  }

  pub fn to_array(&self) -> [u8; 3] {
    self.0.split('.').into_iter()
      .map(|e| e.parse().unwrap())
      .enumerate().fold([0u8; 3], |mut acc, (i, e)| {
        acc[i] = e;
        acc
      })
  }

  pub fn pack_version(self) -> Option<u8> {
    if self < Self::min_version() || self > Self::max_version() {
      return None;
    }

    let mut result: Option<u8> = None;

    let mut i: usize = 0;

    while result.is_none() && i < FORMATS.len() {
      let format = FORMATS[i];
      let inside = format.0.split('-')
        .map(MinecraftVersion::new)
        .enumerate()
        .all(|(i, e)| match i {
          0 => self >= e,
          1 => self <= e,
          _ => panic!("More than two minecraft versions detected")
        });

      if inside {
        result = Some(format.1);
      }

      i+= 1;
    }

    result
  }
  
  pub fn version_to_str(pack_format: u8) -> &'static str {
    for format in FORMATS {
      if format.1 == pack_format {
        return format.0;
      }
    }
    ""
  }
}