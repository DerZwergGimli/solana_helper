#[cfg(test)]
mod test_helper {
    use reqwest::StatusCode;
    use tokio;
    use std::env;
    use crate::helper;

    #[tokio::test]
    async fn test_fetch_success() {
        env::remove_var("SOLANABEACH_TOKEN");
        let result = helper::fetch("http://example.com".to_string(), None).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_fetch_success_with_env() {
        env::set_var("SOLANABEACH_TOKEN", "supertoken123");
        let result = helper::fetch("http://example.com".to_string(), None).await.unwrap();
        assert_eq!(result.status(), StatusCode::OK);
    }
}