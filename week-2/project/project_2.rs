fn main(){
	let t:f64 = 450_000.00;
	let m:f64 = 1500_000.00;
	let h:f64 = 750_000.00;
	let d:f64 = 2_850_000.00;
	let a:f64 = 250_000.009;
    
    let  tot = t*2.00 + m*1.00 + h*3.00 + d*3.00 + a*1.00;
    println!("The total price is {}", tot);

    let avg = tot / 5.0;
    println!("The average price is {}", avg);
}