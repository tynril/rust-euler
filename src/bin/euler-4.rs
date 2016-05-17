// A palindromic number reads the same both ways. The largest
// palindrome made from the product of two 2-digit numbers is
// 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two
// 3-digit numbers.

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let n = s.len();
    s.chars().take(n)
        .zip(s.chars().rev().take(n))
        .all(|(a, b)| { a == b })
}

fn find_biggest_palindrome(from: u64, to:u64) -> Option<u64> {
    let (mut i, mut n) = (to, 0u64);
    let mut j;
    
    while i >= from {
        // Early out if this i has no chance to beat our current
        // maximum palindrome.
        if i * to < n {
            break;
        }
        
        j = to;
        while j >= from {
            let value: u64 = i * j;
            if value < n {
                break;
            }
            
            if is_palindrome(value) {
                n = value;
            }
            
            j -= 1;
        }
        
        i -= 1;
    }
    
    if n > 0 {
        Some(n)
    }
    else {
        None
    }
}

fn main() {
    println!("{}", find_biggest_palindrome(100, 999).unwrap());
}
