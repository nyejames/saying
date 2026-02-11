mod say;
#[cfg(test)]
mod tests;

#[macro_export]
macro_rules! say {
    ($($tt:tt)*) => {{
        $crate::__say_parse! {
            tokens = [$($tt)*],
            sgr = [],
            fmt = "",
            args = [],
            newline = true,
            skip = false,
        }
    }};
}
