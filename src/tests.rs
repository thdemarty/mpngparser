#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn test_bw_nok_broken_dimensions() {
        let mut parser = Parser::new(std::path::Path::new("minipng-samples/bw/nok/broken-dimensions.mp"));
        let mpng = parser.parse();
        
        assert_eq!(mpng.is_err(), true);

        let mut parser = Parser::new(std::path::Path::new("minipng-samples/bw/nok/broken-dimensions2.mp"));
        let mpng = parser.parse();
        
        assert_eq!(mpng.is_err(), true);
    }

    #[test]
    fn test_bw_nok_wrong_magic() {
        let mut parser = Parser::new(std::path::Path::new("minipng-samples/bw/nok/wrong-magic.mp"));
        let mpng = parser.parse();
        
        assert_eq!(mpng.is_err(), true);
    }

    #[test]
    fn test_bw_nok_missing_data() {
        let mut parser = Parser::new(std::path::Path::new("minipng-samples/bw/nok/missing-data.mp"));
        let mpng = parser.parse();
        
        assert_eq!(mpng.is_err(), true);
    }

    #[test]
    fn test_bw_nok_missing_header() {
        let mut parser = Parser::new(std::path::Path::new("minipng-samples/bw/nok-missing-header.mp"));
        let mpng = parser.parse();
        
        assert_eq!(mpng.is_err(), true);
    }
}