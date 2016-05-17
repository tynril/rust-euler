// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

struct PrimeFactors {
    divisor: u64,
    value: u64,
}

impl PrimeFactors {
    fn new(value: u64) -> PrimeFactors {
        PrimeFactors { divisor: 2u64, value: value }
    }
}

impl Iterator for PrimeFactors {
    type Item = u64;
    
    fn next(&mut self) -> Option<u64> {
        while self.value > 1u64 {
            if self.value % self.divisor == 0 {
                self.value /= self.divisor;
                return Some(self.divisor);
            }
            
            self.divisor += 1u64;
            
            if self.divisor * self.divisor > self.value {
                let tmp = self.value;
                self.value = 1u64;
                if tmp > 1u64 {
                    return Some(tmp);
                }
            }
        }
        
        return None;
    }
}

fn main() {
    println!("{}", PrimeFactors::new(600851475143).max().unwrap());
}
