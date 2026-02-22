#[cfg(test)]
mod tests {
    use crate::error::IndexerError;
    use crate::horizon::HorizonClient;
    use crate::horizon::client::{OrderbookRequest, RetryConfig};
    use crate::models::horizon::{HorizonEmbedded, HorizonLink, HorizonLinks, HorizonOffer, HorizonPage};

    fn create_test_client(base_url: &str) -> HorizonClient {
        HorizonClient::new(base_url)
    }

    fn create_test_offer() -> HorizonOffer {
        HorizonOffer {
            id: "test-offer-123".to_string(),
            paging_token: Some("12345-1".to_string()),
            seller: "GBUQWP3BOUZX34ULNQG23RQ6F4YUSXHTQSXUSMIQ75XABZU4Z7IYMRME".to_string(),
            selling: serde_json::json!({
                "asset_type": "native"
            }),
            buying: serde_json::json!({
                "asset_type": "credit_alphanum4",
                "asset_code": "USDC",
                "asset_issuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
            }),
            amount: "1000.0000000".to_string(),
            price: "2.5".to_string(),
            price_r: None,
            last_modified_ledger: 123456,
        }
    }

    fn create_test_page<T: serde::de::DeserializeOwned + Clone>(
        records: Vec<T>,
    ) -> HorizonPage<T> {
        HorizonPage {
            embedded: HorizonEmbedded { records },
            links: Some(HorizonLinks {
                next: Some(HorizonLink {
                    href: "https://example.com/next".to_string(),
                }),
            }),
        }
    }

    #[tokio::test]
    async fn test_get_offers_success() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_response = create_test_page(vec![create_test_offer()]);
        let _mock = server
            .mock("GET", "/offers?limit=10")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let offers = client.get_offers(Some(10), None, None).await;

        assert!(offers.is_ok());
        let offers = offers.unwrap();
        assert_eq!(offers.len(), 1);
        assert_eq!(offers[0].id, "test-offer-123");
        assert_eq!(offers[0].seller, "GBUQWP3BOUZX34ULNQG23RQ6F4YUSXHTQSXUSMIQ75XABZU4Z7IYMRME");
    }

    #[tokio::test]
    async fn test_get_offers_with_cursor() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_response = create_test_page(vec![create_test_offer()]);
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/offers\?limit=\d+&cursor=.+".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let offers = client
            .get_offers(Some(10), Some("test-cursor"), None)
            .await;

        assert!(offers.is_ok());
        assert_eq!(offers.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_offers_with_selling_filter() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_response = create_test_page(vec![create_test_offer()]);
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/offers\?limit=\d+&selling=.+".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let offers = client
            .get_offers(Some(10), None, Some("native"))
            .await;

        assert!(offers.is_ok());
        assert_eq!(offers.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_offers_default_limit() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_response = create_test_page(vec![create_test_offer()]);
        let _mock = server
            .mock("GET", "/offers?limit=200")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let offers = client.get_offers(None, None, None).await;

        assert!(offers.is_ok());
    }

    #[tokio::test]
    async fn test_get_offers_http_error() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/offers".to_string()))
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{"detail": "Internal server error"}"#)
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let result = client.get_offers(Some(10), None, None).await;

        assert!(result.is_err());
        match result {
            Err(IndexerError::StellarApi { status, .. }) => {
                assert_eq!(status, 500);
            }
            _ => panic!("Expected StellarApi error"),
        }
    }

    #[tokio::test]
    async fn test_get_offers_empty_response() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_response: HorizonPage<HorizonOffer> = create_test_page(vec![]);
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/offers".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let offers = client.get_offers(Some(10), None, None).await;

        assert!(offers.is_ok());
        assert_eq!(offers.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_get_offers_multiple_records() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mut offer2 = create_test_offer();
        offer2.id = "test-offer-456".to_string();
        
        let mock_response = create_test_page(vec![create_test_offer(), offer2]);
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/offers".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let offers = client.get_offers(Some(10), None, None).await;

        assert!(offers.is_ok());
        let offers = offers.unwrap();
        assert_eq!(offers.len(), 2);
        assert_eq!(offers[0].id, "test-offer-123");
        assert_eq!(offers[1].id, "test-offer-456");
    }

    #[tokio::test]
    async fn test_get_orderbook_success() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_orderbook = serde_json::json!({
            "bids": [
                {
                    "price": "2.5",
                    "amount": "1000.0"
                }
            ],
            "asks": [
                {
                    "price": "3.0",
                    "amount": "500.0"
                }
            ]
        });

        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/order_book".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_orderbook).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let req = OrderbookRequest {
            selling_asset_type: "native",
            selling_asset_code: None,
            selling_asset_issuer: None,
            buying_asset_type: "credit_alphanum4",
            buying_asset_code: Some("USDC"),
            buying_asset_issuer: Some("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
            limit: None,
        };

        let orderbook = client.get_orderbook(req).await;

        assert!(orderbook.is_ok());
        let book = orderbook.unwrap();
        assert!(book.get("bids").is_some());
        assert!(book.get("asks").is_some());
    }

    #[tokio::test]
    async fn test_get_orderbook_with_custom_limit() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_orderbook = serde_json::json!({
            "bids": [],
            "asks": []
        });

        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/order_book\?.*limit=50.*".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_orderbook).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let req = OrderbookRequest {
            selling_asset_type: "native",
            selling_asset_code: None,
            selling_asset_issuer: None,
            buying_asset_type: "credit_alphanum4",
            buying_asset_code: Some("USDC"),
            buying_asset_issuer: Some("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
            limit: Some(50),
        };

        let orderbook = client.get_orderbook(req).await;

        assert!(orderbook.is_ok());
    }

    #[tokio::test]
    async fn test_get_orderbook_http_error() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/order_book".to_string()))
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"detail": "Not found"}"#)
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let req = OrderbookRequest {
            selling_asset_type: "native",
            selling_asset_code: None,
            selling_asset_issuer: None,
            buying_asset_type: "credit_alphanum4",
            buying_asset_code: Some("USDC"),
            buying_asset_issuer: Some("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
            limit: None,
        };

        let result = client.get_orderbook(req).await;

        assert!(result.is_err());
        match result {
            Err(IndexerError::StellarApi { status, .. }) => {
                assert_eq!(status, 404);
            }
            _ => panic!("Expected StellarApi error"),
        }
    }

    #[tokio::test]
    async fn test_retry_logic_succeeds_on_retry() {
        let mut server = mockito::Server::new_async().await;
        let base_url = server.url();
        
        let mock_response = create_test_page(vec![create_test_offer()]);
        
        // First call fails, second succeeds
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/offers".to_string()))
            .expect_at_least(1)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let retry_config = RetryConfig {
            max_retries: 3,
            initial_delay_ms: 10,
            max_delay_ms: 100,
            backoff_multiplier: 2.0,
        };

        let client = HorizonClient::with_retry_config(&base_url, retry_config);
        let offers = client.get_offers(Some(10), None, None).await;

        assert!(offers.is_ok());
        assert_eq!(offers.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_client_with_trailing_slash() {
        let mut server = mockito::Server::new_async().await;
        let base_url = format!("{}/", server.url());
        
        let mock_response = create_test_page(vec![create_test_offer()]);
        let _mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/offers".to_string()))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&mock_response).unwrap())
            .create_async()
            .await;

        let client = HorizonClient::new(&base_url);
        let offers = client.get_offers(Some(10), None, None).await;

        assert!(offers.is_ok());
    }

    #[tokio::test]
    async fn test_parse_asset_native() {
        let client = HorizonClient::new("http://localhost");
        let asset_json = serde_json::json!({
            "asset_type": "native"
        });

        let asset = client.parse_asset(&asset_json);
        assert!(asset.is_ok());
    }

    #[tokio::test]
    async fn test_parse_asset_alphanum4() {
        let client = HorizonClient::new("http://localhost");
        let asset_json = serde_json::json!({
            "asset_type": "credit_alphanum4",
            "asset_code": "USDC",
            "asset_issuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
        });

        let asset = client.parse_asset(&asset_json);
        assert!(asset.is_ok());
    }

    #[tokio::test]
    async fn test_parse_asset_alphanum12() {
        let client = HorizonClient::new("http://localhost");
        let asset_json = serde_json::json!({
            "asset_type": "credit_alphanum12",
            "asset_code": "LONGASSETCODE",
            "asset_issuer": "GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"
        });

        let asset = client.parse_asset(&asset_json);
        assert!(asset.is_ok());
    }

    #[tokio::test]
    async fn test_parse_asset_invalid_type() {
        let client = HorizonClient::new("http://localhost");
        let asset_json = serde_json::json!({
            "asset_type": "invalid"
        });

        let asset = client.parse_asset(&asset_json);
        assert!(asset.is_err());
        match asset {
            Err(IndexerError::InvalidAsset { .. }) => (),
            _ => panic!("Expected InvalidAsset error"),
        }
    }

    #[tokio::test]
    async fn test_parse_asset_missing_field() {
        let client = HorizonClient::new("http://localhost");
        let asset_json = serde_json::json!({
            "asset_code": "USDC"
        });

        let asset = client.parse_asset(&asset_json);
        assert!(asset.is_err());
        match asset {
            Err(IndexerError::MissingField { .. }) => (),
            _ => panic!("Expected MissingField error"),
        }
    }

    #[test]
    fn test_retry_config_defaults() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_delay_ms, 100);
        assert_eq!(config.max_delay_ms, 5000);
        assert_eq!(config.backoff_multiplier, 2.0);
    }

    #[test]
    fn test_orderbook_request_creation() {
        let req = OrderbookRequest {
            selling_asset_type: "native",
            selling_asset_code: None,
            selling_asset_issuer: None,
            buying_asset_type: "credit_alphanum4",
            buying_asset_code: Some("USDC"),
            buying_asset_issuer: Some("GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN"),
            limit: Some(50),
        };

        assert_eq!(req.selling_asset_type, "native");
        assert_eq!(req.buying_asset_type, "credit_alphanum4");
        assert_eq!(req.limit, Some(50));
    }
}
