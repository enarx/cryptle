use std::cell::RefCell;
use std::rc::Rc;

pub struct GameState {
    pub guesses: Rc<RefCell<Vec<String>>>,
    pub matches: Rc<RefCell<Vec<String>>>,
    pub players: Rc<RefCell<Vec<String>>>,
    pub winners: Rc<RefCell<Vec<String>>>,
    pub letters: Rc<RefCell<Vec<String>>>,
    pub word: String,
}

impl GameState {
    pub fn from(word: String) -> GameState {
        GameState {
            guesses: Rc::new(RefCell::new(vec![])),
            matches: Rc::new(RefCell::new(vec![])),
            players: Rc::new(RefCell::new(vec![])),
            winners: Rc::new(RefCell::new(vec![])),
            letters: Rc::new(RefCell::new(vec![String::new(); 5])),
            word,
        }
    }
}
