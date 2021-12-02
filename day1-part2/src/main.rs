use std::fs;

fn main() {
    let data = fs::read_to_string("sample.txt").expect("Unable to read file");
    let mut numbers : Vec<i32> = data.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut n_tot : i32 = 0;
    let mut n_last : i32 = 0;
    for n in 0..numbers.len()-2 {
        let slice = &numbers[n..n+3];
        let n_sum = slice.iter().sum();
        println!("{}", n_sum);
        let inc = n > 0 && n_sum > n_last;
        if inc {
            println!("(increased)");
            n_tot = n_tot + 1;
        }
        n_last = n_sum;
    }
    println!("{}", n_tot);
}
