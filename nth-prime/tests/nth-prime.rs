extern crate nth_prime;
use nth_prime::*;

#[test]
fn first_prime() {
    assert_eq!(nprime(1).unwrap(), 2);
}
#[test]
fn second_prime() {
    assert_eq!(nprime(2).unwrap(), 3);
}
#[test]
fn sixth_prime() {
    assert_eq!(nprime(6).unwrap(), 13);
}
#[test]
fn big_prime() {
    assert_eq!(nprime(10001).unwrap(), 104743);
}
#[test]
fn no_zeroth_prime() {
    assert!(nprime(0).is_err());
}
