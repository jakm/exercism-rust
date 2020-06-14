#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list.is_empty() {
        if second_list.is_empty() {
            return Comparison::Equal;
        } else {
            return Comparison::Sublist;
        }
    } else {
        if second_list.is_empty() {
            return Comparison::Superlist;
        }
    }

    let c;

    if first_list.len() < second_list.len() {
        c = if second_list
            .windows(first_list.len())
            .any(|w| w == first_list)
        {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else if first_list.len() > second_list.len() {
        c = if first_list
            .windows(second_list.len())
            .any(|w| w == second_list)
        {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    } else {
        c = if first_list == second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    }

    c
}
