use std::io;
use std::io::Write;
// A program that helps you determine how many months it will take to pay off credit card balance.
// Inputs: balance, apbr, monthly_payment.
// Process: (-1/30) * (log(1 + ((balance / monthly_payment)*(1 - pow(1 + (apbr / 365), 30))) ) / log(1 + (apbr / 365)))
// output: It will take you {} months to pay off this card

fn calculate_credit_card_payoff_time(balance: f64, apbr: f64, monthly_payment: f64) -> i64 {
    if balance < 1.0 {
        return 0;
    }
    // calculate monthly payment
    let daily_rate: f64 = (apbr / 100.0) / 365.0;
    // calculate formula
    let res: f64 = (-1.0 / 30.0)
        * (1.0 + (balance / monthly_payment * (1.0 - (1.0 + daily_rate).powf(30.0)))).log10()
        / (1.0 + daily_rate).log10();
    // round up
    res.ceil() as i64
}
#[cfg(test)]
mod tests {
    use super::calculate_credit_card_payoff_time;

    #[test]
    fn test_credit_card_payoff_time() {
        assert_eq!(calculate_credit_card_payoff_time(5000.0, 12.0, 100.0), 70);
        assert_eq!(calculate_credit_card_payoff_time(100.0, 18.0, 10.0), 11);
        assert_eq!(calculate_credit_card_payoff_time(0.0, 15.0, 50.0), 0);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // get input_balance, "What is your balance? "
    let input_balance: f64 = get_input("What is your balance? ");
    // get apbr, "What is the APR on the card (as a percent)? "
    let apbr: f64 = get_input("What is the APR on the card (as a percent)? ");
    // get monthly_payment, "What is the monthly payment you can make? "
    let monthly_payment: f64 = get_input("What is the monthly payment you can make? ");
    // calculate months
    let months: i64

    // print, "It will take you 70 month/s to pay off this card"
}
