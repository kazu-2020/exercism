#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    use Comparison::*;

    match (first_list.len(), second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => {
            if contains(first_list, second_list) {
                Superlist
            } else {
                Unequal
            }
        }
        (m, n) if m < n => {
            if contains(second_list, first_list) {
                Sublist
            } else {
                Unequal
            }
        }
        _ => {
            if first_list == second_list {
                Equal
            } else {
                Unequal
            }
        }
    }
}

fn contains(a: &[i32], b: &[i32]) -> bool {
    a.windows(b.len()).any(|window| window == b)
}
