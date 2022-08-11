mod game_state;

use game_state::GameState;
use mini_http;
use mini_http::Server;
use rand::Rng;
use std::rc::Rc;

// All the web server and network code by Harald H.
// https://github.com/haraldh

fn index_page() -> &'static [u8] {
    include_bytes!("../client/index.html")
}

fn index_js() -> &'static [u8] {
    include_bytes!("../client/index.js")
}

fn style_css() -> &'static [u8] {
    include_bytes!("../client/styles.css")
}

const NOT_FOUND: &str = r#"
<!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML 2.0//EN">
<html><head>
<title>404 Not Found</title>
</head><body>
<h1>Not Found</h1>
<p>The requested URL was not found on this server.</p>
</body></html>
"#;

// Single-player mode
//
// Color scheme:
// y (yellow): right letter, wrong position
// g (green): letter at the right position

fn check_single(query: Option<&str>, state: Rc<GameState>) -> Vec<u8> {
    let mut response = vec![b'c'; 5];

    // Get guess parameter
    if query.is_some() {
        let the_params = query.unwrap();
        let the_params_parts = the_params.split_once("&").unwrap();
        let the_guess = the_params_parts.0;
        let the_guess_parts = the_guess.split_once("=").unwrap();
        let guess = the_guess_parts.1;
        //println!("The guess: {}", guess);

        let guess_size: usize = guess.len() as usize;
        if guess_size == 5 {
            for letter_index in 0..guess_size {
                if guess.as_bytes()[letter_index] == state.word.as_bytes()[letter_index] {
                    response[letter_index] = b'g';
                } else {
                    for letter_byte in state.word.as_bytes() {
                        if guess.as_bytes()[letter_index].eq(letter_byte)
                            && response[letter_index] == b'c'
                        {
                            response[letter_index] = b'y';
                        }
                    }
                }
            }
        }
    }

    return response;
}

// Multi-player mode
//
// Color scheme:
// b (blue): letter at the right position for at least one word
// p (purple): word match
// r (red): word was already a match

fn check_multi(query: Option<&str>, state: Rc<GameState>) -> Vec<u8> {
    let mut response = vec![b'c'; 5];

    // Get guess and player parameters
    if query.is_some() {
        let the_params = query.unwrap();
        let the_params_parts = the_params.split_once("&").unwrap();
        let the_guess = the_params_parts.0;
        let the_player = the_params_parts.1;
        let the_guess_parts = the_guess.split_once("=").unwrap();
        let guess = the_guess_parts.1;
        //println!("The guess: {}", guess);
        let the_player_parts = the_player.split_once("=").unwrap();
        let player = the_player_parts.1;
        //println!("The player: {}", player);

        // Wrong word size
        let word_size: usize = guess.len() as usize;
        if word_size != 5 {
            return response;
        }

        // Check if this word was already a match
        let matches_index = state.matches.borrow().iter().position(|x| x == guess);
        if matches_index.is_some() {
            response = vec![b'r'; 5];
            return response;
        }

        // Check letters
        for letter_index in 0..word_size {
            let letter_char = guess.as_bytes()[letter_index] as char;
            let found_char = state.letters.borrow()[letter_index]
                .chars()
                .any(|ch| ch == letter_char);
            if found_char {
                response[letter_index] = b'b';
            } else {
                state.letters.borrow_mut()[letter_index].push_str(&letter_char.to_string());
            }
        }

        // Check if this word is a new match
        let guesses_index = state.guesses.borrow().iter().position(|x| x == guess);
        if guesses_index.is_some() {
            // Check if it matches a previous guess from the player
            let winner = &state.players.borrow()[guesses_index.unwrap()];
            if winner == player {
                response = vec![b'r'; 5];
                return response;
            }

            // Push word to matches
            state.matches.borrow_mut().push(guess.to_string());
            response = vec![b'p'; 5];
            //println!("New match: {}", guess.to_string());

            // Push winners
            state.winners.borrow_mut().push(player.to_string());
            state.winners.borrow_mut().push(winner.to_string());
            //println!("Winners: {}, {}", player.to_string(), winner.to_string());
        } else {
            // Push new word to guesses
            state.guesses.borrow_mut().push(guess.to_string());
            state.players.borrow_mut().push(player.to_string());
            //println!("New word: {}", guess.to_string());
        }
    }

    return response;
}

fn check_winners(state: Rc<GameState>) -> Vec<u8> {
    let mut response = String::from("");
    let comma = String::from(", ");
    let winners_size: usize = state.winners.borrow().len() as usize;

    for winners_index in 0..winners_size {
        let winner = &state.winners.borrow()[winners_index];
        if response == "" {
            response = winner.to_string();
        } else {
            response = response + &comma + winner;
        }
    }

    return response.as_bytes().to_vec();
}

fn check_matches(state: Rc<GameState>) -> Vec<u8> {
    let mut response = String::from("");
    let comma = String::from(", ");
    let matches_size = state.matches.borrow().len();

    for matches_index in 0..matches_size {
        let matched = &state.matches.borrow()[matches_index];
        if response == "" {
            response = matched.to_string();
        } else {
            response = response + &comma + matched;
        }
    }

    return response.as_bytes().to_vec();
}

#[cfg(target_os = "wasi")]
fn get_server() -> Server {
    mini_http::Server::preopened().unwrap()
}

#[cfg(not(target_os = "wasi"))]
fn get_server() -> Server {
    mini_http::Server::new("127.0.0.1:10030").unwrap()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Generate random number from word list
    const WORDLIST: &str = include_str!("../client/wordList.txt");

    let mut rng = rand::thread_rng();
    let mut random_index: usize = rng.gen_range(0..WORDLIST.len() / 6);

    while WORDLIST.as_bytes()[random_index] != 0x0A {
        // check for newline
        random_index += 1;
    }
    random_index += 1;
    let the_word_static = &WORDLIST[random_index..random_index + 5];
    let the_word_copy: &'static str = the_word_static.clone();
    let the_word_string = the_word_copy.to_lowercase();

    // ("The word: {}", the_word_string);

    let state: Rc<GameState> = Rc::new(GameState::from(the_word_string));

    // Run server
    get_server()
        .tcp_nodelay(true)
        .start(move |req| match req.uri().path() {
            "/index.js" => mini_http::Response::builder()
                .status(200)
                .header("Content-Type", "text/javascript")
                .body(index_js().to_vec())
                .unwrap(),
            "/styles.css" => mini_http::Response::builder()
                .status(200)
                .header("Content-Type", "text/css")
                .body(style_css().to_vec())
                .unwrap(),
            "/single" => mini_http::Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(check_single(req.uri().query(), state.clone()))
                .unwrap(),
            "/multi" => mini_http::Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(check_multi(req.uri().query(), state.clone()))
                .unwrap(),
            "/winners" => mini_http::Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(check_winners(state.clone()))
                .unwrap(),
            "/matches" => mini_http::Response::builder()
                .status(200)
                .header("Content-Type", "text/plain")
                .body(check_matches(state.clone()))
                .unwrap(),
            "/" => mini_http::Response::builder()
                .status(200)
                .header("Content-Type", "text/html")
                .body(index_page().to_vec())
                .unwrap(),
            _ => mini_http::Response::builder()
                .status(404)
                .body(NOT_FOUND.as_bytes().to_vec())
                .unwrap(),
        })?;
    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}
