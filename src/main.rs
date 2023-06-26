fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sync() {}

    #[tokio::test]
    async fn test_async() {}
}