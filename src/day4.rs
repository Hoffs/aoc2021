use crate::aoc::AocSolution2;

#[derive(Debug)]
struct Board {
    cells: Vec<Vec<i32>>,
    marked: Option<Vec<Vec<bool>>>,
    marked_count: i32,
    won: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            cells: Vec::new(),
            marked: None,
            marked_count: 0,
            won: false,
        }
    }

    fn finish_setup(self: &mut Self) {
        let mut marked_row = Vec::with_capacity(self.cells.first().unwrap().len());
        marked_row.resize(marked_row.capacity(), false);
        let mut marked = Vec::with_capacity(self.cells.len());
        marked.resize_with(marked.capacity(), || marked_row.clone());
        self.marked = Some(marked)
    }

    fn mark(self: &mut Self, num: &i32) -> bool {
        if self.won {
            return false;
        }

        let marked_ref = self.marked.as_mut().unwrap();
        for (row_idx, row) in self.cells.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if cell == num {
                    marked_ref[row_idx][col_idx] = true;
                    self.marked_count += 1;

                    if self.marked_count >= row.len().try_into().unwrap()
                        || self.marked_count >= self.cells.len().try_into().unwrap()
                    {
                        // check if won
                        return self.won();
                    }
                }
            }
        }

        false
    }

    fn won(self: &mut Self) -> bool {
        let marked_ref = self.marked.as_ref().unwrap();
        for row in marked_ref {
            if row.iter().all(|x| *x) {
                self.won = true;
                return true
            }
        }

        'outer: for i in 0..marked_ref.len() {
            for row in marked_ref {
                if !row[i] {
                    continue 'outer;
                }
            }

            self.won = true;
            return true
        }

        false
    }

    fn sum(self: &Self) -> i32 {
        let mut sum = 0;
        let marked_ref = self.marked.as_ref().unwrap();
        for (row_idx, row) in marked_ref.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if !cell {
                    sum += self.cells[row_idx][col_idx];
                }
            }
        }

        sum
    }
}

fn p1(input: &str) -> String {
    let mut lines = input.lines();
    let picks: Vec<i32> = lines.next().unwrap_or_default().split(',').map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();

    let mut board: Board = Board::new();
    for line in lines {
        if line.is_empty() {
            // new board
            if board.cells.len() > 0 {
                board.finish_setup();
                boards.push(board);
            }
            board = Board::new();
        } else {
            board.cells.push(
                line.split_whitespace()
                    .map(|c| c.parse().unwrap_or_default())
                    .collect(),
            )
        }
    }

    if board.cells.len() > 0 {
        board.finish_setup();
        boards.push(board);
    }

    for pick in picks {
        for board in &mut boards {
            if board.mark(&pick) {
                return (board.sum() * pick).to_string()
            }
        }
    }

    "".to_string()
}

fn p2(input: &str) -> String {
    let mut lines = input.lines();
    let picks: Vec<i32> = lines.next().unwrap_or_default().split(',').map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();

    let mut board: Board = Board::new();
    for line in lines {
        if line.is_empty() {
            // new board
            if board.cells.len() > 0 {
                board.finish_setup();
                boards.push(board);
            }
            board = Board::new();
        } else {
            board.cells.push(
                line.split_whitespace()
                    .map(|c| c.parse().unwrap_or_default())
                    .collect(),
            )
        }
    }

    if board.cells.len() > 0 {
        board.finish_setup();
        boards.push(board);
    }

    let mut winning_sum = None;
    for pick in picks {
        for board in &mut boards {
            if board.mark(&pick) {
                winning_sum = Some(board.sum() * pick)
            }
        }
    }

    winning_sum.unwrap_or_default().to_string()
}

pub static SOLUTION: AocSolution2 = AocSolution2 {
    year: 2021,
    day: 4,
    solve_p1: p1,
    solve_p2: p2,
};
