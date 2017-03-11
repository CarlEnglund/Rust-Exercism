extern crate nth_prime;
use nth_prime::*;

#[test]
fn first_prime() {
    assert_eq!(nprime(1), 2)
}
#[test]
fn second_prime() {
    assert_eq!(nprime(2), 3)
}
#[test]
#[ignore]
fn sixth_prime() {
    assert_eq!(nprime(6), 13)
}
#[test]
#[ignore]
fn big_prime() {
    assert_eq!(nprime(10001), 104743)
}
#[test]
#[ignore]
fn no_zeroth_prime() {
//    assert_eq!(nprime(0), false)
}
