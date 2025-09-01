use core::panic;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(bottles: u32) -> String {
    let next_bottles = if bottles > 0 { bottles - 1 } else { 0 };

    let current = number_to_word(bottles);
    let next = number_to_word(next_bottles);

    let bottle_word = |n: u32| if n == 1 { "bottle" } else { "bottles" };

    format!(
        "{} green {} hanging on the wall,\n\
         {} green {} hanging on the wall,\n\
         And if one green bottle should accidentally fall,\n\
         There'll be {} green {} hanging on the wall.",
        capitalize(current),
        bottle_word(bottles),
        capitalize(current),
        bottle_word(bottles),
        next,
        bottle_word(next_bottles)
    )
}

fn number_to_word(n: u32) -> &'static str {
    match n {
        0 => "no",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => panic!("Invalid number"),
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + &s[1..],
    }
}
