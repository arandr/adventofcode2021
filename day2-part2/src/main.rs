use std::fs;

struct Position {
    horizontal: i32,
    depth: i32,
    aim : i32,
}

fn build_position(x: i32, y: i32, w: i32) -> Position {
    return Position {
        horizontal: x,
        depth: y,
        aim: w,
    };
}

fn advance(position: Position, command: &str) -> Position {
    let direction: &str = command.split(' ').next().unwrap();
    let value: i32 = command.split(' ').last().unwrap().parse().expect("wrong value!");
    
    return match direction {
        "forward" => build_position(position.horizontal + value, position.depth + position.aim * value, position.aim),
        "up" => build_position(position.horizontal, position.depth, position.aim - value),
        "down" => build_position(position.horizontal, position.depth, position.aim + value),
        _ => position,
    };
}

fn main() {
    let data = fs::read_to_string("sample.txt").expect("Unable to read file");
    let instructions = data.lines();
    let init_pos = Position {
        horizontal: 0,
        depth: 0,
        aim: 0
    };
    
    let final_pos: Position = instructions.fold(init_pos, advance);
    println!("depth: {}, pos: {}, aim: {}, total: {}", final_pos.depth, final_pos.horizontal, final_pos.aim, final_pos.depth * final_pos.horizontal);

}
