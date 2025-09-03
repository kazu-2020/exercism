pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .flat_map(|&factor| magical_item_multiples(limit, factor))
        .collect::<std::collections::HashSet<u32>>()
        .into_iter()
        .sum()
}

fn magical_item_multiples(limit: u32, base: u32) -> Vec<u32> {
    let mut multiples = Vec::new();
    let mut current = base;

    while current < limit {
        multiples.push(current);
        current += base;
    }

    multiples
}
