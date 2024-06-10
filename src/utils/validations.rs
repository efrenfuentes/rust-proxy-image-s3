use regex::Regex;

pub fn validate_resolution(resolution: &str) -> bool {
    let re = Regex::new(r"^\d{1,4}x\d{1,4}$").unwrap();

    re.is_match(resolution) || resolution == "original"
}

pub fn validate_extension(extension: &str) -> bool {
    let re = Regex::new(r"^(?i)(jpg|jpeg|png|gif)$").unwrap();

    re.is_match(extension)
}
