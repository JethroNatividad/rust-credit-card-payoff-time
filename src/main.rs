// A program that helps you determine how many months it will take to pay off credit card balance.
// Inputs: balance, apbr, monthly_payment.
// Process: (-1/30) * (log(1 + ((balance / monthly_payment)*(1 - pow(1 + (apbr / 365), 30))) ) / log(1 + (apbr / 365)))
// output: It will take you {} months to pay off this card

fn calculate_credit_card_payoff_time(balance: f64, apbr: f64, monthly_payment: f64) -> f64 {
    // calculate monthly payment
    // calculate formula
    // round
    // return
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_card_payoff_time() {
        assert_eq!(calculate_credit_card_payoff_time(5000.0, 12.0, 100.0), 70.0);
    }
}
fn main() {
    println!("Hello, world!");
}
