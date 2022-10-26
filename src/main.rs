//mod http_req;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(serde::Deserialize, Debug)]
struct QuoteSummary {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let client = reqwest::Client::new();
    let query = vec![
        ("modules", "summaryDetail"),
        ("formatted", "false"),
        ("lang", "en-US"),
        ("region", "US"),
        ("corsDomain", "finance.yahoo.com"),
    ];
    let spy_q = "https://query2.finance.yahoo.com/v7/finance/quote?symbols=SPY&region=US&lang=en-US";
    let aapl_qs = "https://query2.finance.yahoo.com/v10/finance/quoteSummary/aapl";
    let response = client.get(aapl_qs)
          .query(&query)
          .send().await?;

    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Url:\n{:#?}", response.url());

    let body: serde_json::Value = response.json().await?;
    println!("Body:\n{:#?}", body);

    Ok(())
}
