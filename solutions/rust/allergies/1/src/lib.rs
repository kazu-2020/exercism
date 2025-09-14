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

impl Allergen {
    fn all() -> Vec<Allergen> {
        vec![
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

    fn bit_value(&self) -> u32 {
        match self {
            Allergen::Eggs => 1 << 0,
            Allergen::Peanuts => 1 << 1,
            Allergen::Shellfish => 1 << 2,
            Allergen::Strawberries => 1 << 3,
            Allergen::Tomatoes => 1 << 4,
            Allergen::Chocolate => 1 << 5,
            Allergen::Pollen => 1 << 6,
            Allergen::Cats => 1 << 7,
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & allergen.bit_value()) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all()
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
