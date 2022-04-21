pub const FORMATS: [(&str, u8);7] = [
  ("1.6.1-1.8.9",   1),
  ("1.9-1.10.2",    2),
  ("1.11-1.12.2",   3),
  ("1.13-1.14.4",   4),
  ("1.15-1.16.1",   5),
  ("1.16.2-1.16.4", 6),
  ("1.17+",         7)
];

pub fn version_to_str(pack_format: u8) -> &'static str {
  for format in FORMATS {
    if format.1 == pack_format {
      return format.0;
    }
  }
  ""
}