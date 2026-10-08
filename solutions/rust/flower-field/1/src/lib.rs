pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return Vec::new();
    }

    let rows = garden.len();
    let cols = garden[0].len();
    let mut result: Vec<String> = Vec::with_capacity(rows);

    for row in 0..rows {
        let mut row_result = String::with_capacity(cols);
        for col in 0..cols {
            if garden[row].chars().nth(col) == Some('*') {
                row_result.push('*');
            } else {
                let mut count = 0;
                // Check all 8 adjacent cells
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue; // Skip current cell
                        }
                        let new_row = row as i32 + dr;
                        let new_col = col as i32 + dc;
                        if new_row >= 0 && new_row < rows as i32 && 
                           new_col >= 0 && new_col < cols as i32 {
                            if garden[new_row as usize].chars().nth(new_col as usize) == Some('*') {
                                count += 1;
                            }
                        }
                    }
                }
                if count > 0 {
                    row_result.push_str(&count.to_string());
                } else {
                    row_result.push(' ');
                }
            }
        }
        result.push(row_result);
    }

    result
}
