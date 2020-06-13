/// What should the type of _function be?
pub fn map<T, U, F>(input: Vec<T>, mut func: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut output = Vec::new();
    for item in input {
        output.push(func(item));
    }
    output
}
