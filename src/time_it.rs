#[macro_export]
macro_rules! time_it {
    ($took:ident, $return:ident, $expression:expr) => {
        let start = std::time::Instant::now();
        $return = $expression;
        let end = std::time::Instant::now();

        $took = end.duration_since(start);
    };
}
