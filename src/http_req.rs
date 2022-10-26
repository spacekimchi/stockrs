use std::collections::HashMap;
/*
 * https://query2.finance.yahoo.com/v10/finance/quoteSummary/aapl?modules=assetProfile%2CbalanceSheetHistory%2CbalanceSheetHistoryQuarterly%2CcalendarEvents%2CcashflowStatementHistory%2CcashflowStatementHistoryQuarterly%2CdefaultKeyStatistics%2Cearnings%2CearningsHistory%2CearningsTrend%2CesgScores%2CfinancialData%2CfundOwnership%2CfundPerformance%2CfundProfile%2CindexTrend%2CincomeStatementHistory%2CincomeStatementHistoryQuarterly%2CindustryTrend%2CinsiderHolders%2CinsiderTransactions%2CinstitutionOwnership%2CmajorHoldersBreakdown%2CpageViews%2Cprice%2CquoteType%2CrecommendationTrend%2CsecFilings%2CnetSharePurchaseActivity%2CsectorTrend%2CsummaryDetail%2CsummaryProfile%2CtopHoldings%2CupgradeDowngradeHistory&formatted=false&lang=en-US&region=US&corsDomain=finance.yahoo.com
 *
 *
 * formatted={true, false}; required: False
 * lang=en-Us
 * region=US
 * corsDomain=finance.yahoo.com
 *
 */


const ENDPOINTS: HashMap<&str, &str> = HashMap::from([
    ("quote_summary", "https://query2.finance.yahoo.com/v10/finance/quoteSummary"),
]);

fn build_query_params(params: &Vec<(&str, &str)>) -> &str {
    params.iter().map(|param| param.0 + '=' + param.1 ).
}

    /*
     * "{"quoteSummary" :
     *      {
     *          "result":[{
     *              "summaryDetail":{
     *                   "maxAge":1,
     *                   "priceHint":2,
     *                   "previousClose":147.27,
     *                   "open":147.185,
     *                   "dayLow":146.0,
     *                   "dayHigh":150.23,
     *                   "regularMarketPreviousClose":147.27,
     *                   "regularMarketOpen":147.185,
     *                   "regularMarketDayLow":146.0,
     *                   "regularMarketDayHigh":150.23,
     *                   "dividendRate":0.92,
     *                   "dividendYield":0.0062,
     *                   "exDividendDate":1659657600,
     *                   "payoutRatio":0.1471,
     *                   "fiveYearAvgDividendYield":1.02,
     *                   "beta":1.249815,
     *                   "trailingPE":24.702478,
     *                   "forwardPE":23.206522,
     *                   "volume":75981918,
     *                   "regularMarketVolume":75981918,
     *                   "averageVolume":82039868,
     *                   "averageVolume10days":82123520,
     *                   "averageDailyVolume10Day":82123520,
     *                   "bid":149.53,
     *                   "ask":149.63,
     *                   "bidSize":1000,
     *                   "askSize":1200,
     *                   "marketCap":2401781153792,
     *                   "fiftyTwoWeekLow":129.04,
     *                   "fiftyTwoWeekHigh":182.94,
     *                   "priceToSalesTrailing12Months":6.1974735,
     *                   "fiftyDayAverage":154.107,
     *                   "twoHundredDayAverage":157.31915,
     *                   "trailingAnnualDividendRate":0.89,
     *                   "trailingAnnualDividendYield":0.0060433215,
     *                   "currency":
     *                   "USD",
     *                   "fromCurrency":null,
     *                   "toCurrency":null,
     *                   "lastMarket":null,
     *                   "coinMarketCapLink":null,
     *                   "algorithm":null,
     *                   "tradeable":false
     *              }
     *          }],
     *          "error":null
     *      }
     *  }"
     * 
     */
