use std::fs;

fn update_counts(mut v: Vec<i32>, entry: &str) -> Vec<i32> {
    for (pos, c) in entry.chars().enumerate() {
        v[pos] = v[pos] + (c.to_digit(2).unwrap() as i32)
    }
    return v;
}

fn most_common_bits(v: &Vec<i32>, total: i32, len: usize) -> u32 {
    let mut final_int: u32 = 0;
    for (pos, val) in v.iter().enumerate() {
        let bit: u32 = match val {
            val if val > &(total / 2) => 1,
            _ => 0,
        };
        println!("{}: {}", bit, pos);
        final_int = final_int | ( bit << (len-pos) );
    }
    return final_int;
}

fn main() {
    let data = fs::read_to_string("sample.txt").expect("Unable to read file");
    let values = data.lines();
    let total = values.clone().count();
    let size = values.clone().last().unwrap().chars().count();
    let mut counts: Vec<i32> = vec![0; size];
    
    counts = values.fold(counts, update_counts);
    println!("counts: {:?} total {}", counts, total);
    let gamma: u32 = most_common_bits(&counts, total.try_into().unwrap(), size-1);
    let epsilon: u32 = !gamma << (32 - size) >> (32 - size);
    println!("epsilon: {:#b}, gamma: {:#b}, total: {}", epsilon, gamma, epsilon*gamma);

}
