#[derive(PartialEq, Debug, Clone)]
enum Currency {
    Pln,
    Eur
}

#[derive(Debug)]
 struct MonetaryAmount {
     value: f64,
     currency: Currency,
 }

impl MonetaryAmount {
    
    fn add(&mut self, other: &MonetaryAmount) -> Result<(), String> {
        self.check_currency(other)?;
        self.value += other.value;
        Ok(())
    }

    fn subtract(&mut self, other: &MonetaryAmount) -> Result<(), String> {
        self.check_currency(other)?;
        self.value -= other.value;
        Ok(())
    }
    
    fn check_currency(&self, other: &MonetaryAmount) -> Result<(), String> {
        if self.currency != other.currency{
            return Err(String::from("Currency not match!"));
        }
        Ok(())
    }
    
    fn convert(amount: &MonetaryAmount, exchange_rate: f64, currency: &Currency) -> Self {
        Self {
            value: amount.value / exchange_rate,
            currency: currency.clone(),
        }
    }
    
    fn new(value: f64, currency: &Currency) -> Self {
        Self { value, currency: currency.clone() }
    }
    
}

pub fn run() {
    let mut balance = MonetaryAmount::new(1_000.0, &Currency::Eur);
    let income = MonetaryAmount::new(2_000.0, &Currency::Eur);

    match balance.add(&income) { 
        Ok(_) => println!("Balance updated: {:?}", balance),
        Err(message) => println!("Error: {message}"),
    }
    
    let result_pln = MonetaryAmount::convert(&balance, 0.2, &Currency::Pln);
    println!("Balance converted: {:?}", result_pln);
}