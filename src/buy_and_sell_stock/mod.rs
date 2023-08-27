pub fn buy_sell_stock(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut lowest = 0;
    let mut highest = 0;

    for (i, price) in prices.iter().enumerate() {
        if *price < prices[lowest] {
            lowest = i;
            highest = i;
        } else if *price > prices[highest] {
            highest = i;
            let profit = prices[highest] - prices[lowest];
            if profit > max_profit {
                max_profit = profit;
            }
        }
    }

    max_profit
}
