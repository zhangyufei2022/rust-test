/// 买卖股票的最佳时机 II
pub fn max_profit(prices: Vec<i32>) -> i32 {
    // use std::cmp::max;
    let mut sum = 0;
    for i in 1..prices.len() {
        // sum = sum + max(0, prices[i] - prices[i - 1]);
        sum = sum + (prices[i] - prices[i - 1]).max(0);
    }

    sum as i32
}

pub fn max_profit_2(prices: Vec<i32>) -> i32 {
    prices.windows(2).map(|x| (x[1] - x[0]).max(0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let prices = vec![7, 1, 6, 3, 4, 9];
        let res = max_profit(prices);
        assert_eq!(res, 11);

        let prices = vec![7, 1, 6, 3, 4, 9];
        let res = max_profit_2(prices);
        assert_eq!(res, 11);
    }
}
