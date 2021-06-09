use regex::Regex;

pub trait RegexExt {
    fn capture_first<'a>(&self, text: &'a str) -> Option<&'a str>;
}

impl RegexExt for Regex {
    fn capture_first<'a>(&self, text: &'a str) -> Option<&'a str> {
        self.captures(text)
            .map(|capture| capture.get(1))
            .flatten()
            .map(|matching| matching.as_str())
    }
}
