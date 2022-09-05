trait Sale{
    fn amount(&self) -> f64;
}

struct FullSale(f64); //(f64) means tuple struct
impl Sale for FullSale{
    fn amount(&self)->f64{
        self.0
    }
}

struct OneDollarOffCoupan(f64);
impl Sale for OneDollarOffCoupan{
    fn amount(&self)->f64{
        self.0 - 1.0
    }
}

struct TenPercentOffPromo(f64);
impl Sale for TenPercentOffPromo{
    fn amount(&self)->f64{
        self.0 * 0.9
    }
}

fn calculate_revenue(sales: &Vec<Box<dyn Sale>>) -> f64 {
    sales.iter().map(|sale| sale.amount()).sum()
}

fn main(){
    let price = 20.0;
    let regular = Box::new(FullSale(price));
    let coupon = Box::new(OneDollarOffCoupan(price));
    let promo = Box::new(TenPercentOffPromo(price));

    let sales: Vec<Box<dyn Sale>> = vec![regular, coupon, promo];
    let revenue = calculate_revenue(&sales);
    println!("Total Revenue: {}", revenue);
}