use std::collections::LinkedList;
use std::io;
use std::time::Duration;
use crossterm::{cursor, event::{poll, read, Event, KeyCode, KeyEvent}, execute, terminal, ExecutableCommand};
extern crate rand;
use rand::Rng;

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction,
}

// Clears the game board
fn clear() {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

fn main() {
    // Initialize game elements
    clear();
    let mut snake = Snake {
        body: vec![(5, 5), (5, 6), (5, 7)].into_iter().collect(),
        direction: Direction::Right,
    };

    let mut food = (10, 10);
    let mut score = 0;
    let mut game_over = false;

    let board_width = 100;
    let board_height = 20;

    let duration_ms_default = 50;
    let mut duration_ms = duration_ms_default;
    
    // Draw initial game state
    draw_game(&snake, &food, score, &snake.direction, board_height, board_width);
    clear();

    // Game loop
    while !game_over {
        // Draw game board
        draw_game(&snake, &food, score, &snake.direction, board_height, board_width);

        // Handle player input
        let mut head = snake.body.front().unwrap().clone();

        // if head.0 > food.0 { snake.direction = change_direction(Direction::Left, snake.direction); }
        // else if head.0 < food.0 { snake.direction = change_direction(Direction::Right, snake.direction); }

        // else if head.1 > food.1 { snake.direction = change_direction(Direction::Up, snake.direction); }
        // else if head.1 < food.1 { snake.direction = change_direction(Direction::Down, snake.direction); }
        
        if poll(Duration::from_millis(duration_ms)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Up => snake.direction = change_direction(Direction::Up, snake.direction),
                    KeyCode::Down => snake.direction = change_direction(Direction::Down, snake.direction),
                    KeyCode::Left => snake.direction = change_direction(Direction::Left, snake.direction),
                    KeyCode::Right => snake.direction = change_direction(Direction::Right, snake.direction),
                    _ => ()
                }
            }
        }

        //  quickkkkkk
        if poll(Duration::from_millis(duration_ms)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() { 
                if code == KeyCode::Char('f') { 
                    duration_ms = 0; 
                }
            }
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() { 
                if code == KeyCode::Char('g') { 
                    duration_ms = duration_ms_default; 
                }
            }
        }

        // let ten_millis = time::Duration::from_millis(duration_ms);
        // let now = time::Instant::now();
        // thread::sleep(ten_millis);
        // assert!(now.elapsed() >= ten_millis);

        // Update snake position
        let mut head = snake.body.front().unwrap().clone();
        match snake.direction {
            Direction::Up => {
                if head.1 == 0 { head.1 = board_height; } // vertical
                else { head.1 -= 1; }
            },
            Direction::Down => {
                if head.1 == board_height-1 { head.1 = 0; } // vertical
                else { head.1 += 1; }
            },
            Direction::Left => {
                if head.0 == 0 { head.0 = board_width; } // horizontal
                else { head.0 -= 1; }
            },
            Direction::Right => {
                if head.0 == board_width-1 { head.0 = 0; } // horizontal
                else { head.0 += 1; }
            },
        }

        // Check for collisions
        if head == food {
            score += 1;
            // Generate new food location
            let food_x: i32 = rand::thread_rng().gen_range(0..board_width);
            let food_y: i32 = rand::thread_rng().gen_range(0..board_height);
            food = (food_x, food_y);
        } else {
            snake.body.pop_back();
        }

        if head.0 < 0 || head.0 >= board_width || head.1 < 0 || head.1 >= board_height || snake.body.contains(&head) {
            game_over = true;
        }

        snake.body.push_front(head);

        // Move cursor to the start of the game board
        execute!(io::stdout(), cursor::MoveTo(0, 1)).unwrap();

        // Draw updated game state
        draw_game(&snake, &food, score, &snake.direction, board_height, board_width);
    }

    // Display game over screen
    draw_game_over(score);
}

fn change_direction(new_direction: Direction, current_direction: Direction) -> Direction {
    // Don't make the snake go into itself
    if 
    current_direction == Direction::Up && new_direction == Direction::Down ||
    current_direction == Direction::Down && new_direction == Direction::Up ||
    current_direction == Direction::Left && new_direction == Direction::Right ||
    current_direction == Direction::Right && new_direction == Direction::Left  {
        
        return current_direction;
    } else {
        return new_direction;
		clear();
    }
}

fn draw_game(snake: &Snake, food: &(i32, i32), score: u32, direction: &Direction, board_height: i32, board_width: i32) {
    // Draw the game board
    for y in 0..board_height {
        for x in 0..board_width {
            let c = if (x, y) == *food {
                '$' // Snake body
            } else if snake.body.contains(&(x, y)) {
                'o' // Food
            } else {
                '.' // Empty space
            };
            print!("{}", c);
        }
        println!();
    }

    // Move cursor to the bottom to draw the score
    execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();
    // Draw the score and direction 
    println!("Score: {} Direction: {:?} - - - -", score, direction);
}

fn draw_game_over(score: u32) {
    clear();
    // Display game over message and final score
    println!("Game Over! Your final score: {}", score);
}
