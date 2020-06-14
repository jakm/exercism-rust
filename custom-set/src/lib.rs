#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T> CustomSet<T>
where
    T: std::cmp::Ord + std::clone::Clone + std::fmt::Debug,
{
    pub fn new(input: &[T]) -> Self {
        let mut s = CustomSet { items: Vec::new() };
        s.items.extend_from_slice(input);
        s.items.sort();
        s
    }

    pub fn contains(&self, element: &T) -> bool {
        match self.items.binary_search(element) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn add(&mut self, element: T) {
        if let Err(idx) = self.items.binary_search(&element) {
            self.items.insert(idx, element)
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.is_empty() {
            return true;
        }
        if let Ok(idx) = other.items.binary_search(&self.items[0]) {
            if self.len() > other.len() - idx {
                return false;
            }

            for i in 0..self.len() {
                if self.items[i] != other.items[idx + i] {
                    return false;
                }
            }
            return true;
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.is_empty() || other.is_empty() {
            return true;
        }
        let mut idx = 0_usize;
        for item_a in &self.items {
            while idx < other.len() {
                let item_b = &other.items[idx];
                if *item_a == *item_b {
                    return false;
                } else if *item_a > *item_b {
                    idx += 1;
                    continue;
                }
                break;
            }
        }
        true
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut items;

        if self.is_empty() {
            items = self.items.iter().cloned().collect();
        } else if other.is_empty() {
            items = other.items.iter().cloned().collect();
        } else {
            items = Vec::new();
            let mut idx = 0_usize;
            for item_a in &self.items {
                while idx < other.len() {
                    let item_b = &other.items[idx];
                    if *item_a == *item_b {
                        items.push(item_a.clone());
                        idx += 1;
                        break;
                    } else if *item_a > *item_b {
                        idx += 1;
                        continue;
                    } else {
                        // B > A - keep idx unchanged and load next item (A) from self
                        break;
                    }
                }
            }
        }

        CustomSet { items }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut items;

        if self.is_empty() {
            items = Vec::new();
        } else if other.is_empty() {
            items = self.items.iter().cloned().collect();
        } else {
            items = Vec::new();
            let mut start = 0_usize;
            for item_a in &self.items {
                start = match other.items[start..].binary_search(item_a) {
                    Ok(idx) => idx,
                    Err(idx) => {
                        items.push(item_a.clone());
                        idx
                    }
                }
            }
        }

        CustomSet { items }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut items: Vec<T> = self.items.iter().cloned().collect();
        items.extend(other.items.iter().cloned());
        items.sort();
        items.dedup();
        CustomSet { items }
    }
}
