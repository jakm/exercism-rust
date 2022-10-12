pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let n = match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        };

        n & self.score > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .map(|n| 2_u32.pow(n))
            .filter_map(|n| {
                if n & self.score > 0 {
                    Some(match n {
                        1 => Allergen::Eggs,
                        2 => Allergen::Peanuts,
                        4 => Allergen::Shellfish,
                        8 => Allergen::Strawberries,
                        16 => Allergen::Tomatoes,
                        32 => Allergen::Chocolate,
                        64 => Allergen::Pollen,
                        128 => Allergen::Cats,
                        _ => unimplemented!(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
