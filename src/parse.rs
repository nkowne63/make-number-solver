use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

pub(crate) fn prompt_input() -> (Vec<u32>, u32) {
    println!("Input numbers >");
    let input = get_input()
        .split(' ')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect();
    println!("Input target >");
    let target = get_input().trim().parse::<u32>().unwrap();
    (input, target)
}
