use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frames: Rc<RefCell<Frame>>,
    current: Weak<RefCell<Frame>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        let frames = Rc::new(RefCell::new(Frame::new(1)));
        let current = Rc::downgrade(&frames);
        Self { frames, current }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.current.upgrade() {
            Some(current) => match current.borrow_mut().roll(pins) {
                Ok(Some(frame)) => {
                    self.current = frame;
                    Ok(())
                }
                Ok(None) => Ok(()),
                Err(err) => Err(err),
            },
            None => panic!("Pointer to the current frame is not set."),
        }
    }

    pub fn score(&self) -> Option<u16> {
        self.frames.borrow().score()
    }
}

#[derive(Debug)]
enum FrameType {
    Open,
    Spare,
    Strike,
    Final { fill_ball: bool },
}

#[derive(Debug)]
struct Frame {
    typ: FrameType,
    throws: Vec<u16>,
    next: Option<Rc<RefCell<Frame>>>,
    number: u16,
    done: bool,
}

impl Frame {
    fn new(number: u16) -> Self {
        Self {
            typ: FrameType::Open,
            throws: Vec::new(),
            next: None,
            done: false,
            number,
        }
    }

    fn roll(&mut self, pins: u16) -> Result<Option<Weak<RefCell<Self>>>, Error> {
        // check validity of the game
        if self.done {
            return Err(Error::GameComplete);
        }
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        self.throws.push(pins);
        let throws_sum: u16 = self.throws.iter().sum();
        match self.typ {
            FrameType::Final { fill_ball: true } => {
                // the two balls after a final strike cannot score an invalid number of pins
                if self.throws.len() > 2
                    && self.throws[0] == 10
                    && self.throws[1] != 10
                    && self.throws[1..].iter().sum::<u16>() > 10
                {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
            _ => {
                if throws_sum > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
            }
        };

        // compute progress in the game
        // if the next throw should be in a new frame, create one
        // if the frame remains, return Ok(None)
        // type of the current frame is updated depending on the current throw
        match self.typ {
            FrameType::Open => {
                if pins == 10 {
                    self.typ = FrameType::Strike;
                    self.done = true;
                    return Ok(Some(self.next_frame()));
                }
                if self.throws.len() < 2 {
                    // next throw in the current frame
                    return Ok(None);
                } else {
                    if throws_sum == 10 {
                        self.typ = FrameType::Spare;
                    }
                    self.done = true;
                    return Ok(Some(self.next_frame()));
                }
            }
            FrameType::Final { fill_ball: false } => {
                if throws_sum == 10 {
                    self.typ = FrameType::Final { fill_ball: true };
                    return Ok(None);
                }
                if self.throws.len() == 2 {
                    self.done = true;
                }
            }
            FrameType::Final { fill_ball: true } => {
                if self.throws.len() == 3 {
                    self.done = true;
                }
            }
            _ => (),
        }
        Ok(None)
    }

    fn next_frame(&mut self) -> Weak<RefCell<Self>> {
        let mut next = Self::new(self.number + 1);
        if next.number == 10 {
            next.typ = FrameType::Final { fill_ball: false };
        }
        let next = Rc::new(RefCell::new(next));
        let current = Rc::downgrade(&next);
        self.next = Some(next);
        current
    }

    fn score(&self) -> Option<u16> {
        if !self.done {
            return None;
        }
        let mut total_score = self.throws.iter().sum();
        match self.typ {
            FrameType::Final { fill_ball: _ } => {
                return Some(total_score);
            }
            FrameType::Strike => {
                // account the next two throws
                total_score += match &self.next {
                    Some(next) => next.borrow().score_throws(2)?,
                    None => return None,
                };
            }
            FrameType::Spare => {
                // account the next throw
                total_score += match &self.next {
                    Some(next) => next.borrow().score_throws(1)?,
                    None => return None,
                };
            }
            _ => (),
        }
        total_score += match &self.next {
            Some(next) => next.borrow().score()?,
            None => return None,
        };
        Some(total_score)
    }

    fn score_throws(&self, n: usize) -> Option<u16> {
        if !self.done {
            return None;
        }
        let mut total_score = self.throws.iter().take(n).sum();
        if n > self.throws.len() {
            total_score += match &self.next {
                Some(next) => next.borrow().score_throws(n - self.throws.len())?,
                None => return None,
            }
        }
        Some(total_score)
    }
}
