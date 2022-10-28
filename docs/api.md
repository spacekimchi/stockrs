# List of yahoo finance APIs


## Endpoints

1. **Quote**

`https://query2.finance.yahoo.com/v7/finance/quote`

required params: [symbols: ticker of company]
response object: "quoteResponse"
result will have an array of information of symbols in query params

successfull call json will be in response.body["quoteResponse"]["result"]
errors can be found in response.body["quoteResponse"]["error"]

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
                "postMarketChange": Number(-3.2700195),
                "postMarketChangePercent": Number(-0.86057675),
                "postMarketPrice": Number(376.71),
                "postMarketTime": Number(1666904492),
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

For aapl in market hours:

```

"quoteResponse": Object {
    "error": Null,
    "result": Array [
        Object {
            "ask": Number(143.4),
            "askSize": Number(10),
            "averageAnalystRating": String("1.9 - Buy"),
            "averageDailyVolume10Day": Number(83777730),
            "averageDailyVolume3Month": Number(82400950),
            "bid": Number(144.87),
            "bidSize": Number(8),
            "bookValue": Number(3.61),
            "cryptoTradeable": Bool(false),
            "currency": String("USD"),
            "customPriceAlertConfidence": String("HIGH"),
            "displayName": String("Apple"),
            "dividendDate": Number(1660176000),
            "earningsTimestamp": Number(1666900800),
            "earningsTimestampEnd": Number(1666900800),
            "earningsTimestampStart": Number(1666900800),
            "epsCurrentYear": Number(6.08),
            "epsForward": Number(6.43),
            "epsTrailingTwelveMonths": Number(6.05),
            "esgPopulated": Bool(false),
            "exchange": String("NMS"),
            "exchangeDataDelayedBy": Number(0),
            "exchangeTimezoneName": String("America/New_York"),
            "exchangeTimezoneShortName": String("EDT"),
            "fiftyDayAverage": Number(152.7634),
            "fiftyDayAverageChange": Number(-7.963394),
            "fiftyDayAverageChangePercent": Number(-0.05212894),
            "fiftyTwoWeekHigh": Number(182.94),
            "fiftyTwoWeekHighChange": Number(-38.14),
            "fiftyTwoWeekHighChangePercent": Number(-0.20848365),
            "fiftyTwoWeekLow": Number(129.04),
            "fiftyTwoWeekLowChange": Number(15.76001),
            "fiftyTwoWeekLowChangePercent": Number(0.122132756),
            "fiftyTwoWeekRange": String("129.04 - 182.94"),
            "financialCurrency": String("USD"),
            "firstTradeDateMilliseconds": Number(345479400000),
            "forwardPE": Number(22.519442),
            "fullExchangeName": String("NasdaqGS"),
            "gmtOffSetMilliseconds": Number(-14400000),
            "language": String("en-US"),
            "longName": String("Apple Inc."),
            "market": String("us_market"),
            "marketCap": Number(2327052025856),
            "marketState": String("POST"),
            "messageBoardId": String("finmb_24937"),
            "postMarketChange": Number(-1.800003),
            "postMarketChangePercent": Number(-1.243096),
            "postMarketPrice": Number(143.0),
            "postMarketTime": Number(1666903189),
            "priceEpsCurrentYear": Number(23.81579),
            "priceHint": Number(2),
            "priceToBook": Number(40.110806),
            "quoteSourceName": String("Nasdaq Real Time Price"),
            "quoteType": String("EQUITY"),
            "region": String("US"),
            "regularMarketChange": Number(-4.550003),
            "regularMarketChangePercent": Number(-3.046537),
            "regularMarketDayHigh": Number(149.04),
            "regularMarketDayLow": Number(144.13),
            "regularMarketDayRange": String("144.13 - 149.04"),
            "regularMarketOpen": Number(148.07),
            "regularMarketPreviousClose": Number(149.35),
            "regularMarketPrice": Number(144.8),
            "regularMarketTime": Number(1666900804),
            "regularMarketVolume": Number(85651510),
            "sharesOutstanding": Number(16070800384),
            "shortName": String("Apple Inc."),
            "sourceInterval": Number(15),
            "symbol": String("AAPL"),
            "tradeable": Bool(false),
            "trailingAnnualDividendRate": Number(0.89),
            "trailingAnnualDividendYield": Number(0.005959156),
            "trailingPE": Number(23.933884),
            "triggerable": Bool(true),
            "twoHundredDayAverage": Number(156.99306),
            "twoHundredDayAverageChange": Number(-12.193054),
            "twoHundredDayAverageChangePercent": Number(-0.0776662),
            "typeDisp": String("Equity"),
        },
    ],
},
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
