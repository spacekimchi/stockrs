# List of yahoo finance APIs


## Endpoints

1. **Quote**

`https://query2.finance.yahoo.com/v7/finance/quote`

required params: [symbols: ticker of company]
response object: "quoteResponse"

Example call: 
```
url: "https://query2.finance.yahoo.com/v7/finance/quote?symbols=SPY&region=US&lang=en-US"
Object {
    "quoteResponse": Object {
        "error": Null,
        "result": Array [
            Object {
                "ask": Number(0.0),
                "askSize": Number(14),
                "averageDailyVolume10Day": Number(100104170),
                "averageDailyVolume3Month": Number(83214759),
                "bid": Number(0.0),
                "bidSize": Number(8),
                "bookValue": Number(429.22),
                "cryptoTradeable": Bool(false),
                "currency": String("USD"),
                "customPriceAlertConfidence": String("HIGH"),
                "epsTrailingTwelveMonths": Number(18.43),
                "esgPopulated": Bool(false),
                "exchange": String("PCX"),
                "exchangeDataDelayedBy": Number(0),
                "exchangeTimezoneName": String("America/New_York"),
                "exchangeTimezoneShortName": String("EDT"),
                "fiftyDayAverage": Number(386.0206),
                "fiftyDayAverageChange": Number(-1.1005859),
                "fiftyDayAverageChangePercent": Number(-0.0028511067),
                "fiftyTwoWeekHigh": Number(479.98),
                "fiftyTwoWeekHighChange": Number(-95.06),
                "fiftyTwoWeekHighChangePercent": Number(-0.1980499),
                "fiftyTwoWeekLow": Number(348.11),
                "fiftyTwoWeekLowChange": Number(36.810028),
                "fiftyTwoWeekLowChangePercent": Number(0.10574252),
                "fiftyTwoWeekRange": String("348.11 - 479.98"),
                "financialCurrency": String("USD"),
                "firstTradeDateMilliseconds": Number(728317800000),
                "fullExchangeName": String("NYSEArca"),
                "gmtOffSetMilliseconds": Number(-14400000),
                "language": String("en-US"),
                "longName": String("SPDR S&P 500 ETF Trust"),
                "market": String("us_market"),
                "marketCap": Number(353272659968),
                "marketState": String("PRE"),
                "messageBoardId": String("finmb_6160262"),
                "preMarketChange": Number(-2.100006),
                "preMarketChangePercent": Number(-0.5455695),
                "preMarketPrice": Number(382.82),
                "preMarketTime": Number(1666773814),
                "priceHint": Number(2),
                "priceToBook": Number(0.89678955),
                "quoteSourceName": String("Nasdaq Real Time Price"),
                "quoteType": String("ETF"),
                "region": String("US"),
                "regularMarketChange": Number(6.05002),
                "regularMarketChangePercent": Number(1.59686),
                "regularMarketDayHigh": Number(385.25),
                "regularMarketDayLow": Number(378.671),
                "regularMarketDayRange": String("378.671 - 385.25"),
                "regularMarketOpen": Number(378.79),
                "regularMarketPreviousClose": Number(378.87),
                "regularMarketPrice": Number(384.92),
                "regularMarketTime": Number(1666728000),
                "regularMarketVolume": Number(78846348),
                "sharesOutstanding": Number(917782016),
                "shortName": String("SPDR S&P 500"),
                "sourceInterval": Number(15),
                "symbol": String("SPY"),
                "tradeable": Bool(false),
                "trailingAnnualDividendRate": Number(5.662),
                "trailingAnnualDividendYield": Number(0.014944441),
                "trailingPE": Number(20.885513),
                "trailingThreeMonthNavReturns": Number(-5.75952),
                "trailingThreeMonthReturns": Number(-5.75952),
                "triggerable": Bool(true),
                "twoHundredDayAverage": Number(411.6299),
                "twoHundredDayAverageChange": Number(-26.7099),
                "twoHundredDayAverageChangePercent": Number(-0.06488814),
                "typeDisp": String("ETF"),
                "ytdReturn": Number(-20.2643),
            },
        ],
    },
}
```


2. **Quote Summary**

`https://query2.finance.yahoo.com/v10/finance/quoteSummary/{symbol}`

Required Params: [modules: comma separated list of summaries]
    Will default to `modules=summaryDetail` if modules query param is omitted
Response Object: "quoteSummary"

Example call:

```
url: https://query2.finance.yahoo.com/v10/finance/quoteSummary/spy?modules=summaryDetail

Object {
    "quoteSummary": Object {
        "error": Null,
        "result": Array [
            Object {
                "summaryDetail": Object {
                    "algorithm": Null,
                    "ask": Number(0.0),
                    "askSize": Number(1200),
                    "averageDailyVolume10Day": Number(81965800),
                    "averageVolume": Number(82245757),
                    "averageVolume10days": Number(81965800),
                    "beta": Number(1.249815),
                    "bid": Number(150.92),
                    "bidSize": Number(1200),
                    "coinMarketCapLink": Null,
                    "currency": String("USD"),
                    "dayHigh": Number(152.49),
                    "dayLow": Number(149.36),
                    "dividendRate": Number(0.92),
                    "dividendYield": Number(0.006),
                    "exDividendDate": Number(1659657600),
                    "fiftyDayAverage": Number(153.237),
                    "fiftyTwoWeekHigh": Number(182.94),
                    "fiftyTwoWeekLow": Number(129.04),
                    "fiveYearAvgDividendYield": Number(1.02),
                    "forwardPE": Number(23.692068),
                    "fromCurrency": Null,
                    "lastMarket": Null,
                    "marketCap": Number(2448225730560),
                    "maxAge": Number(1),
                    "open": Number(150.09),
                    "payoutRatio": Number(0.1471),
                    "previousClose": Number(149.45),
                    "priceHint": Number(2),
                    "priceToSalesTrailing12Months": Number(6.3173175),
                    "regularMarketDayHigh": Number(152.49),
                    "regularMarketDayLow": Number(149.36),
                    "regularMarketOpen": Number(150.09),
                    "regularMarketPreviousClose": Number(149.45),
                    "regularMarketVolume": Number(74732290),
                    "toCurrency": Null,
                    "tradeable": Bool(false),
                    "trailingAnnualDividendRate": Number(0.89),
                    "trailingAnnualDividendYield": Number(0.005955169),
                    "trailingPE": Number(25.180164),
                    "twoHundredDayAverage": Number(157.10725),
                    "volume": Number(74732290),
                },
            },
        ],
    },
}

```
