pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let plant = |c| match c {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => unreachable!(),
    };

    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let student_index = students.iter().position(|&s| s == student).unwrap();
    diagram
        .lines()
        .flat_map(|line| {
            let start = student_index * 2;
            let end = start + 1;

            line[start..=end].chars().collect::<Vec<_>>()
        })
        .map(plant)
        .collect()
}
