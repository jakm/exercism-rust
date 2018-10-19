pub fn raindrops(n: usize) -> String {
    let mut output = String::new();
    for i in 1..n+1 {
        if n % i == 0 {
            match i {
                3 => output.push_str("Pling"),
                5 => output.push_str("Plang"),
                7 => output.push_str("Plong"),
                _ => (),
            }
        }
    }

    if output.len() > 0 {
        output
    } else {
        n.to_string()
    }
}
