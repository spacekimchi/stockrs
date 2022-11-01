mod api;
mod api_service;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let query = vec![
        ("formatted", "false"),
        ("lang", "en-US"),
        ("region", "US"),
        ("corsDomain", "finance.yahoo.com"),
        ("symbols", "spy")
    ];
    let response = api::quote::Response::get(&["spy"], &query).await?;
    println!("body: {:#?}", response);
    Ok(())
}

