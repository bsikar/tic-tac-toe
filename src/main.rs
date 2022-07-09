use macroquad::prelude::*;

const WIDTH: f32 = 600.0;
const HEIGHT: f32 = 600.0;
const SYMBOL_SIZE: f32 = 350.0;
const TEXT_SIZE: f32 = 75.0;

#[derive(PartialEq, Copy, Clone)]
enum Player {
    None,
    Player1,
    Player2,
}

struct TicTacToe {
    board: [[Player; 3]; 3],
    winner: Player,
    turn: Player,
}

impl TicTacToe {
    fn new() -> TicTacToe {
        rand::srand(macroquad::miniquad::date::now() as _);
        TicTacToe {
            board: [[Player::None; 3]; 3],
            winner: Player::None,
            turn: Player::Player1,
        }
    }

    fn draw(&self) {
        // boarder
        for i in 0..3 {
            for j in 0..3 {
                let x = (WIDTH / 3.0) * j as f32;
                let y = (HEIGHT / 3.0) * i as f32;
                let w = WIDTH / 3.0;
                let h = HEIGHT / 3.0;

                draw_rectangle_lines(x, y, w, h, 10.0, GRAY);
            }
        }

        // draw x or o
        for i in 0..3 {
            for j in 0..3 {
                let board = self.board[i][j];
                let text = match board {
                    Player::Player1 => "X",
                    Player::Player2 => "O",
                    Player::None => "",
                };

                let text_size = measure_text(text, None, SYMBOL_SIZE as u16, 1.0);
                let x = WIDTH * j as f32 / 3.0;
                let y = HEIGHT * i as f32 / 3.0;
                let w = WIDTH / 3.0;
                let h = HEIGHT / 3.0;
                let text_x = x + w - w / 2.0 - text_size.width / 1.75;
                let text_y = y + h - h / 2.0 + text_size.height / 2.0;

                if board == Player::Player1 {
                    draw_text(text, text_x, text_y, SYMBOL_SIZE, LIGHTGRAY);
                } else if board == Player::Player2 {
                    draw_text(text, text_x, text_y, SYMBOL_SIZE, GRAY);
                }
            }
        }
    }

    fn draw_winner(&self) {
        if self.turn == Player::None {
            let text = match self.winner {
                Player::Player1 => "Player 1 wins!",
                Player::Player2 => "Player 2 wins!",
                Player::None => "Draw!",
            };

            let text_size = measure_text(text, None, TEXT_SIZE as u16, 1.0);
            let x = WIDTH / 2.0 - text_size.width / 2.0;
            let y = HEIGHT / 2.0 + text_size.height / 2.0;

            draw_rectangle(
                0.0,
                HEIGHT / 3.0,
                WIDTH,
                HEIGHT / 3.0,
                Color::new(0.0, 0.0, 0.0, 0.5),
            );
            draw_text(text, x, y, TEXT_SIZE, ORANGE);
        }
    }

    fn play(&mut self) {
        self.draw();
        self.draw_winner();
        if self.turn == Player::None {
            return;
        }

        self.make_move();
        self.check_win();
    }

    fn make_move(&mut self) {
        for i in 0..3 {
            for j in 0..3 {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let mouse_position = mouse_position();

                    let x = (WIDTH / 3.0) * j as f32;
                    let y = (HEIGHT / 3.0) * i as f32;
                    let w = WIDTH / 3.0;
                    let h = HEIGHT / 3.0;

                    if mouse_position.0 > x
                        && mouse_position.0 < x + w
                        && mouse_position.1 > y
                        && mouse_position.1 < y + h
                        && self.board[i][j] == Player::None
                    {
                        if self.turn == Player::Player1 {
                            self.board[i][j] = Player::Player1;
                            self.turn = Player::Player2;
                        } else {
                            self.board[i][j] = Player::Player2;
                            self.turn = Player::Player1;
                        }
                    }
                }
            }
        }
    }

    fn check_win(&mut self) {
        for i in 0..3 {
            // check rows
            if self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
                && self.board[i][0] != Player::None
            {
                self.winner = self.board[i][0];
                self.turn = Player::None;
            }
            // check columns
            if self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
                && self.board[0][i] != Player::None
            {
                self.winner = self.board[0][i];
                self.turn = Player::None;
            }
        }
        // check diagonals
        if self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
            && self.board[0][0] != Player::None
        {
            self.winner = self.board[0][0];
            self.turn = Player::None;
        }
        if self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
            && self.board[0][2] != Player::None
        {
            self.winner = self.board[0][2];
            self.turn = Player::None;
        }
        // if no winner
        if self
            .board
            .iter()
            .all(|x| x.iter().all(|y| *y != Player::None))
        {
            self.winner = Player::None;
            self.turn = Player::None;
        }
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut tictactoe = TicTacToe::new();

    loop {
        tictactoe.play();
        if is_key_down(KeyCode::R) {
            tictactoe = TicTacToe::new();
        }
        next_frame().await;
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}
