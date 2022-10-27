//mod http_req;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase", default)]
struct Quote {
    ask: f64,
    ask_size: u32,
    bid: f64,
    bid_size: u32,
    currency: String,
    average_daily_volume_10_day: u64,
    average_daily_volume_3_month: u64,
    exchange_data_delayed_by: u32,
    exchange_timezone_name: String, /* String("America/New_York"), */
    exchange_timezone_short_name: String, /* String("EDT"), */
    eps_trailing_twelve_months: f64,
    fifty_day_average: f64,
    fifty_day_average_change: f64,
    fifty_day_average_change_percent: f64,
    fifty_two_week_high: f64,
    fifty_two_week_high_change: f64,
    fifty_two_week_high_change_percent: f64,
    fifty_two_week_low: f64,
    fifty_two_week_low_change: f64,
    fifty_two_week_low_change_percent: f64,
    fifty_two_week_range: String, /* String("348.11 - 479.98"), */
    full_exchange_name: String, /* String("NYSEArca") */
    long_name: String, /* String("SPDR S&P 500 ETF Trust"), */
    short_name: String, /* String("SPDR S&P 500"), */
    market: String, /* String("us_market"), */
    market_cap: u64,
    market_state: String, /* String("PRE"), */
    post_market_change: f64,
    post_market_change_percent: f64,
    post_market_price: f64,
    post_market_time: u64,
    pre_market_change: f64,
    pre_market_change_percent: f64,
    pre_market_price: f64,
    pre_market_time: u64,
    quote_source_name: String, /* String("Nasdaq Real Time Price"), */
    quote_type: String, /* String("ETF"), */
    region: String, /* String("US"), */
    regular_market_change: f64,
    regular_market_change_percent: f64,
    regular_market_day_high: f64,
    regular_market_day_low: f64,
    regular_market_day_range: String, /* String("378.671 - 385.25"), */
    regular_market_open: f64,
    regular_market_previous_close: f64,
    regular_market_price: f64,
    regular_market_time: u64,
    regular_market_volume: u64,
    shares_outstanding: u64,
    source_interval: u32, /* default 15 */
    symbol: String, /* String("SPY"), */
    trailing_annual_dividend_rate: f64,
    trailing_annual_dividend_yield: f64,
    trailing_p_e: f64,
    trailing_three_month_nav_returns: f64,
    trailing_three_month_returns: f64,
    two_hundred_day_average: f64,
    two_hundred_day_average_change: f64,
    two_hundred_day_average_change_percent: f64,
    type_disp: String, /* String("ETF"), */
    ytd_return: f64,
}

impl Default for Quote {
    fn default() -> Self {
        Self {
            ask: 0.0,
            ask_size: 0,
            average_daily_volume_10_day: 0,
            average_daily_volume_3_month: 0,
            bid: 0.0,
            bid_size: 0,
            currency: "".to_string(),
            exchange_data_delayed_by: 0,
            exchange_timezone_name: "".to_string(), /* String("America/New_York"), */
            eps_trailing_twelve_months: 0.0,
            exchange_timezone_short_name: "".to_string(), /* String("EDT"), */
            fifty_day_average: 0.0,
            fifty_day_average_change: 0.0,
            fifty_day_average_change_percent: 0.0,
            fifty_two_week_high: 0.0,
            fifty_two_week_high_change: 0.0,
            fifty_two_week_high_change_percent: 0.0,
            fifty_two_week_low: 0.0,
            fifty_two_week_low_change: 0.0,
            fifty_two_week_low_change_percent: 0.0,
            fifty_two_week_range: "".to_string(), /* String("348.11 - 479.98"), */
            full_exchange_name: "".to_string(), /* String("NYSEArca") */
            long_name: "".to_string(), /* String("SPDR S&P 500 ETF Trust"), */
            short_name: "".to_string(), /* String("SPDR S&P 500"), */
            market: "".to_string(), /* String("us_market"), */
            market_cap: 0,
            market_state: "".to_string(), /* String("PRE"), */
            post_market_change: 0.0,
            post_market_change_percent: 0.0,
            post_market_price: 0.0,
            post_market_time: 0,
            pre_market_change: 0.0,
            pre_market_change_percent: 0.0,
            pre_market_price: 0.0,
            pre_market_time: 0,
            quote_source_name: "".to_string(), /* String("Nasdaq Real Time Price"), */
            quote_type: "".to_string(), /* String("ETF"), */
            region: "".to_string(), /* String("US"), */
            regular_market_change: 0.0,
            regular_market_change_percent: 0.0,
            regular_market_day_high: 0.0,
            regular_market_day_low: 0.0,
            regular_market_day_range: "".to_string(), /* String("378.671 - 385.25"), */
            regular_market_open: 0.0,
            regular_market_previous_close: 0.0,
            regular_market_price: 0.0,
            regular_market_time: 0,
            regular_market_volume: 0,
            shares_outstanding: 0,
            source_interval: 0, /* default 15 */
            symbol: "".to_string(), /* String("SPY"), */
            trailing_annual_dividend_rate: 0.0,
            trailing_annual_dividend_yield: 0.0,
            trailing_p_e: 0.0,
            trailing_three_month_nav_returns: 0.0,
            trailing_three_month_returns: 0.0,
            two_hundred_day_average: 0.0,
            two_hundred_day_average_change: 0.0,
            two_hundred_day_average_change_percent: 0.0,
            type_disp: "".to_string(), /* String("ETF"), */
            ytd_return: 0.0,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Response {
    #[serde(alias="quoteResponse", alias="finance")]
    response: QuoteResponse,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QuoteResponse {
    #[serde(rename="error")]
    response_error: Option<ResponseError>,
    #[serde(rename="result")]
    response_result: Option<Vec<Quote>>,
}

#[derive(serde::Deserialize, Debug)]
struct ResponseError {
    code: Option<String>,
    description: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let query = vec![
        ("formatted", "false"),
        ("lang", "en-US"),
        ("region", "US"),
        ("corsDomain", "finance.yahoo.com"),
    ];
    let spy_q = "https://query2.finance.yahoo.com/v7/finance/quote?symbols=spy";
    let aapl_qs = "https://query2.finance.yahoo.com/v10/finance/quoteSummary/aapl";
    let response = client.get(spy_q)
          .query(&query)
          .send().await?;

    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Url:\n{:#?}", response.url());

    let body: Response = response.json().await?;

    println!("Body:\n{:#?}", body);

    Ok(())
}

