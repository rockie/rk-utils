use std::collections::HashSet;

/// StringUtil provides utility methods for string manipulation.
pub trait StringUtil {
    /// is_quoted returns true if the string is quoted.
    fn is_quoted(&self) -> bool;

    /// substring returns a substring of the string.
    /// The start and end parameters can be negative.
    /// If start is negative, it is treated as len(self) + start.
    /// If end is negative, it is treated as len(self) + end.    
    fn substring(&self, start: i64, end: i64) -> &str;

    /// unquote removes the quotes from the string.
    /// If unescape is true, it will unescape the string.
    /// If quote_set is provided, it will only unquote if the quote character is in the set.
    fn unquote(&self, unescape: bool, quote_set: Option<&HashSet<char>>) -> String;

    /// url_to_nodes returns a vector of nodes from a URL string.
    fn url_to_nodes(&self) -> Vec<&str>;
}

impl StringUtil for str {
    fn is_quoted(&self) -> bool {
        (self.starts_with('\'') || self.starts_with('"'))
            && self.chars().next() == self.chars().last()
    }

    fn unquote(&self, unescape: bool, quote_set: Option<&HashSet<char>>) -> String {
        if self.is_empty() || self.len() < 2 {
            return self.to_string();
        }

        let start = self.chars().next().unwrap();
        let end = self.chars().last().unwrap();

        if start != end {
            return self.to_string();
        }

        let default_quote_chars = HashSet::from(['"', '\'']);
        let quote_set = quote_set.unwrap_or(&default_quote_chars);

        if !quote_set.contains(&start) {
            return self.to_string();
        }

        let result = self.substring(1, -1);

        if unescape {
            let escaped_quote = format!("\\{}", start);
            return result.replace(&escaped_quote, &start.to_string());
        }

        result.to_string()
    }

    fn substring(&self, start: i64, end: i64) -> &str {
        let _start = if start < 0 {
            (self.len() as i64 + start) as usize
        } else {
            start as usize
        };

        let _end = if end <= 0 {
            (self.len() as i64 + end) as usize
        } else {
            end as usize
        };

        if _start > _end {
            ""
        } else {
            &self[_start.._end]
        }
    }

    fn url_to_nodes(&self) -> Vec<&str> {
        // split and filter empty strings and pad with a "/"" at the start of the return vector
        let mut nodes = vec!["/"];
        nodes.extend(self.split('/').filter(|s| !s.is_empty()));
        nodes
    }
}
