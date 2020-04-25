pub fn is_armstrong_number(num: u32) -> bool {
    let no_digits = ((num as f32).log10() as u32) + 1;
    let mut n = num;
    let mut sum = 0;
    while n != 0 {
        sum += (n % 10).pow(no_digits);
        n = n / 10;
    }
    num == sum
}
