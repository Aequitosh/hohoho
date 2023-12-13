use std::fmt;
use std::str::Chars;

use thiserror::Error as ThisError;

#[non_exhaustive]
#[allow(non_snake_case)]
#[derive(ThisError, Debug)]
pub enum Error {
    InvalidTriplet(String),
    MissingHoHoHO(usize),
    MissingHOHoHo,
}

impl fmt::Display for Error {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::InvalidTriplet(ref s) => write!(f, "Invalid HOHOHO-triplet: {}", s),
            Error::MissingHoHoHO(n)      => write!(f, "{} missing HoHoHO(s) (\"closing bracket\")", n),
            Error::MissingHOHoHo         => write!(f, "Missing HOHoHo (\"opening bracket\")"),
        }
    }
}

/// An executable **HOHOHO!** command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    /// Increment memory cell at pointer.
    IncrementCell,
    /// Decrement memory cell at pointer.
    DecrementCell,
    /// Move pointer to the right (or "forward" on the "tape").
    MoveRight,
    /// Move pointer to the left (or "backward" on the "tape").
    MoveLeft,
    /// Jump past the matching `HoHoHO` if the cell at the pointer is `0`.
    JumpForward,
    /// Jump back to the matching `HOHoHo` if the cell at the pointer is *not* `0`.
    JumpBackward,
    /// Output the character signified by the memory cell where the pointer is.
    OutputFromCell,
    /// Input a character and store it in the memory cell where the pointer is.
    InputToCell,
}

impl fmt::Display for Command {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Command::IncrementCell  => write!(f, "HOHOHO"),
            Command::DecrementCell  => write!(f, "HoHoHo"),
            Command::MoveRight      => write!(f, "HOHOHo"),
            Command::MoveLeft       => write!(f, "HoHOHO"),
            Command::JumpForward    => write!(f, "HOHoHo"),
            Command::JumpBackward   => write!(f, "HoHoHO"),
            Command::OutputFromCell => write!(f, "HoHOHo"),
            Command::InputToCell    => write!(f, "HOHoHO"),
        }
    }
}

impl Command {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "HOHOHO" => Some(Command::IncrementCell),
            "HoHoHo" => Some(Command::DecrementCell),
            "HOHOHo" => Some(Command::MoveRight),
            "HoHOHO" => Some(Command::MoveLeft),
            "HOHoHo" => Some(Command::JumpForward),
            "HoHoHO" => Some(Command::JumpBackward),
            "HoHOHo" => Some(Command::OutputFromCell),
            "HOHoHO" => Some(Command::InputToCell),
            _ => None,
        }
    }

    #[rustfmt::skip]
    fn as_brainfuck(&self) -> &'static char {
        match *self {
            Command::IncrementCell  => &'+',
            Command::DecrementCell  => &'-',
            Command::MoveRight      => &'>',
            Command::MoveLeft       => &'<',
            Command::JumpForward    => &'[',
            Command::JumpBackward   => &']',
            Command::OutputFromCell => &'.',
            Command::InputToCell    => &',',
        }
    }
}

pub struct Program {
    commands: Vec<Command>,
}

struct TripletParseIter<'a> {
    source: Chars<'a>,
    command_buf: String,
    jump_stack: Vec<()>,
    done: bool,
}

impl<'a> TripletParseIter<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars(),
            command_buf: String::new(),
            jump_stack: Vec::new(),
            done: false,
        }
    }

    fn try_match_triplet(&mut self) -> Result<Command, Error> {
        if let Some(cmd) = Command::from_str(self.command_buf.as_ref()) {
            match cmd {
                Command::JumpForward => {
                    self.jump_stack.push(());
                }
                Command::JumpBackward => {
                    if self.jump_stack.pop().is_none() {
                        return Err(Error::MissingHOHoHo);
                    }
                }
                _ => {}
            };

            Ok(cmd)
        } else {
            Err(Error::InvalidTriplet(self.command_buf.clone()))
        }
    }
}

impl<'a> Iterator for TripletParseIter<'a> {
    type Item = Result<Command, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        self.command_buf.clear();

        for c in self.source.by_ref() {
            if c.is_whitespace() {
                continue;
            }

            self.command_buf.push(c);

            if self.command_buf.len() == 6 {
                let res = self.try_match_triplet();

                if res.is_err() {
                    self.done = true;
                }

                return Some(res);
            }
        }

        if !self.jump_stack.is_empty() {
            let missing_count = self.jump_stack.len();
            return Some(Err(Error::MissingHoHoHO(missing_count)));
        }

        self.done = true;

        None
    }
}

impl Program {
    pub fn parse(source: &str) -> Result<Self, Error> {
        let commands = TripletParseIter::new(source).collect::<Result<Vec<Command>, Error>>()?;

        Ok(Program { commands })
    }

    pub fn to_brainfuck(self) -> Result<brainfuck::program::Program, brainfuck::program::Error> {
        let bf: String = String::from_iter(self.commands.into_iter().map(|cmd| cmd.as_brainfuck()));
        brainfuck::program::Program::parse(&bf)
    }
}
