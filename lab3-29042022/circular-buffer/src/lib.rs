pub struct CircularBuffer<T> {
    used: usize,
    w_index: usize,
    r_index: usize,
    field: Vec<Option<T>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer { used: 0, w_index: 0, r_index: 0, field: vec![None; capacity] }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.used == self.field.len() { return Err(Error::FullBuffer); }
        self.field[self.w_index] = Some(_element);
        self.w_index = (self.w_index + 1) % self.field.len();
        if self.used != self.field.len() { self.used += 1; }
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.used == 0 { return Err(Error::EmptyBuffer); }
        let res = self.field[self.r_index].take().unwrap();
        self.r_index = (self.r_index + 1) % self.field.len();
        if self.used > 0 { self.used -= 1; }
        Ok(res)
    }

    pub fn clear(&mut self) {
        self.used = 0;
        self.field = vec![None; self.field.len()];
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.used != self.field.len() {
            self.write(_element).unwrap();
            return;
        }
        self.field[self.r_index] = Some(_element);
        self.r_index = (self.r_index + 1) % self.field.len();
    }
}
