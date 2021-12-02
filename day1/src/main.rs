use std::fs;

fn main() {
    let data = fs::read_to_string("sample.txt").expect("Unable to read file");
    let mut numbers = data.lines();
    let mut n_last : i32 = numbers.next().unwrap().trim().parse().expect("wrong first number!");
    let mut n_tot : i32 = 0;
    for n in numbers {
        let n_int : i32 = n.trim().parse().expect("wrong number!");
        println!("{}", n_int);
        let inc = n_int > n_last;
        if inc {
            println!("(increased)");
            n_tot = n_tot + 1;
        }
        n_last = n_int;
    }
    println!("{}", n_tot);

}
