// Each new term in the Fibonacci sequence is generated by adding
// the previous two terms. By starting with 1 and 2, the first 10
// terms will be:
//
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
//
// By considering the terms in the Fibonacci sequence whose values
// do not exceed four million, find the sum of the even-valued
// terms.

fn main() {
    let (mut current, mut next, mut sum) = (0u64, 1u64, 0u64);
    
    loop {
        let value = current + next;
        current = next;
        next = value;
        
        if value % 2 == 0 {
            sum += value;
        }
        
        if value > 4_000_000 {
            break;
        }
    }
    
    println!("{}", sum);
}
