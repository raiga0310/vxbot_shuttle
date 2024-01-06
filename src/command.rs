use regex::Regex;

// 正規表現マッチングを行う関数
pub fn match_url(content: &str) -> Option<(String, String)> {
    let regex = Regex::new(
        r"https:\/\/(x|twitter)\.com\/(?<username>[a-zA-Z0-9_]{1,16})\/status\/(?<hash>[0-9]+)",
    )
    .unwrap();

    regex
        .captures(content)
        .map(|caps| (caps["username"].to_string(), caps["hash"].to_string()))
}

pub fn match_command(content: &str) -> Option<String> {
    let regex = Regex::new(r"x!\s+(?<mode>fixup|fx|vx)").unwrap();

    regex
        .captures(content)
        .map(|caps| (caps["mode"].to_string()))
}

// テスト関数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_url() {
        let content = "text https://twitter.com/user123/status/12345678 text";
        let (username, hash) = match_url(content).unwrap();
        assert_eq!(username, "user123");
        assert_eq!(hash, "12345678");
    }

    #[test]
    fn test_match_command() {
        let content = "x! fixup";
        let mode = match_command(content).unwrap();
        assert_eq!(mode, "fixup");
    }
}