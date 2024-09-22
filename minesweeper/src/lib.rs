pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");

    // index grid size
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }
    let cols = minefield[0].len();

    // declare annotated board
    let mut annotated = Vec::new();

    let mut row_counter: usize = 0;

    // iterate over rows within minefield
    for row in minefield.iter() {
        let mut annotated_row = String::new();

        // iterate over cells within row
        for (i, cell) in row.as_bytes().iter().enumerate() {
            let is_mine = *cell == b'*';

            if !is_mine {
                annotated_row.push_str(&get_mine_count(minefield, row_counter, i as usize));
            } else {
                annotated_row.push(*cell as char);
            }
        }
        annotated.push(annotated_row);
        row_counter += 1;
    }
    annotated
}

fn get_mine_count(minefield: &[&str], cell_row: usize, cell_col: usize) -> String {
    let rows = minefield.len();
    let cols = minefield[0].len();

    // declare counter
    let mut mines_count = 0;

    // Check the adjacent cells
    for i in -1..=1 {
        let row_index = (cell_row as i32 + i) as usize;
        if row_index < rows {
            for j in -1..=1 {
                let new_col = (cell_col as i32 + j) as usize;
                if new_col < cols {
                    if minefield[row_index as usize].as_bytes()[new_col] == b'*' {
                        mines_count += 1;
                    }
                }
            }
        }
    }
    if mines_count != 0 {
        format!("{}", mines_count)
    } else {
        String::from(" ")
    }
}
