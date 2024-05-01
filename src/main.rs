use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute, terminal,
};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::time::Duration;
use std::{collections::LinkedList, io::stdin, time::Instant};

extern crate rand;
use rand::Rng;

use serde_json::Value;

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_str(&self) -> &str {
        match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        }
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction,
}

fn main() {
    // Path to the options.json file
    let file_path = "options.json";

    // Open the file in read mode
    let mut file = File::open(file_path).expect("Failed to open file");

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Deserialize json
    let settings: Value = serde_json::from_str(&contents).expect("Invalid JSON");

    // Extract settings
    let speed = settings["speed"].as_u64().expect("1 not found / !integer");
    let boost_speed = settings["boost_speed"]
        .as_u64()
        .expect("2 not found / !integer");

    let board_width: i32 = settings["width"].as_i64().expect("3") as i32;
    let board_height: i32 = settings["height"].as_i64().expect("4") as i32;

    let ai = settings["ai"].as_bool().expect("5 not found / !type");
    let god = settings["god"].as_bool().expect("6 not found / !type");

    let boost_key_str = settings["boost_key"]
        .as_str()
        .expect("boost_key not found / !type");
    let boost_key_char = boost_key_str.chars().next().unwrap();
    let boost_key: KeyCode = KeyCode::Char(boost_key_char);

    // // Get user to choose mode
    // let mut choice = String::new();
    // println!("Choose mode: ");
    // println!("a) normal ");
    // println!("b) safe ");
    // println!("c) ai ");
    // println!();
    //
    // stdin()
    //     .read_line(&mut choice)
    //     .expect("Did not enter a valid string");
    // if let Some('\n') = choice.chars().next_back() {
    //     choice.pop();
    // }
    // if let Some('\r') = choice.chars().next_back() {
    //     choice.pop();
    // }
    //
    // static mut MODE: &str = "normal";
    // unsafe {
    //     if choice == "safe" || choice == "b" {
    //         MODE = "safe";
    //     } else if choice == "ai" || choice == "c" {
    //         MODE = "ai";
    //     }
    // }

    // Initialize game elements
    clear();
    let mut snake = Snake {
        body: vec![(5, 5), (5, 6), (5, 7)].into_iter().collect(),
        direction: Direction::Right,
    };

    let mut food = (10, 10);
    let mut score = 0;
    let mut game_over = false;

    let duration_ms_default = speed;
    let mut duration_ms = duration_ms_default;

    let mut speed_debounce: bool = false;

    // Draw initial game state
    draw_game(
        &snake,
        &food,
        score,
        &snake.direction,
        board_height,
        board_width,
        0,
    );
    clear();

    let now = Instant::now();

    // Game loop
    while !game_over {
        // Draw game board
        draw_game(
            &snake,
            &food,
            score,
            &snake.direction,
            board_height,
            board_width,
            now.elapsed().as_secs(),
        );

        // AI
        let mut head = snake.body.front().unwrap().clone();
            if ai == true {
                if head.0 > food.0 {
                    snake.direction = change_direction(Direction::Left, snake.direction);
                } else if head.0 < food.0 {
                    snake.direction = change_direction(Direction::Right, snake.direction);
                } else if head.1 > food.1 {
                    snake.direction = change_direction(Direction::Up, snake.direction);
                } else if head.1 < food.1 {
                    snake.direction = change_direction(Direction::Down, snake.direction);
                }
            }
        
        // Handle player input
        if poll(Duration::from_millis(duration_ms)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                match code {
                    KeyCode::Up => {
                        snake.direction = change_direction(Direction::Up, snake.direction)
                    }
                    KeyCode::Down => {
                        snake.direction = change_direction(Direction::Down, snake.direction)
                    }
                    KeyCode::Left => {
                        snake.direction = change_direction(Direction::Left, snake.direction)
                    }
                    KeyCode::Right => {
                        snake.direction = change_direction(Direction::Right, snake.direction)
                    }
                    _ => {
                        if code == boost_key {
                            if speed_debounce == false {
                                duration_ms = boost_speed;
                                speed_debounce = true;
                            } else {
                                duration_ms = duration_ms_default;
                                speed_debounce = false;
                            }
                        }
                    }
                }
            }
        }

        // Update snake position
        match snake.direction {
            Direction::Up => {
                if head.1 <= 0 {
                    head.1 = board_height;
                }
                // vertical
                else {
                    head.1 -= 1;
                }
            }
            Direction::Down => {
                if head.1 >= board_height - 1 {
                    head.1 = 0;
                }
                // vertical
                else {
                    head.1 += 1;
                }
            }
            Direction::Left => {
                if head.0 <= 0 {
                    head.0 = board_width;
                }
                // horizontal
                else {
                    head.0 -= 1;
                }
            }
            Direction::Right => {
                if head.0 >= board_width - 1 {
                    head.0 = 0;
                }
                // horizontal
                else {
                    head.0 += 1;
                }
            }
        }

        // Check for collisions
        if head == food {
            score += 1;
            // Generate new food location
            food = (
                rand::thread_rng().gen_range(0..board_width),
                rand::thread_rng().gen_range(0..board_height),
            );
        } else {
            snake.body.pop_back();
        }

        {
            // if head.0 < 0 || head.0 >= board_width || head.1 < 0 || head.1 >= board_height || snake.body.contains(&head) {
            //     game_over = true;
            // }

            if snake.body.contains(&head) && !god {
                game_over = true;
            }
        }

        snake.body.push_front(head);

        // Move cursor to the start of the game board
        execute!(io::stdout(), cursor::MoveTo(0, 1)).unwrap();
    }

    // Display game over screen
    draw_game_over(score);
}

// Clears the game board
fn clear() {
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
}

fn change_direction(new_direction: Direction, current_direction: Direction) -> Direction {
    // Don't make the snake go into itself
    if current_direction == Direction::Up && new_direction == Direction::Down
        || current_direction == Direction::Down && new_direction == Direction::Up
        || current_direction == Direction::Left && new_direction == Direction::Right
        || current_direction == Direction::Right && new_direction == Direction::Left
    {
        return current_direction;
    } else {
        return new_direction;
    }
}

fn move_possible(head: (i32, i32), body: &LinkedList<(i32, i32)>, direction: Direction, mode: &str) -> bool {
    if mode == "safe ai" { return true; }

    if direction == Direction::Left  && body.contains(&(head.0 - 1, head.1    )) { return false; }
    if direction == Direction::Right && body.contains(&(head.0 + 1, head.1    )) { return false; }
    if direction == Direction::Up    && body.contains(&(head.0,     head.1 - 1)) { return false; }
    if direction == Direction::Down  && body.contains(&(head.0,     head.1 + 1)) { return false; }
    
    return true;
}

fn move_somewhere(head: (i32, i32), body: &LinkedList<(i32, i32)>) -> Direction {
    if move_possible(head, body, Direction::Left, "ai") { return Direction::Left; }
    if move_possible(head, body, Direction::Right, "ai") { return Direction::Right; }
    if move_possible(head, body, Direction::Up, "ai") { return Direction::Up; }
    return Direction::Down;
}

fn draw_game(
    snake: &Snake,
    food: &(i32, i32),
    score: u32,
    direction: &Direction,
    board_height: i32,
    board_width: i32,
    now: u64,
) {
    // Draw the game board
    for y in 0..board_height {
        for x in 0..board_width {
            let c = if (x, y) == *food {
                "$ " // The goodies
            } else if snake.body.contains(&(x, y)) {
                "o " // Snake
            } else {
                ". " // Empty space
            };
            print!("{}", c);
        }
        println!();
    }

    // Move cursor to the bottom to draw the score
    execute!(io::stdout(), cursor::MoveTo(0, 0)).unwrap();
    // Draw the score and direction
    println!(
        "Score: {:<15} Direction: {:<15} Time Elapsed: {:<14} Press 'O' for options",
        score,
        direction.as_str(),
        now,
    );
}

fn draw_game_over(score: u32) {
    clear();
    // Display game over message and final score
    println!("Game Over! Your final score: {}", score);
}
