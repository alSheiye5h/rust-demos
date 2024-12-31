use std::io;
use rand::Rng;
use std::sync::Mutex;
use lazy_static::lazy_static;


lazy_static! {
    static ref PLAYER1: Mutex<String> = Mutex::new(String::new());
    static ref PLAYER2: Mutex<String> = Mutex::new(String::new());
    static ref ROLE: Mutex<i32> = Mutex::new(1); 
    static ref BOARD: Mutex<Vec<Vec<char>>> = Mutex::new(vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ]);
}

fn toggle_role() {
    let mut role = ROLE.lock().unwrap();
    *role = match *role {
        1 => 2,
        2 => 1,
        _ => 0,
    }
}

fn intro() {
    let mut player1: String = String::new();
    let mut player2: String = String::new();

    println!("player1: Enter your name :");
    io::stdin().read_line(&mut player1).expect("naming player1 failed");
    println!("player2: Enter your name :");
    io::stdin().read_line(&mut player2).expect("naming player2 failed");

    let mut role = ROLE.lock().unwrap();
    let mut rng = rand::thread_rng();
    let random: i32 = rng.gen_range(0..=10);

    if random < 5 { *role = 1 } else { *role = 2 };

    let mut player1_lock = PLAYER1.lock().unwrap();
    let mut player2_lock = PLAYER2.lock().unwrap();

    *player1_lock = player1.trim().to_string();
    *player2_lock = player2.trim().to_string();
}

fn show_board() {
    let board = BOARD.lock().unwrap();
    for elem in board.iter() {
        println!("{} | {} | {}", elem[0], elem[1], elem[2]);
    }
}

fn choice() -> bool {
    let mut response: bool = false;
    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("failed choosing");

    let choix: i32 = match choice.trim().parse::<i32>() {
        Ok(n) => {
            if (1..=9).contains(&n) {
                response = true;
                n
            } else {
                panic!("Invalid choice !");
            }
        },
        _ => panic!("Invalid choice !"),
    };
    response = edit_board(choix);
    println!("{}", response);
    response
}

fn edit_board(choix: i32) -> bool {
    let mut board = BOARD.lock().unwrap();
    let role = ROLE.lock().unwrap();
    let symbol: char = match *role {
        1 => 'X',
        2 => 'O',
        _ => panic!("Invalid role value"), // Handle any unexpected values
    };

    match choix {
        1 => {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[0][0] = symbol;
            } else {
                return false;
            }
        } 
        2 => {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[0][1] = symbol;
            } else {
                return false;
            }
        } 
        3 =>  {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[0][2] = symbol;
            } else {
                return false;
            }
        }  
        4 =>  {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[1][0] = symbol;
            } else {
                return false;
            }
        }  
        5 => {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[1][1] = symbol;
            } else {
                return false;
            }
        }  
        6 => {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[1][2] = symbol;
            } else {
                return false;
            }
        }  
        7 => {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[2][0] = symbol;
            } else {
                return false;
            }
        }  
        8 => {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[2][1] = symbol;
            } else {
                return false;
            }
        }  
        9 => {
            if board[0][0].to_string().parse::<i32>().is_ok() {
                board[2][2] = symbol;
            } else {
                return false;
            }
        }  
        _ => panic!("Invalid position: {}. Please choose a number between 1 and 9.", choix), 
    }
    true

}

fn check_win() -> bool {
    let mut response: bool = false;
    let board = BOARD.lock().unwrap();
    for v in board.iter() {
        if v[0] == v[1] && v[0] == v[2] {
            response = true;
        }    
    }
    for i in 0..=2 {
        if board[0][i] == board[1][i] && board[0][i] == board[2][i] {
            response = true;
        }
    }
    if board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        response = true;
    } if board[0][2] == board[1][1] && board[0][2] == board[2][0] {
        response = true;
    }
    response
}

fn check_draw() -> bool {
    let mut response = true;
    let board = BOARD.lock().unwrap();
    for v in board.iter() {
        for el in v.iter() {
            match el.to_string().parse::<i32>() {
                Ok(n) => {response = false},
                _ => response = true,
            }
        }   
    }
    response
}



fn main() {
    intro();
    while true {
        show_board();
        choice();
        if check_win() {
            let role = ROLE.lock().unwrap();
            let winner: String = match *role {
                1 => "player 1".to_string(),
                2 => "player 2".to_string(),
                _ => "no one".to_string(),
            };
            show_board();
            println!("{} won!", winner);
            break;
        } else if check_draw() {
            show_board();
            println!("draw!");
            break;
        }
        toggle_role();
    }
}