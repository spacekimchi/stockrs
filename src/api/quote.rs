use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Quote {
    pub ask: f64,
    pub ask_size: u32,
    pub bid: f64,
    pub bid_size: u32,
    pub currency: String,
    pub average_daily_volume_10_day: u64,
    pub average_daily_volume_3_month: u64,
    pub exchange_data_delayed_by: u32,
    pub exchange_timezone_name: String, /* String("America/New_York"), */
    pub exchange_timezone_short_name: String, /* String("EDT"), */
    pub eps_trailing_twelve_months: f64,
    pub fifty_day_average: f64,
    pub fifty_day_average_change: f64,
    pub fifty_day_average_change_percent: f64,
    pub fifty_two_week_high: f64,
    pub fifty_two_week_high_change: f64,
    pub fifty_two_week_high_change_percent: f64,
    pub fifty_two_week_low: f64,
    pub fifty_two_week_low_change: f64,
    pub fifty_two_week_low_change_percent: f64,
    pub fifty_two_week_range: String, /* String("348.11 - 479.98"), */
    pub full_exchange_name: String, /* String("NYSEArca") */
    pub long_name: String, /* String("SPDR S&P 500 ETF Trust"), */
    pub short_name: String, /* String("SPDR S&P 500"), */
    pub market: String, /* String("us_market"), */
    pub market_cap: u64,
    pub market_state: String, /* String("PRE"), */
    pub post_market_change: f64,
    pub post_market_change_percent: f64,
    pub post_market_price: f64,
    pub post_market_time: u64,
    pub pre_market_change: f64,
    pub pre_market_change_percent: f64,
    pub pre_market_price: f64,
    pub pre_market_time: u64,
    pub quote_source_name: String, /* String("Nasdaq Real Time Price"), */
    pub quote_type: String, /* String("ETF"), */
    pub region: String, /* String("US"), */
    pub regular_market_change: f64,
    pub regular_market_change_percent: f64,
    pub regular_market_day_high: f64,
    pub regular_market_day_low: f64,
    pub regular_market_day_range: String, /* String("378.671 - 385.25"), */
    pub regular_market_open: f64,
    pub regular_market_previous_close: f64,
    pub regular_market_price: f64,
    pub regular_market_time: u64,
    pub regular_market_volume: u64,
    pub shares_outstanding: u64,
    pub source_interval: u32, /* default 15 */
    pub symbol: String, /* String("SPY"), */
    pub trailing_annual_dividend_rate: f64,
    pub trailing_annual_dividend_yield: f64,
    pub trailing_p_e: f64,
    pub trailing_three_month_nav_returns: f64,
    pub trailing_three_month_returns: f64,
    pub two_hundred_day_average: f64,
    pub two_hundred_day_average_change: f64,
    pub two_hundred_day_average_change_percent: f64,
    pub type_disp: String, /* String("ETF"), */
    pub ytd_return: f64,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(alias="quoteResponse", alias="finance")]
    pub response: QuoteResponse,
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct QuoteResponse {
    pub error: Option<ResponseError>,
    pub result: Option<Vec<Quote>>,
}

impl Default for QuoteResponse {
    fn default() -> Self {
        Self {
            error: Some(ResponseError::default()),
            result: Some(vec![]),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct ResponseError {
    pub code: String,
    pub description: String,
}

impl Default for ResponseError {
    fn default() -> Self {
        Self {
            code: "".to_string(),
            description: "".to_string(),
        }
    }
}

impl Response {
    pub async fn get(symbols: &[&str], query: &Vec<(&str, &str)>) -> Result<Response, reqwest::Error> {
        let response = reqwest::get(
            format!("https://query2.finance.yahoo.com/v7/finance/quote?symbols={}&{}",
                symbols.join(","),
                query.iter().fold(
                    "".to_string(),
                    |a, &b|
                    { format!("{}&{}={}", a, b.0, b.1) }
                )
            )).await?;
        /*
         *  println!("Status: {}", response.status());
         *  println!("Headers:\n{:#?}", response.headers());
         *  println!("Url:\n{:#?}", response.url());
         */
        match response.status() {
            reqwest::StatusCode::OK => {
                response.json::<Response>().await
            }
            _ => {
                response.json::<Response>().await
            }
        }
    }
}

