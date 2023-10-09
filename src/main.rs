use std::env;
use std::io::Write;
use std::ops::Add;
use num_bigint::{BigUint, ToBigUint};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::PointStyle;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let binary_name = args[0].to_owned();
    args = args[1..].to_vec();

    if args.len() >= 2 {
        fs::create_dir_all("./Data/").unwrap();

        if args.contains(&String::from("-c")) || args.contains(&String::from("--count")) {
            args.retain(|arg| (arg != &String::from("-c") && arg != &String::from("--count")));
            args.sort();

            let start: usize = args[0].parse().expect(&format!("{} is not an integer!", args[0]));
            let stop: usize = args[1].parse().expect(&format!("{} is not an integer!", args[0]));
            count_cross_sums_per_s(start, stop);
        } else {
            args.sort();

            let start: usize = args[0].parse().expect(&format!("{} is not an integer!", args[0]));
            let stop: usize = args[1].parse().expect(&format!("{} is not an integer!", args[0]));
            find_cross_sums_per_s(start, stop);
        }
    } else {
        println!("Usage: {} [OPTIONS] <START> <STOP>", binary_name);
        println!("");
        println!("Options:");
        println!("-c, --count  Counts all cross sums per s");
    }
}


fn plot(data: Vec<(f64, f64)>, s_start: usize, s_stop: usize) {
    let plot: Plot = Plot::new(data).point_style(
        PointStyle::new()
            .colour("#35C788"),
    );

    let view = ContinuousView::new()
        .add(plot)
        .x_label("s")
        .y_label("Anzahl der möglichen Quersummen");

    Page::single(&view).save(format!("./Data/result_count_s{}-{}.svg", s_start, s_stop)).unwrap();
}

fn count_cross_sums_per_s(start: usize, stop: usize) {
	let mut cross_sums_per_s: HashMap<u64, u64> = HashMap::new();

    for s in start..stop {
        let k_vec: Vec<u8>  = vec![1; s];
        let m_vec: Vec<u8> = vec![4; s];

        let mut cross_sums: Vec<BigUint> = vec![];

        println!("s = {}", s);

        for i in 0..k_vec.len() {
            let mut n_vec: Vec<u8>  = vec![4; s];
            n_vec[i] = 5;

            let k: BigUint = concat(&k_vec);
            let m: BigUint = concat(&m_vec);
            let n: BigUint = concat(&n_vec);

            let result: BigUint = calc_formula(&k, &m, &n);
            let cross_sum: BigUint = calc_cross_sum(&result);

            if !cross_sums.contains(&cross_sum) {
                println!("{}", cross_sum);
                cross_sums.push(cross_sum);
            }
        }

        if s < stop -1 {
            println!("");
        }

        cross_sums_per_s.insert(s as u64, cross_sums.len() as u64);
    }

    let cross_sums_per_s_json: HashMap<String, u64> = HashMap::from_iter(
        cross_sums_per_s.clone().into_iter()
        .map(| (key, value)| (key.to_string(), value))
    );
    fs::write(format!("./Data/result_count_s{}-{}.json", start, stop), &serde_json::to_string(&cross_sums_per_s_json).unwrap()).expect("Unable to write file");

    let plot_data = Vec::from_iter(cross_sums_per_s
        .into_iter()
        .map(|(key, value)| ((key as f64) , (value as f64)))
    );
    plot(plot_data, start, stop);
}

fn find_cross_sums_per_s(start: usize, stop: usize) {
    let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("./Data/result_s{}-{}.txt", start, stop))
            .expect("Unable to open file");

    for s in start..stop {
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
            println!("Cross sum(c) = {}", cross_sum);

            if s < stop -1 || ( s == stop -1 && i < k_vec.len() -1) {
                println!("");
            }

            writeln!(file, "s = {}", s).expect("TODO");
            writeln!(file, "c = {}² - {}² + {} = {}", n, m, k, &result).expect("TODO");
            writeln!(file, "Cross sum(c) = {}", cross_sum).expect("TODO");
            writeln!(file, "").expect("TODO");
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
