pub fn annotate(garden: &[&str]) -> Vec<String> {
    let height = garden.len();
    if height == 0 {
        return vec![];
    }
    let width = garden[0].len();

    let mut result = Vec::with_capacity(height);

    for row in 0..height {
        let mut annotated_row = String::with_capacity(width);
        let row_bytes = garden[row].as_bytes();

        for col in 0..width {
            if row_bytes[col] == b'*' {
                annotated_row.push('*');
                continue;
            }

            let mut count = 0;
            for dy in [-1isize, 0, 1] {
                for dx in [-1isize, 0, 1] {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    let ny = row as isize + dy;
                    let nx = col as isize + dx;
                    if ny >= 0
                        && ny < height as isize
                        && nx >= 0
                        && nx < width as isize
                        && garden[ny as usize].as_bytes()[nx as usize] == b'*'
                    {
                        count += 1;
                    }
                }
            }

            if count == 0 {
                annotated_row.push(' ');
            } else {
                annotated_row.push((b'0' + count) as char);
            }
        }

        result.push(annotated_row);
    }

    result
}
