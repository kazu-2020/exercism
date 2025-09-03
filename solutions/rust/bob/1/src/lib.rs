pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "Fine. Be that way!";
    }

    if is_question(message) && is_yelling(message) {
        return "Calm down, I know what I'm doing!";
    }
    if is_yelling(message) {
        return "Whoa, chill out!";
    }
    if is_question(message) {
        return "Sure.";
    }

    "Whatever."
}

fn is_question(message: &str) -> bool {
    message.ends_with('?')
}

fn is_yelling(message: &str) -> bool {
    message.to_uppercase() == message
}
