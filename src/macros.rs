#[macro_export]
macro_rules! left_pad {
    ($s:expr, $w:expr) => {
        &(" ".repeat($w - $s.len()) + $s)
    };
}

#[macro_export]
macro_rules! max_length {
    ($x:expr) => {
        $x.iter().map(|s| s.len()).max().unwrap()
    };
}
