static CORRECT_USAGE_TEXT: &'static str =
    "Correct usage: ducci n_1 n_2 .. n_N\nWhere n_1, n_2, .. , n_N are N positive integers.";

fn main() {
    let mut current: Vec<u64> = std::env::args()
        .skip(1)
        .map(|arg| match arg.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Invalid argument: {}", arg);
                eprintln!("{}", CORRECT_USAGE_TEXT);
                std::process::exit(2)
            }
        })
        .collect();

    if current.is_empty() {
        eprintln!("{}", CORRECT_USAGE_TEXT);
        std::process::exit(2)
    }

    let mut visited = std::collections::HashSet::new();
    while !visited.contains(&current) {
        println!("{:?}", current);
        let mut next_step = Vec::with_capacity(current.len());
        for i in 0..current.len() {
            let left = current[i];
            let right = current[(i + 1) % current.len()];
            next_step.push(if left < right { right - left } else { left - right })
        }
        visited.insert(current);
        current = next_step;
    }

    println!("{} steps", visited.len());
}
