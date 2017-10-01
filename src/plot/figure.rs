use stamp::Stamp;

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
            for _c in 0..self.width {
                s += &self.filler;

            }
            if r != self.height - 1 {
                s += "\n";
            }
        }

        s
    }
}

pub struct BorderChars {
    left: &'static str,
    bottom_left: &'static str,
    bottom: &'static str,
    bottom_right: &'static str,
    right: &'static str,
    top_left: &'static str,
    top: &'static str,
    top_right: &'static str,
}

pub const ASCII_BORDER: BorderChars = BorderChars {
    left: "|",
    bottom_left: "+",
    bottom: "-",
    bottom_right: "+",
    right: "|",
    top_left: "+",
    top: "-",
    top_right: "+",
};

pub const UNICODE_BORDER: BorderChars = BorderChars {
    left: "│",
    bottom_left: "└",
    bottom: "─",
    bottom_right: "┘",
    right: "│",
    top_left: "┌",
    top: "─",
    top_right: "┐",
};

pub struct Border {
    chars: BorderChars,
    height: usize,
    width: usize,
}

impl Border {
    pub fn new(chars: BorderChars, width: usize, height: usize) -> Self {
        Border {
            chars,
            height,
            width,
        }
    }

    pub fn render(&self) -> String {
        self.render_checked().unwrap()
    }

    fn render_checked(&self) -> Result<String, ()> {
        let filled = Stamp::new(&Filled::blank(self.width, self.height).render())?;

        let bottom_left = Stamp::new(self.chars.bottom_left)?;
        let bottom_right = Stamp::new(self.chars.bottom_right)?;
        let top_left = Stamp::new(self.chars.top_left)?;
        let top_right = Stamp::new(self.chars.top_right)?;

        let bottom = Stamp::new(&self.render_bottom())?;
        let left = Stamp::new(&self.render_left())?;
        let right = Stamp::new(&self.render_right())?;
        let top = Stamp::new(&self.render_top())?;

        let layered = filled
            .layer(&top_left, 0, 0)?
            .layer(&top, 1, 0)?
            .layer(&top_right, self.width - 1, 0)?
            .layer(&right, self.width - 1, 1)?
            .layer(&bottom_right, self.width - 1, self.height - 1)?
            .layer(&bottom, 1, self.height - 1)?
            .layer(&bottom_left, 0, self.height - 1)?
            .layer(&left, 0, 1)?;

        Ok(layered.render())
    }

    fn render_bottom(&self) -> String {
        render_horizontal_line(self.chars.bottom, self.width - 2)
    }

    fn render_left(&self) -> String {
        render_vertical_line(self.chars.left, self.height - 2)
    }

    fn render_right(&self) -> String {
        render_vertical_line(self.chars.right, self.height - 2)
    }

    fn render_top(&self) -> String {
        render_horizontal_line(self.chars.top, self.width - 2)
    }
}

fn render_horizontal_line(c: &str, size: usize) -> String {
    use std::iter::{FromIterator, repeat};

    String::from_iter(repeat(c.to_string()).take(size))
}

fn render_vertical_line(c: &str, size: usize) -> String {
    let mut s = String::new();

    for i in 0..size {
        s += c;

        if i != size - 1 {
            s += "\n";
        }
    }

    s
}
