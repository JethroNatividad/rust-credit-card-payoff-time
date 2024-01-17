// A program that helps you determine how many months it will take to pay off credit card balance.
// Inputs: balance, apbr, monthly_payment.
// Process: (-1/30) * (log(1 + ((balance / monthly_payment)*(1 - pow(1 + (apbr / 365), 30))) ) / log(1 + (apbr / 365)))
// output: It will take you {} months to pay off this card

fn calculate_credit_card_payoff_time(balance: f64, apbr: f64, monthly_payment: f64) -> f64 {
    if balance < 1.0 {
        return 0.0;
    }
    // calculate monthly payment
    let daily_rate: f64 = (apbr / 100.0) / 365.0;
    // calculate formula
    let res: f64 = (-1.0 / 30.0)
        * (1.0 + (balance / monthly_payment * (1.0 - (1.0 + daily_rate).powf(30.0)))).log10()
        / (1.0 + daily_rate).log10();
    // round up
    res.ceil()
}
#[cfg(test)]
mod tests {
    use super::calculate_credit_card_payoff_time;

    #[test]
    fn test_credit_card_payoff_time() {
        assert_eq!(calculate_credit_card_payoff_time(5000.0, 12.0, 100.0), 70.0);
        assert_eq!(calculate_credit_card_payoff_time(100.0, 18.0, 10.0), 11.0);
        assert_eq!(calculate_credit_card_payoff_time(0.0, 15.0, 50.0), 0.0);
        assert_eq!(calculate_credit_card_payoff_time(3000.0, 24.0, 30.0), 146.0);
        assert_eq!(
            calculate_credit_card_payoff_time(10000.0, 20.0, 500.0),
            26.0
        );
        assert_eq!(
            calculate_credit_card_payoff_time(2500.0, 0.0, 200.0),
            f64::INFINITY
        );
    }
}
fn main() {
    println!("Hello, world!");
}
