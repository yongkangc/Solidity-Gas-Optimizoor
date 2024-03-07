fn lexer() -> &'static str {
    "lexer"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        assert_eq!(lexer(), "lexer");
    }
}
