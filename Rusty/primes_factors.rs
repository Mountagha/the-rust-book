

pub fn factors(n: u64) -> Vec<u64> {
    let mut primes_divisors: Vec<u64> = Vec::new();
        let mut quotient = n;
    let mut cur_div = 2;
    let mut reconstructed_number = 1;
    loop {
        if is_prime(cur_div) {
            if quotient % cur_div == 0 {
                primes_divisors.push(cur_div);
                reconstructed_number *= cur_div;
                quotient /= cur_div;
            } else {
                //index += 1;
                cur_div += 1;
            }
        } else {
            cur_div += 1;
        }
        if (cur_div >= n/2) || reconstructed_number == n {
            break;
        }
    }
    primes_divisors
}

fn main() {
    let v = factors1(93_819_012_551);
    //let v = factors(8);
    //let v = factors(901_255);
    for x in v {
        println!("{}", x);
    }
}

// saw this solution online. was amazed.
pub fn factors1(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut candidates = 2..;
    while n > 1 {
        let x = candidates.next().unwrap();
        while n % x == 0 {
            n /= x;
            factors.push(x);
        }
    }
    factors
}