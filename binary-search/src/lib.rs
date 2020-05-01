pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut size = array.len();
    if size == 0 {
        return None;
    }
    let mut base = 0_usize;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let val = array[mid];
        if val == key {
            base = mid;
            break;
        }
        if val < key {
            base = mid;
        }
        size -= half;
    }

    if array[base] == key {
        Some(base)
    } else {
        None
    }
}
