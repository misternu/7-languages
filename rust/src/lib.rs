fn prime_factors(num: i64) -> Vec<i64> {
    let mut number = num;
    let mut primes = vec![];
    let mut try = 2;
    while number != 1 {
        if number % try == 0 {
            primes.push(try);
            number /= try;
        } else {
            try += 1;
        }
    }
    primes
}


#[test]
fn prime_factors_of_one() {
    assert_eq!(prime_factors(1), []);
}

#[test]
fn prime_factors_of_two() {
    assert_eq!(prime_factors(2), [2]);
}

#[test]
fn prime_factors_of_3() {
    assert_eq!(prime_factors(3), [3]);
}

#[test]
fn prime_factors_of_four() {
    assert_eq!(prime_factors(4), [2,2]);
}

#[test]
fn prime_factors_of_five() {
    assert_eq!(prime_factors(5), [5]);
}

#[test]
fn prime_factors_of_six() {
    assert_eq!(prime_factors(6), [2, 3]);
}

#[test]
fn prime_factors_of_seven() {
    assert_eq!(prime_factors(7), [7]);
}

#[test]
fn prime_factors_of_eight() {
    assert_eq!(prime_factors(8), [2,2,2]);
}

#[test]
fn prime_factors_of_nine() {
    assert_eq!(prime_factors(9), [3,3]);
}

#[test]
fn prime_factors_of_ten() {
    assert_eq!(prime_factors(10), [2,5]);
}

#[test]
fn prime_factors_of_eleven() {
    assert_eq!(prime_factors(11), [11]);
}

#[test]
fn prime_factors_of_twelve() {
    assert_eq!(prime_factors(12), [2,2,3]);
}

#[test]
fn prime_factors_of_24() {
    assert_eq!(prime_factors(24), [2,2,2,3]);
}

#[test]
fn prime_factors_of_48() {
    assert_eq!(prime_factors(48), [2,2,2,2,3]);
}
