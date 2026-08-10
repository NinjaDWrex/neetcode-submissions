use std::collections::HashSet;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // rowset
        let mut row_sets : Vec<HashSet<char>> = vec![HashSet::new(); 9];
        // columnset
        let mut col_sets : Vec<HashSet<char>> = vec![HashSet::new(); 9];
        // gridset
        let mut grid_sets : Vec<HashSet<char>> = vec![HashSet::new(); 9];
        
        for (i,r) in board.into_iter().enumerate() {
            for (it,c) in r.iter().enumerate() {
                if c == &'.' {
                    continue;
                }
                if row_sets[i].contains(&c) || col_sets[it].contains(&c) || grid_sets[i/3 * 3 + it/3].contains(&c) {
                    return false;
                }
                // add number to sets
                row_sets[i].insert(*c);
                col_sets[it].insert(*c);
                grid_sets[i/3 * 3 + it/3].insert(*c);
            }
        }
        return true;
    }
}
