type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    /*
    let res = reqwest::get("https://query1.finance.yahoo.com/v8/finance/chart/SPY?region=US&lang=en-US&includePrePost=false&interval=2m&useYfid=true&range=1d&corsDomain=finance.yahoo.com&.tsrc=finance").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    */



    /*
    let res = reqwest::get("https://query2.finance.yahoo.com/v7/finance/quote?symbols=SPY&fields=exchangeTimezoneName,exchangeTimezoneShortName,regularMarketTime,gmtOffSetMilliseconds&region=US&lang=en-US").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    */

    let res = reqwest::get("https://query2.finance.yahoo.com/v10/finance/quoteSummary/aapl?modules=summaryDetail&formatted=false&lang=en-US&region=US&corsDomain=finance.yahoo.com").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.json().await?;
    println!("Body:\n{:?}", body);

    /*
    let mut map = std::collections::HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client.get("https://query1.finance.yahoo.com/v8/finance/chart/SPY?region=US&lang=en-US&includePrePost=false&interval=2m&useYfid=true&range=1d&corsDomain=finance.yahoo.com&.tsrc=finance")
        .send()
        .await?
        .text()
        .await?;

    println!("{:#?}", res);
    */

    Ok(())
}
