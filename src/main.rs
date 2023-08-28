mod buy_and_sell_stock;

fn main() {
    let prices = vec![7, 6, 4, 3, 1];
    let result = buy_and_sell_stock::buy_sell_stock(prices);
    println!("The result is: {result}");
}
