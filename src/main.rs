

fn main() {
    let mut primes: Vec<u32> = vec![2,3];
    

    for i in 5..1000000 {
        if is_prime(&primes, i) {
            primes.push(i);
        }
    }
    println!("{:?}",primes.len());
}

fn is_prime(primes: &Vec<u32>, number: u32) -> bool{
    !primes.iter()
        .take_while(|&&n| ((number as f64).sqrt() as u32) >= n ) //Optimization
        .any(|a| number%a == 0) // if any number is divisible with parameter return
}
