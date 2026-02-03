pub fn parse_cpu_usage(line: &str) -> Option<f64> {
    // The output from Get-Counter includes the counter name and then the value.
    // We are only interested in the lines that can be parsed as a float.
    // The values are sometimes enclosed in quotes, so we need to remove them.
    line.trim().replace('"', "").parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu_usage_with_valid_number() {
        assert_eq!(parse_cpu_usage(" \"6.250000\" "), Some(6.25));
    }

    #[test]
    fn test_parse_cpu_usage_with_header() {
        assert_eq!(parse_cpu_usage("\"\\Processor(_Total)\\% Processor Time\""), None);
    }

    #[test]
    fn test_parse_cpu_usage_with_empty_string() {
        assert_eq!(parse_cpu_usage(""), None);
    }

    #[test]
    fn test_parse_cpu_usage_with_invalid_string() {
        assert_eq!(parse_cpu_usage("not a number"), None);
    }
}
