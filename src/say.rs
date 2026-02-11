#[doc(hidden)]
#[macro_export]
macro_rules! __say_join_sgr {
    ([$only:tt]) => {
        stringify!($only)
    };

    ([$first:tt $($rest:tt)+]) => {
        concat!(
            stringify!($first),
            ";",
            $crate::__say_join_sgr!([$($rest)+])
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __say_apply_sgr {
    (
        codes = [$($codes:tt)*],
        rest = $rest:tt,
        sgr = [$($_old:tt)*],
        fmt = $fmt:expr,
        args = [$($args:expr),*],
    ) => {
        $crate::__say_parse! {
            tokens = $rest,
            sgr = [$($codes)*],
            fmt = concat!(
                $fmt,
                "\x1b[",
                $crate::__say_join_sgr!([$($codes)*]),
                "m"
            ),
            args = [$($args),*],
        }
    };
}


#[doc(hidden)]
#[macro_export]
macro_rules! __say_parse {

    // End of input → emit
    (
        tokens = [],
        sgr = [$($sgr:tt)*],
        fmt = $fmt:expr,
        args = [$($args:expr),* $(,)?],
    ) => {{

        // Always reset at the end
        println!(concat!($fmt, "\x1b[0m"), $($args),*);
    }};

    // Comma separator - just skip it
    (
        tokens = [, $($rest:tt)*],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = $args:tt,
    ) => {
        $crate::__say_parse! {
            tokens = [$($rest)*],
            sgr = $sgr,
            fmt = $fmt,
            args = $args,
        }
    };

    // String literal
    (
        tokens = [$lit:literal $($rest:tt)*],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = [$($args:expr),*],
    ) => {
        $crate::__say_parse! {
            tokens = [$($rest)*],
            sgr = $sgr,
            fmt = concat!($fmt, $lit),
            args = [$($args),*],
        }
    };

    // Debug display: # followed by anything
    (
        tokens = [# $($rest:tt)*],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = $args:tt,
    ) => {
        $crate::__say_parse_debug! {
            tokens = [$($rest)*],
            sgr = $sgr,
            fmt = $fmt,
            args = $args,
        }
    };

    // Could be a style keyword - dispatch to check
    (
        tokens = [$style:ident $($rest:tt)*],
        sgr = [$($sgr:tt)*],
        fmt = $fmt:expr,
        args = $args:tt,
    ) => {
        $crate::__say_style_dispatch! {
            style = $style,
            rest = [$($rest)*],
            sgr = [$($sgr)*],
            fmt = $fmt,
            args = $args,
        }
    };

    // Expression (complex, like method calls)
    (
        tokens = [$expr:expr, $($rest:tt)*],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = [$($args:expr),*],
    ) => {
        $crate::__say_parse! {
            tokens = [$($rest)*],
            sgr = $sgr,
            fmt = concat!($fmt, "{}"),
            args = [$($args),*, $expr],
        }
    };

    // Expression at end (no comma)
    (
        tokens = [$expr:expr],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = [$($args:expr),*],
    ) => {
        $crate::__say_parse! {
            tokens = [],
            sgr = $sgr,
            fmt = concat!($fmt, "{}"),
            args = [$($args),*, $expr],
        }
    };

}

#[doc(hidden)]
#[macro_export]
macro_rules! __say_style_dispatch {
    (
        style = $style:ident,
        rest = $rest:tt,
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = $args:tt,
    ) => {
        $crate::__say_style_dispatch_inner! {
            $style,
            rest = $rest,
            sgr = $sgr,
            fmt = $fmt,
            args = $args
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __say_style_dispatch_inner {
    // Match each known style and expand to __say_apply_sgr
    (Reset, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [0], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Bold, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [1], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Underline, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [4], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Invert, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [7], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };

    // Pretty debug: Pretty #expr followed by comma
    (Pretty, rest = [# $expr:expr, $($rest:tt)*], sgr = $sgr:tt, fmt = $fmt:expr, args = [$($args:expr),* $(,)?]) => {
        $crate::__say_parse! {
            tokens = [$($rest)*],
            sgr = $sgr,
            fmt = concat!($fmt, "{:#?}"),
            args = [$($args,)* $expr],
        }
    };

    // Pretty debug: Pretty #expr at end
    (Pretty, rest = [# $expr:expr], sgr = $sgr:tt, fmt = $fmt:expr, args = [$($args:expr),* $(,)?]) => {
        $crate::__say_parse! {
            tokens = [],
            sgr = $sgr,
            fmt = concat!($fmt, "{:#?}"),
            args = [$($args,)* $expr],
        }
    };

    (Black, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [30], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Red, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [31], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Green, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [32], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Yellow, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [33], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Blue, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [34], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Magenta, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [35], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (Cyan, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [36], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (White, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [37], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };

    (BlackHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [40], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (RedHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [41], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (GreenHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [42], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (YellowHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [43], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (BlueHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [44], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (MagentaHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [45], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (CyanHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [46], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };
    (WhiteHL, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_apply_sgr! { codes = [47], rest = $rest, sgr = $sgr, fmt = $fmt, args = $args, }
    };

    // Fallback: identifier followed by . (method call/field access) - need to re-parse as expr
    ($ident:ident, rest = [. $($rest:tt)*], sgr = $sgr:tt, fmt = $fmt:expr, args = $args:tt) => {
        $crate::__say_parse_expr! {
            tokens = [$ident . $($rest)*],
            sgr = $sgr,
            fmt = $fmt,
            args = $args,
        }
    };

    // Fallback for unknown identifiers - treat as simple expression
    ($other:ident, rest = $rest:tt, sgr = $sgr:tt, fmt = $fmt:expr, args = [$($args:expr),* $(,)?]) => {
        $crate::__say_parse! {
            tokens = $rest,
            sgr = $sgr,
            fmt = concat!($fmt, "{}"),
            args = [$($args,)* $other],
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __say_parse_expr {
    // Expression followed by comma
    (
        tokens = [$expr:expr, $($rest:tt)*],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = [$($args:expr),* $(,)?],
    ) => {
        $crate::__say_parse! {
            tokens = [$($rest)*],
            sgr = $sgr,
            fmt = concat!($fmt, "{}"),
            args = [$($args,)* $expr],
        }
    };

    // Expression at end (no comma after)
    (
        tokens = [$expr:expr],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = [$($args:expr),* $(,)?],
    ) => {
        $crate::__say_parse! {
            tokens = [],
            sgr = $sgr,
            fmt = concat!($fmt, "{}"),
            args = [$($args,)* $expr],
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __say_parse_debug {
    // Debug expression followed by comma
    (
        tokens = [$expr:expr, $($rest:tt)*],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = [$($args:expr),* $(,)?],
    ) => {
        $crate::__say_parse! {
            tokens = [$($rest)*],
            sgr = $sgr,
            fmt = concat!($fmt, "{:?}"),
            args = [$($args,)* $expr],
        }
    };

    // Debug expression at end (no comma after)
    (
        tokens = [$expr:expr],
        sgr = $sgr:tt,
        fmt = $fmt:expr,
        args = [$($args:expr),* $(,)?],
    ) => {
        $crate::__say_parse! {
            tokens = [],
            sgr = $sgr,
            fmt = concat!($fmt, "{:?}"),
            args = [$($args,)* $expr],
        }
    };
}


