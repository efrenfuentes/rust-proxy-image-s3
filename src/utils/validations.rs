use regex::Regex;

pub fn validate_resolution(resolution: &str) -> bool {
    let re = Regex::new(r"^\d{1,4}x\d{1,4}$").unwrap();

    if re.is_match(resolution) || resolution == "original" {
        return true;
    }

    false
}

pub fn validate_extension(extension: &str) -> bool {
    let re = Regex::new(r"^(?i)(jpg|jpeg|png|gif)$").unwrap();

    if re.is_match(extension) {
        return true;
    }

    false
}
