pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let x_num = garden[0].len();
    let y_num = garden.len();

    let result = garden
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &ch)| {
                    if ch == b'*' {
                        '*'
                    } else {
                        let x_start = if x == 0 { 0 } else { x - 1 };
                        let x_end = if x == x_num - 1 { x_num - 1 } else { x + 1 };
                        let y_start = if y == 0 { 0 } else { y - 1 };
                        let y_end = if y == y_num - 1 { y_num - 1 } else { y + 1 };

                        let mut count = 0;
                        for dy in y_start..=y_end {
                            for dx in x_start..=x_end {
                                if garden[dy].as_bytes()[dx] == b'*' {
                                    count += 1;
                                }
                            }
                        }
                        if count > 0 {
                            (b'0' + count as u8) as char
                        } else {
                            ' '
                        }
                    }
                })
                .collect()
        })
        .collect();

    println!("result is {:?}", result);
    result
}
