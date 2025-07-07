pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs,         // 1
    Peanuts,      // 2
    Shellfish,    // 4
    Strawberries, // 8
    Tomatoes,     // 16
    Chocolate,    // 32
    Pollen,       // 64
    Cats,         // 128
}

impl Allergen {
    fn score(&self) -> u32 {
        match self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }

    fn all() -> &'static [Allergen] {
        &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies {
            score: score & 0xFF, // only keep the lowest 8 bits (0-255)
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & allergen.score() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all()
            .iter()
            .cloned()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
