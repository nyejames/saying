mod say;
#[cfg(test)]
mod tests;

/// Print styled and coloured text to stdout with ANSI SGR codes.
///
/// `say!` expands at compile time into a single `print!` invocation.
/// It uses only `macro_rules!` (no proc macros), has zero dependencies,
/// and introduces no runtime overhead beyond the underlying `print!`.
///
/// By default, output is written to `stdout` and a newline is appended.
/// Use the `Inline` style to suppress the automatic newline.
///
/// # Basic Usage
///
/// Styles and colours are written directly before expressions.
///
/// # Style Persistence
///
/// Once a colour or style is set, it applies to all following arguments until changed:
///
/// ```
/// # use saying::say;
/// say!(Red "This ", "is ", "all ", "red");
/// say!(Red "This is red, ", Blue "this is blue");
/// ```
///
/// ```rust
/// use saying::say;
///
/// say!("Hello");
/// say!(Green "Hello");
/// say!(Red Bold "Error:");
/// say!(Red "Hello ", Blue "world");
/// ```
///
/// # Examples
///
/// ```
/// // Status messages
/// say!(Green Bold "✓ Build successful");
/// say!(Red "✗ Test failed");
///
/// // Mixed styles on one line
/// say!(
///     Yellow "Warning: ",
///     Reset "File ",
///     Bold "config.toml",
///     Reset " not found"
/// );
///
/// // Method calls on expressions
/// say!(Blue "Result: ", "hello".to_uppercase());
/// say!(Red "Repeat: ", "^".repeat(10));
/// ```
///
/// # Expressions Instead of `{}` Formatting
///
/// Rather than using `"{}"` placeholders, expressions are written inline
/// in the order they should appear. Any valid Rust expression is allowed.
///
/// ```rust
/// let x = 2 + 2;
/// let name = "world";
///
/// say!(Yellow x);
/// say!(Cyan "Hello ", name, "!");
/// say!(Magenta 3 * (1 + 2));
/// ```
///
/// Expressions do not need to be wrapped in parentheses.
///
/// # Inline Printing
///
/// By default, `say!` adds a newline. Use `Inline` to prevent this:
///
/// ```
/// # use saying::say;
/// say!(Inline "Loading");
/// say!(Inline ".");
/// say!(Inline ".");
/// say!("done!");  // This one adds the newline
/// ```
///
/// # Debug Formatting
///
/// Prefix an expression with `#` to debug-print it (`{:?}`).
/// Use `Pretty` before `#` for pretty debug formatting (`{:#?}`).
///
/// ```rust
/// let values = vec![1, 2, 3];
///
/// say!(Green "Values: ", #values);
/// say!(Blue "Values: ", Pretty #values);
/// ```
///
/// Additional `#`-based format specifiers are supported:
///
/// ```rust
/// let n = 255;
///
/// say!(Green "Hex: ", #x n);     // 0xff
/// say!(Blue "Binary: ", #b 15);  // 0b1111
/// say!(Red "Octal: ", #o 64);    // 0o100
/// ```
///
/// # Colours and Styles
///
/// Supports the basic 8 ANSI colours and their highlight variants (`RedHL`,
/// `BlueHL`, etc.), along with styling keywords such as:
///
/// - `Bold`
/// - `Underline`
/// - `Italic`
/// - `Invert`
/// - `Bright`
/// - `Dark`
/// - `Reset`
/// - `Inline`
///
/// ```rust
/// say!(RedHL Bold "Warning");
/// say!(Green Underline "Success");
/// ```
///
/// # Notes
///
/// - Only compatible with ANSI-capable terminals.
/// - Does not perform runtime capability checks.
/// - Emits a compile error if styles are provided without any expressions.
/// - All standard `println!` formatting errors still apply.
///
/// For more advanced examples, see the crate-level documentation and tests.

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
