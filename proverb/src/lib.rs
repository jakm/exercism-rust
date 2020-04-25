pub fn build_proverb(list: &[&str]) -> String {
    let mut out = vec![];
    let n = list.len();
    if n >= 2 {
        let mut first = list[0];
        for i in 1..n {
            let second = list[i];
            out.push(format!("For want of a {} the {} was lost.", first, second));
            first = second;
        }
    }
    if n >= 1 {
        out.push(format!("And all for the want of a {}.", list[0]));
    }
    out.join("\n")
}
