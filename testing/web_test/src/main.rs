fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        assert!(true);
    }
}

#[cfg(all(test, feature = "web"))]
mod web_test {
    #[test]
    fn test_web() {
        assert!(true);
    }
}
