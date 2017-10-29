use term;


pub fn error(err: &str) {
    let mut t = term::stderr().expect("Couldn't open terminal device");

    let _ = t.attr(term::Attr::Bold);
    let _ = t.fg(term::color::BLACK);
    let _ = write!(t, "dent: ");

    let _ = t.fg(term::color::RED);
    let _ = write!(t, "error: ");

    let _ = t.reset();
    let _ = writeln!(t, "{}", err);
}
