pub struct Pattern {
    pub prefix: String,
    pub suffix: String,
}

impl From<&str> for Pattern {
    fn from(pattern: &str) -> Self {
        let placeholder = "{}";
        if let Some(tag_start) = pattern.find(placeholder) {
            let prefix = pattern[..tag_start].to_owned();
            let suffix = pattern[tag_start + placeholder.len()..].to_owned();
            Pattern { prefix, suffix }
        } else {
            Pattern { prefix: pattern.to_owned(), suffix: "".to_string()}
        }
        
    }
}

impl Pattern {
    pub fn match_and_extract(&self, input: &str) -> Result<Option<String>, ()> {
        if "" == &self.suffix && input == &self.prefix {
            return Ok(None);
        } else if input.starts_with(&self.prefix) && input.ends_with(&self.suffix) {
            let sub_start = self.prefix.len();
            let sub_end = input.len() - self.suffix.len();
            return Ok(Some(input[sub_start..sub_end].to_owned()));
        } else {
            return Err(());
        }
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pattern_matching() {
        let patterns: Vec<crate::util::Pattern> = vec![
            "Hello {}, how are you?".into(),
            "It is {} degrees outside".into(),
            "BDK is a bitcoin {}".into(),
            "This str has no varible.".into(),
        ];

        let input1 = "Hello Josh, how are you?";
        for pattern in &patterns {
            if let Ok(Some(substring)) = pattern.match_and_extract(input1) {
                assert_eq!("Josh", substring);
            }
        }

        let input2 = "It is 30 degrees outside";
        for pattern in &patterns {
            if let Ok(Some(substring)) = pattern.match_and_extract(input2) {
                assert_eq!("30", substring);
            }
        }

        let input3 = "BDK is a bitcoin project";
        for pattern in &patterns {
            if let Ok(Some(substring)) = pattern.match_and_extract(input3) {
                assert_eq!("project", substring);
            }
        }

        let input4 = "This str has no varible.";
        let result4 = patterns[3].match_and_extract(input4);
        assert_eq!(result4, Ok(None));

        let input5 = "This str has no varible";
        let result5 = patterns[3].match_and_extract(input5);
        assert_eq!(result5, Err(()));
    }
}