pub fn nprime(n: usize) -> usize {
    let store: Vec<usize> = primes_lt(1000000); 
    store[n-1]
}

fn primes_lt(bound: usize) -> Vec<usize> {
    let mut primes: Vec<bool> = (0..bound + 1).map(|num| num == 2 || num & 1 != 0).collect();
    let mut num = 3usize;
    while num * num <= bound {        
        let mut j = num * num;
        while j <= bound {
            primes[j] = false;
            j += num;
        }
        num += 2;
    }
    primes.into_iter().enumerate().skip(2).filter_map(|(i, p)| if p {Some(i)} else {None}).collect::<Vec<usize>>()
}
