use std::fmt;
use std::net::SocketAddr;


#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub address: SocketAddr,
    pub state: State,
    pub last_message: u8,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.address, self.state)
    }
}

#[derive(Copy, Clone)]
pub enum State {
    Alive,
    Questionable,
    Dead,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            &State::Alive => "alive",
            &State::Questionable => "questionable",
            &State::Dead => "dead",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &fmt::Display).fmt(f)
    }
}