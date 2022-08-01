pub struct Computer {
    intcode: Vec<usize>,
}

impl Computer {
    pub fn new(intcode: &Vec<usize>) -> Computer {
        Computer {
            intcode: intcode.to_vec(),
        }
    }

    pub fn input(&mut self, a: usize, b: usize) {
        self.intcode[1] = a;
        self.intcode[2] = b;
    }

    pub fn run(&mut self) -> usize {
        let mut pointer: usize = 0;
        loop {
            let opcode = self.intcode[pointer];
            match opcode {
                1 => {
                    self.add(
                        self.intcode[pointer + 1],
                        self.intcode[pointer + 2],
                        self.intcode[pointer + 3],
                    );
                    pointer += 4;
                }
                2 => {
                    self.mul(
                        self.intcode[pointer + 1],
                        self.intcode[pointer + 2],
                        self.intcode[pointer + 3],
                    );
                    pointer += 4;
                }
                99 => return self.intcode[0],
                _ => panic!("intcode at index {pointer} not valid"),
            }
        }
    }

    fn add(&mut self, a: usize, b: usize, out: usize) {
        self.intcode[out] = self.intcode[a] + self.intcode[b];
    }

    fn mul(&mut self, a: usize, b: usize, out: usize) {
        self.intcode[out] = self.intcode[a] * self.intcode[b];
    }
}
