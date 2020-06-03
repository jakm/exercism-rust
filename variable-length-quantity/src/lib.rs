#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = Vec::new();

    for v in values.iter() {
        let mut v = *v;

        if v <= 0x7f {
            res.push(v as u8);
            continue;
        }

        let mut tmp = Vec::new();
        for i in 0.. {
            if v == 0 {
                break;
            }
            let mut b = (v & 0x7f) as u8;
            if i > 0 {
                b |= 0x80;
            }
            tmp.push(b);
            v >>= 7;
        }
        tmp.reverse();
        res.extend_from_slice(&tmp);
    }

    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = Vec::new();

    let mut complete = false;
    let mut tmp: u32 = 0;

    for &b in bytes {
        complete = false;
        tmp = checked_shl(tmp, 7).ok_or(Error::Overflow)?;
        tmp |= b as u32 & 0x7f;
        if b & 0x80 == 0 {
            res.push(tmp);
            complete = true;
            tmp = 0;
        }
    }

    if !complete {
        return Err(Error::IncompleteNumber);
    }

    Ok(res)
}

fn checked_shl(mut number: u32, shift: u8) -> Option<u32> {
    let ones = number.count_ones();
    number <<= shift;
    if ones != number.count_ones() {
        None
    } else {
        Some(number)
    }
}
