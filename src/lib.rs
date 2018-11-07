//! # Byteunit
//!
//! Byteunit is Rust utilities to parse string byte size into bytes count, 
//! and format bytes count back to string.
//!
////////////////////////////////////////////////////////////////////////////////
extern crate regex;

use regex::Regex;

pub const B: usize = 1;
pub const KB: usize = 1_000;
pub const MB: usize = 1_000_000;
pub const GB: usize = 1_000_000_000;
pub const TB: usize = 1_000_000_000_000;
pub const PB: usize = 1_000_000_000_000_000;

pub const KIB: usize = 1_024;
pub const MIB: usize = 1_048_576;
pub const GIB: usize = 1_073_741_824;
pub const TIB: usize = 1_099_511_627_776;
pub const PIB: usize = 1_125_899_906_842_624;

#[derive(Debug,PartialEq)]
pub enum Unit {
    B,
    KB,
    MB,
    GB,
    TB,
    PB,
    KIB,
    MIB,
    GIB,
    TIB,
    PIB,
}

fn parse_size_unit<S: Into<String>>(s: S) -> Result<(f64, Unit), &'static str> {
    let str = s.into();
    let re = Regex::new(r"^(?i)(\d+(\.\d+)?) *((k|m|g|t|p|ki|mi|gi|ti|pi)?b)?$").unwrap();
    let captures = re.captures(&str);
    
    match captures {
        Some(res) => {
            let size = res[1].to_owned();
            let unit: String = match res.get(3) {
                Some(val) => val.as_str().to_owned().to_uppercase(),
                None => "B".to_owned(),
            };
            
            Ok((size.parse::<f64>().unwrap(), match &*unit {
                "B" => Unit::B,
                "KB" => Unit::KB,
                "MB" => Unit::MB,
                "GB" => Unit::GB,
                "TB" => Unit::TB,
                "PB" => Unit::PB,
                "KIB" => Unit::KIB,
                "MIB" => Unit::MIB,
                "GIB" => Unit::GIB,
                "TIB" => Unit::TIB,
                "PIB" => Unit::PIB,
                _ => Unit::B,
            }))
        }
        None => Err("Parse Error. Invalid byte format."),
    }
}

/// Parse given string to bytes size
///
/// # Examples  
///
/// ```
/// assert_eq!(byteunit::parse("123").unwrap(), 123);
/// assert_eq!(byteunit::parse("1.23 B").unwrap(), 1);
/// assert_eq!(byteunit::parse("1.23 KB").unwrap(), 1_230);
/// assert_eq!(byteunit::parse("1.23 MB").unwrap(), 1_230_000);
/// assert_eq!(byteunit::parse("1.23 GB").unwrap(), 1_230_000_000);
/// assert_eq!(byteunit::parse("1.23 TB").unwrap(), 1_230_000_000_000);
/// assert_eq!(byteunit::parse("1.23 PB").unwrap(), 1_230_000_000_000_000);
/// assert_eq!(byteunit::parse("1.23 KiB").unwrap(), 1_259);
/// assert_eq!(byteunit::parse("1.23 MiB").unwrap(), 1_289_748);
/// assert_eq!(byteunit::parse("1.23 GiB").unwrap(), 1_320_702_443);
/// assert_eq!(byteunit::parse("1.23 TiB").unwrap(), 1_352_399_302_164);
/// assert_eq!(byteunit::parse("1.23 PiB").unwrap(), 1_384_856_885_416_427);
/// ```
pub fn parse<S: Into<String>>(str: S) -> Result<usize, &'static str> {
    let parsed = parse_size_unit(str);

    match parsed {
        Ok(r) => {
            let value = r.0;
            let unit = r.1;
            
            let bytes = match unit {
                Unit::B => value * B as f64,
                Unit::KB => value * KB as f64,
                Unit::MB => value * MB as f64,
                Unit::GB => value * GB as f64,
                Unit::TB => value * TB as f64,
                Unit::PB => value * PB as f64,
                Unit::KIB => value * KIB as f64,
                Unit::MIB => value * MIB as f64,
                Unit::GIB => value * GIB as f64,
                Unit::TIB => value * TIB as f64,
                Unit::PIB => value * PIB as f64,
            };

            Ok(bytes as usize)
        },
        Err(msg) => Err(msg),
    }
}

/// Parse given string to specific byte unit
///
/// # Examples  
///
/// ```
/// let kb = byteunit::parse_to("123B", byteunit::Unit::KB).unwrap();
/// let mb = byteunit::parse_to("123B", byteunit::Unit::MB).unwrap();
/// 
/// assert_eq!(kb, 0.123);
/// assert_eq!(mb, 0.000123);
/// ```
pub fn parse_to<S: Into<String>>(str: S, result_unit: Unit) -> Result<f64, &'static str> {
    match parse(str) {
        Ok(bytes) => {
            let result = match result_unit {
                Unit::B => bytes as f64,
                Unit::KB => bytes as f64 / KB as f64,
                Unit::MB => bytes as f64 / MB as f64,
                Unit::GB => bytes as f64 / GB as f64,
                Unit::TB => bytes as f64 / TB as f64,
                Unit::PB => bytes as f64 / PB as f64,
                Unit::KIB => bytes as f64 / KIB as f64,
                Unit::MIB => bytes as f64 / MIB as f64,
                Unit::GIB => bytes as f64 / GIB as f64,
                Unit::TIB => bytes as f64 / TIB as f64,
                Unit::PIB => bytes as f64 / PIB as f64,
            };

            Ok(result)
        },
        Err(msg) => Err(msg),
    }
}

/// Format bytes to byte size string
///
/// # Examples
///
/// ```
/// assert_eq!(byteunit::format(123), "123 B");
/// assert_eq!(byteunit::format(1_230), "1.23 KB");
/// assert_eq!(byteunit::format(1_230_000), "1.23 MB");
/// assert_eq!(byteunit::format(1_230_000_000), "1.23 GB");
/// assert_eq!(byteunit::format(1_230_000_000_000), "1.23 TB");
/// assert_eq!(byteunit::format(1_230_000_000_000_000), "1.23 PB");
/// ```
pub fn format(bytes: usize) -> String {
    if bytes < KB {
        return format_to(bytes, Unit::B);
    }

    if bytes < MB {
        return format_to(bytes, Unit::KB);
    }

    if bytes < GB {
        return format_to(bytes, Unit::MB);
    }

    if bytes < TB {
        return format_to(bytes, Unit::GB);
    }

    if bytes < PB {
        return format_to(bytes, Unit::TB);
    }

    format_to(bytes, Unit::PB)
}

/// Format bytes to specific unit byte size string
///
/// # Examples
///
/// ```
/// assert_eq!(byteunit::format_to(1245, byteunit::Unit::KB), "1.25 KB");
/// assert_eq!(byteunit::format_to(1275, byteunit::Unit::KIB), "1.25 KiB");
/// assert_eq!(byteunit::format_to(500, byteunit::Unit::KB), "0.5 KB");
/// assert_eq!(byteunit::format_to(512, byteunit::Unit::KIB), "0.5 KiB");
/// ```
pub fn format_to(bytes: usize, unit: Unit) -> String {
    let result = match unit {
        Unit::B => bytes as f64,
        Unit::KB => bytes as f64 / KB as f64,
        Unit::MB => bytes as f64 / MB as f64,
        Unit::GB => bytes as f64 / GB as f64,
        Unit::TB => bytes as f64 / TB as f64,
        Unit::PB => bytes as f64 / PB as f64,
        Unit::KIB => bytes as f64 / KIB as f64,
        Unit::MIB => bytes as f64 / MIB as f64,
        Unit::GIB => bytes as f64 / GIB as f64,
        Unit::TIB => bytes as f64 / TIB as f64,
        Unit::PIB => bytes as f64 / PIB as f64,
    };

    let mut str = format!("{:.2}", result)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();

    match unit {
        Unit::B => str.push_str(" B"),
        Unit::KB => str.push_str(" KB"),
        Unit::MB => str.push_str(" MB"),
        Unit::GB => str.push_str(" GB"),
        Unit::TB => str.push_str(" TB"),
        Unit::PB => str.push_str(" PB"),
        Unit::KIB => str.push_str(" KiB"),
        Unit::MIB => str.push_str(" MiB"),
        Unit::GIB => str.push_str(" GiB"),
        Unit::TIB => str.push_str(" TiB"),
        Unit::PIB => str.push_str(" PiB"),
    }

    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size_unit() {
        assert_eq!(parse_size_unit("123").unwrap(), (123_f64, Unit::B));
        assert_eq!(parse_size_unit("12.34").unwrap(), (12.34_f64, Unit::B));
        assert_eq!(parse_size_unit("123B").unwrap(), (123_f64, Unit::B));
        assert_eq!(parse_size_unit("12.34B").unwrap(), (12.34_f64, Unit::B));

        assert_eq!(parse_size_unit("12.34kb").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34kib").unwrap(), (12.34_f64, Unit::KIB));
        assert_eq!(parse_size_unit("12.34KB").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34KiB").unwrap(), (12.34_f64, Unit::KIB));
        
        assert_eq!(parse_size_unit("12.34mb").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34mib").unwrap(), (12.34_f64, Unit::MIB));
        assert_eq!(parse_size_unit("12.34MB").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34MiB").unwrap(), (12.34_f64, Unit::MIB));

        assert_eq!(parse_size_unit("12.34gb").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34gib").unwrap(), (12.34_f64, Unit::GIB));
        assert_eq!(parse_size_unit("12.34GB").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34GiB").unwrap(), (12.34_f64, Unit::GIB));

        assert_eq!(parse_size_unit("12.34tb").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34tib").unwrap(), (12.34_f64, Unit::TIB));
        assert_eq!(parse_size_unit("12.34TB").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34TiB").unwrap(), (12.34_f64, Unit::TIB));

        assert_eq!(parse_size_unit("12.34pb").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34pib").unwrap(), (12.34_f64, Unit::PIB));
        assert_eq!(parse_size_unit("12.34PB").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34PiB").unwrap(), (12.34_f64, Unit::PIB));

        assert_eq!(parse_size_unit("12.34 kb").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34 kib").unwrap(), (12.34_f64, Unit::KIB));
        assert_eq!(parse_size_unit("12.34 KB").unwrap(), (12.34_f64, Unit::KB));
        assert_eq!(parse_size_unit("12.34 KiB").unwrap(), (12.34_f64, Unit::KIB));
        
        assert_eq!(parse_size_unit("12.34 mb").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34 mib").unwrap(), (12.34_f64, Unit::MIB));
        assert_eq!(parse_size_unit("12.34 MB").unwrap(), (12.34_f64, Unit::MB));
        assert_eq!(parse_size_unit("12.34 MiB").unwrap(), (12.34_f64, Unit::MIB));

        assert_eq!(parse_size_unit("12.34 gb").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34 gib").unwrap(), (12.34_f64, Unit::GIB));
        assert_eq!(parse_size_unit("12.34 GB").unwrap(), (12.34_f64, Unit::GB));
        assert_eq!(parse_size_unit("12.34 GiB").unwrap(), (12.34_f64, Unit::GIB));

        assert_eq!(parse_size_unit("12.34 tb").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34 tib").unwrap(), (12.34_f64, Unit::TIB));
        assert_eq!(parse_size_unit("12.34 TB").unwrap(), (12.34_f64, Unit::TB));
        assert_eq!(parse_size_unit("12.34 TiB").unwrap(), (12.34_f64, Unit::TIB));

        assert_eq!(parse_size_unit("12.34 pb").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34 pib").unwrap(), (12.34_f64, Unit::PIB));
        assert_eq!(parse_size_unit("12.34 PB").unwrap(), (12.34_f64, Unit::PB));
        assert_eq!(parse_size_unit("12.34 PiB").unwrap(), (12.34_f64, Unit::PIB));
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("123").unwrap(), 123);
        assert_eq!(parse("1.23B").unwrap(), 1);
        assert_eq!(parse("1.23KB").unwrap(), 1_230);
        assert_eq!(parse("1.23MB").unwrap(), 1_230_000);
        assert_eq!(parse("1.23GB").unwrap(), 1_230_000_000);
        assert_eq!(parse("1.23TB").unwrap(), 1_230_000_000_000);
        assert_eq!(parse("1.23PB").unwrap(), 1_230_000_000_000_000);
        assert_eq!(parse("1.23KIB").unwrap(), 1_259);
        assert_eq!(parse("1.23MIB").unwrap(), 1_289_748);
        assert_eq!(parse("1.23GIB").unwrap(), 1_320_702_443);
        assert_eq!(parse("1.23TIB").unwrap(), 1_352_399_302_164);
        assert_eq!(parse("1.23PIB").unwrap(), 1_384_856_885_416_427);
    }

    #[test]
    fn test_parse_to() {
        assert_eq!(parse_to("123", Unit::KB).unwrap(), 0.123);
        assert_eq!(format!("{:.2}", parse_to("1.23KB", Unit::KB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23MB", Unit::MB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23GB", Unit::GB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23TB", Unit::TB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23PB", Unit::PB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23KIB", Unit::KIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23MIB", Unit::MIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23GIB", Unit::GIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23TIB", Unit::TIB).unwrap()), "1.23");
        assert_eq!(format!("{:.2}", parse_to("1.23PIB", Unit::PIB).unwrap()), "1.23");
    }

    #[test]
    fn test_format() {
        assert_eq!(format(123), "123 B");
        assert_eq!(format(1_230), "1.23 KB");
        assert_eq!(format(1_230_000), "1.23 MB");
        assert_eq!(format(1_230_000_000), "1.23 GB");
        assert_eq!(format(1_230_000_000_000), "1.23 TB");
        assert_eq!(format(1_230_000_000_000_000), "1.23 PB");
    }


    #[test]
    fn test_format_to() {
        assert_eq!(format_to(123, Unit::B), "123 B");
        assert_eq!(format_to(1_245, Unit::KB), "1.25 KB");
        assert_eq!(format_to(1_245_000, Unit::MB), "1.25 MB");
        assert_eq!(format_to(1_245_000_000, Unit::GB), "1.25 GB");
        assert_eq!(format_to(1_245_000_000_000, Unit::TB), "1.25 TB");
        assert_eq!(format_to(1_245_000_000_000_000, Unit::PB), "1.25 PB");
        assert_eq!(format_to(1_275, Unit::KIB), "1.25 KiB");
        assert_eq!(format_to(1_306_525, Unit::MIB), "1.25 MiB");
        assert_eq!(format_to(1_337_882_312, Unit::GIB), "1.25 GiB");
        assert_eq!(format_to(1_369_991_488_208, Unit::TIB), "1.25 TiB");
        assert_eq!(format_to(1_402_871_283_925_909, Unit::PIB), "1.25 PiB");

        assert_eq!(format_to(500, Unit::KB), "0.5 KB");
        assert_eq!(format_to(500_000, Unit::MB), "0.5 MB");
        assert_eq!(format_to(500_000_000, Unit::GB), "0.5 GB");
        assert_eq!(format_to(500_000_000_000, Unit::TB), "0.5 TB");
        assert_eq!(format_to(500_000_000_000_000, Unit::PB), "0.5 PB");
        assert_eq!(format_to(512, Unit::KIB), "0.5 KiB");
        assert_eq!(format_to(524_288, Unit::MIB), "0.5 MiB");
        assert_eq!(format_to(536_870_912, Unit::GIB), "0.5 GiB");
        assert_eq!(format_to(549_755_813_888, Unit::TIB), "0.5 TiB");
        assert_eq!(format_to(562_949_953_421_312, Unit::PIB), "0.5 PiB");
    }
}