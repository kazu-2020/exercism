pub fn reply(message: &str) -> &str {
    let message = message.trim();

    match message.trim() {
        "" => "Fine. Be that way!",
        _ if is_yelling(message) && is_question(message) => "Calm down, I know what I'm doing!",
        _ if is_yelling(message) => "Whoa, chill out!",
        _ if is_question(message) => "Sure.",
        _ => "Whatever.",
    }
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message && message.chars().any(|c| c.is_alphabetic())
}
