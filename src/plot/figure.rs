pub struct Filled {
    filler: String,
    height: usize,
    width: usize,
}

impl Filled {
    pub fn new(width: usize, height: usize, filler: &str) -> Self {
        let filler = filler.to_string();

        Filled {
            filler,
            height,
            width
        }
    }

    pub fn blank(width: usize, height: usize) -> Self {
        Self::new(width, height, " ")
    }

    pub fn render(&self) -> String {
        let mut s = String::new();

        for r in 0..self.height {
            for c in 0..self.width {
                s += &self.filler;

            }
            if r != self.height - 1 {
                s += "\n";
            }
        }

        s
    }
}
