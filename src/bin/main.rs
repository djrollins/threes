use rand::{seq::SliceRandom, thread_rng, Rng};
use std::fmt;
use std::io;
use std::io::Write;
use std::ops::{Index, IndexMut};

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

type Card = usize;

#[derive(Clone)]
struct Board {
    cells: [Card; 16],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: Default::default(),
        }
    }

    fn set_random_empty_cell<R>(&mut self, mut rng: R, card: Card)
    where
        R: Rng,
    {
        loop {
            let cell = self.cells.choose_mut(&mut rng).unwrap();
            if *cell == 0 {
                *cell = card;
                break;
            }
        }
    }
}

impl Index<usize> for Board {
    type Output = [Card];

    fn index(&self, index: usize) -> &Self::Output {
        let row = 4 * index;
        &self.cells[row..(row + 4)]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let row = 4 * index;
        &mut self.cells[row..(row + 4)]
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Board: {{\n\r")?;
        for row in 0..4 {
            let row = &self[row];
            write!(
                fmt,
                "  | {:^5} | {:^5} | {:^5} | {:^5} |\n\r",
                row[0], row[1], row[2], row[3]
            )?;
        }
        write!(fmt, "}}")?;

        Ok(())
    }
}

fn merge(fst: Card, snd: Card) -> Option<Card> {
    match (fst, snd) {
        (x, 0) => Some(x),
        (1, 1) | (2, 2) => None,
        (1, 2) | (2, 1) => Some(3),
        (x, y) if (x == y) => Some(x * 2),
        _ => None,
    }
}

fn resolve_left<R>(board: &Board, mut rng: R, new_card: Card) -> Option<Board>
where
    R: Rng,
{
    let mut board = board.clone();
    let mut changed_rows = Vec::new();

    for row in 0..4 {
        let mut row_changed = false;
        for cell in 0..3 {
            if let Some(new) = merge(board[row][cell + 1], board[row][cell]) {
                row_changed = true;
                board[row][cell] = new;
                board[row][cell + 1] = 0;
            }
        }

        if row_changed {
            changed_rows.push(row)
        }
    }

    changed_rows.choose(&mut rng).map(|&row| {
        board[row][3] = new_card;
        board
    })
}

fn resolve_right<R>(board: &Board, mut rng: R, new_card: Card) -> Option<Board>
where
    R: Rng,
{
    let mut board = board.clone();
    let mut changed_rows = Vec::new();

    for row in 0..4 {
        let mut row_changed = false;
        for cell in (0..3).rev() {
            if let Some(new) = merge(board[row][cell], board[row][cell + 1]) {
                row_changed = true;
                board[row][cell] = 0;
                board[row][cell + 1] = new;
            }
        }

        if row_changed {
            changed_rows.push(row)
        }
    }

    changed_rows.choose(&mut rng).map(|&row| {
        board[row][0] = new_card;
        board
    })
}

fn rotate_90_clockwise(board: &Board) -> Board {
    Board {
        #[rustfmt::skip]
        cells: [
            board[3][0], board[2][0], board[1][0], board[0][0],
            board[3][1], board[2][1], board[1][1], board[0][1],
            board[3][2], board[2][2], board[1][2], board[0][2],
            board[3][3], board[2][3], board[1][3], board[0][3],
        ],
    }
}

fn rotate_90_counter_clockwise(board: &Board) -> Board {
    Board {
        #[rustfmt::skip]
        cells: [
            board[0][3], board[1][3], board[2][3], board[3][3],
            board[0][2], board[1][2], board[2][2], board[3][2],
            board[0][1], board[1][1], board[2][1], board[3][1],
            board[0][0], board[1][0], board[2][0], board[3][0],
        ],
    }
}

fn resolve_up<R>(board: &Board, mut rng: R, new_card: Card) -> Option<Board>
where
    R: Rng,
{
    resolve_right(&rotate_90_clockwise(&board), &mut rng, new_card)
        .map(|board| rotate_90_counter_clockwise(&board))
}

fn resolve_down<R>(board: &Board, mut rng: R, new_card: Card) -> Option<Board>
where
    R: Rng,
{
    resolve_right(&rotate_90_counter_clockwise(&board), &mut rng, new_card)
        .map(|board| rotate_90_clockwise(&board))
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    let _stderr = stderr.lock();

    let mut stdout = stdout.into_raw_mode().unwrap();
    let mut stdin = stdin.keys();

    let mut rng = thread_rng();
    let mut basic_deck: [Card; 12] = [1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3];

    let mut board = Board::new();
    basic_deck.shuffle(&mut rng);
    let mut draw_pile = basic_deck.iter();

    for _ in 0..9 {
        board.set_random_empty_cell(&mut rng, *draw_pile.next().unwrap())
    }

    'main: loop {
        let next_card = {
            if let Some(next) = draw_pile.next() {
                *next
            } else {
                basic_deck.shuffle(&mut rng);
                draw_pile = basic_deck.iter();
                *draw_pile.next().unwrap()
            }
        };

        let resolved_up = resolve_up(&mut board, &mut rng, next_card);
        let resolved_left = resolve_left(&mut board, &mut rng, next_card);
        let resolved_right = resolve_right(&mut board, &mut rng, next_card);
        let resolved_down = resolve_down(&mut board, &mut rng, next_card);
        let lose = [
            &resolved_down,
            &resolved_right,
            &resolved_left,
            &resolved_up,
        ]
        .iter()
        .all(|x| x.is_none());

        if lose {
            write!(stdout, "you lose!")?;
            break;
        }

        write!(
            stdout,
            "{}{}{:?}\n\r",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            board
        )?;
        write!(stdout, "next card: {}\n\r", next_card)?;

        use termion::event::Key::*;
        while let Some(Ok(key)) = stdin.next() {
            match key {
                Char('q') => break 'main,
                Char('h') | Char('a') | Left => {
                    if let Some(new_board) = resolved_left {
                        board = new_board;
                        break;
                    }
                }
                Char('j') | Char('s') | Down => {
                    if let Some(new_board) = resolved_down {
                        board = new_board;
                        break;
                    }
                }
                Char('k') | Char('w') | Up => {
                    if let Some(new_board) = resolved_up {
                        board = new_board;
                        break;
                    }
                }
                Char('l') | Char('d') | Right => {
                    if let Some(new_board) = resolved_right {
                        board = new_board;
                        break;
                    }
                }
                _ => continue,
            }
        }
    }

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::style::Reset,
        termion::cursor::Goto(1, 1)
    )
}
