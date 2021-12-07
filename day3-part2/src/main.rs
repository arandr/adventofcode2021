use std::fs;

fn update_counts(mut v: Vec<i32>, entry: &str) -> Vec<i32> {
    for (pos, c) in entry.chars().enumerate() {
        v[pos] = v[pos] + (c.to_digit(2).unwrap() as i32)
    }
    return v;
}

fn update_counts2(mut v: Vec<i32>, entry: &&str) -> Vec<i32> {
    for (pos, c) in entry.chars().enumerate() {
        v[pos] = v[pos] + (c.to_digit(2).unwrap() as i32)
    }
    return v;
}

fn filter_values(v: Vec<i32>, mut values: Vec<&str>, size: usize, total: i32, invert: bool) -> i32 {
    let mut final_int = 0;
    let mut counts = v;
    let mut current_total = total;
    for pos in 0..size {
        println!("counts: {:?}", counts);
        let val = counts[pos];
        let mut bit: u32 = match val {
            val if val == (current_total / 2) && (current_total % 2 == 0) => 1,
            val if val > (current_total / 2) => 1,
            _ => 0,
        };
        if invert {
            bit = match bit {
                1 => 0,
                _ => 1,
            }
        }
        println!("{}: {}", size-1-pos, bit);
        let lval = u32::from_str_radix(values.last().unwrap(), 2).expect("wrong number");
        println!("{:#b} {:#b} {:#b}", lval, lval << (31 -(size-1-pos)), lval << (31 -(size-1-pos)) >> 31);
        values.retain(|&x| u32::from_str_radix(x, 2).expect("wrong number") << (31 - (size-1-pos)) >> 31 == bit);
        println!("left: {}", values.len());
        counts = vec![0; size];
        counts = values.iter().fold(counts, update_counts2);
        current_total = values.len() as i32;
        if values.len() == 1 {
            final_int = i32::from_str_radix(values.last().unwrap(), 2).expect("wrong number");
            break;
        }
    }
    println!("final {:#b}", final_int); 
    return final_int;
}

fn main() {
    let data = fs::read_to_string("sample.txt").expect("Unable to read file");
    let values = data.lines();
    let total = values.clone().count().try_into().unwrap();
    let size = values.clone().last().unwrap().chars().count();
    let mut counts: Vec<i32> = vec![0; size];
    
    counts = values.clone().fold(counts, update_counts);
    println!("counts: {:?} total {}", counts, total);
    let oxygen = filter_values(counts.clone(), values.clone().collect(), size, total, false);
    let co2 = filter_values(counts.clone(), values.clone().collect(), size, total, true);    
    println!("oxygen: {}, co2: {}, total: {}", oxygen, co2, oxygen*co2);

}
