pub struct CircularBuffer<T> {
    buf: Vec<Option<T>>,
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buf = Vec::with_capacity(capacity + 1);
        for _ in 0..capacity + 1 {
            buf.push(None);
        }

        Self {
            buf,
            start: 0,
            end: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if (self.end + 1) % self.buf.len() == self.start {
            return Err(Error::FullBuffer);
        }
        self.overwrite(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.start == self.end {
            return Err(Error::EmptyBuffer);
        }

        let item = self.buf[self.start].take().unwrap();
        self.start = (self.start + 1) % self.buf.len();

        Ok(item)
    }

    pub fn clear(&mut self) {
        self.start = self.end;
        self.buf.iter_mut().for_each(|item| {
            item.take();
        });
    }

    pub fn overwrite(&mut self, element: T) {
        self.buf[self.end] = Some(element);
        self.end = (self.end + 1) % self.buf.len();
        if self.start == self.end {
            self.start = (self.start + 1) % self.buf.len();
        }
    }
}
