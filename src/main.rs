use std::ops::{Add, Range};
use num_bigint::{BigUint, ToBigUint};
use std::collections::HashMap;

fn main() {
    print_cross_sums_per_s(1..100);
    //count_cross_sums_per_s(1..100);
}

fn count_cross_sums_per_s(s_range: Range<usize>) {
	let mut cross_sums_per_s: HashMap<String, usize> = HashMap::new();

    for s in s_range {
        let k_vec: Vec<u8>  = vec![1; s];
        let m_vec: Vec<u8> = vec![4; s];

        let mut cross_sums: Vec<BigUint> = vec![];

        for i in 0..k_vec.len() {
            let mut n_vec: Vec<u8>  = vec![4; s];
            n_vec[i] = 5;

            let k: BigUint = concat(&k_vec);
            let m: BigUint = concat(&m_vec);
            let n: BigUint = concat(&n_vec);

            let result: BigUint = calc_formula(&k, &m, &n);
            let cross_sum: BigUint = calc_cross_sum(&result);

            if !cross_sums.contains(&cross_sum) {
                cross_sums.push(cross_sum);
            }
        }

        cross_sums_per_s.insert(s.to_string(), cross_sums.len());
    }

	println!("{:?}", cross_sums_per_s);
}

fn print_cross_sums_per_s(s_range: Range<usize>) {
    for s in s_range {
        let k_vec: Vec<u8>  = vec![1; s];
        let m_vec: Vec<u8> = vec![4; s];

        for i in 0..k_vec.len() {
            let mut n_vec: Vec<u8>  = vec![4; s];
            n_vec[i] = 5;

            let k: BigUint = concat(&k_vec);
            let m: BigUint = concat(&m_vec);
            let n: BigUint = concat(&n_vec);

            let result: BigUint = calc_formula(&k, &m, &n);
            let cross_sum: BigUint = calc_cross_sum(&result);

            println!("s = {}", s);
            println!("c = {}² - {}² + {} = {}", n, m, k, &result);
            println!("Quersumme(c) = {}", cross_sum);
            println!("");
        }
    }
}

fn concat(vec: &[u8]) -> BigUint {
    vec.iter().fold(0.to_biguint().unwrap(), |acc, elem| acc * 10.to_biguint().unwrap() + (elem.to_biguint().unwrap()))
}


fn calc_formula(k: &BigUint, m: &BigUint, n: &BigUint) -> BigUint {
    n.pow(2) - m.pow(2) + k
}

fn calc_cross_sum(number: &BigUint) -> BigUint {
    let mut cross_sum: BigUint = 0.to_biguint().unwrap();

    let number_str = number.to_string();

    for digit in number_str.chars() {
        cross_sum = cross_sum.add(digit.to_digit(10).unwrap());
    }

    cross_sum
}