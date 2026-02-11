use crate::say;

#[test]
fn basic_print() {
    say!("Hello World!");
}

#[test]
fn styled_print() {
    say!(Red Bold "Hello");
}

#[test]
fn mixed_args() {
    let name = "World";
    say!(Green "Hello ", name, "!");
}

#[test]
fn multiple_styles() {
    say!(Red "Red ", Blue "Blue ", Green Bold "Green Bold");
}

#[test]
fn highlight_colors() {
    say!(Black YellowHL "Warning!");
}

#[test]
fn complex_expressions() {
    let messages = [1, 2, 3];
    say!(Yellow "There are ", messages.len(), " messages");
    say!(Blue "Count: ", messages.len().to_string(), "!");
    say!(Magenta "Count: ", messages.len().to_string(), White " - a different colour at the end");
}

#[test]
fn style_before_expression() {
    let message = "yo";
    say!(Cyan message, ", whats up?");
}

#[test]
fn debug_display_function_call1() {
    let timer = std::time::Instant::now();
    say!(#timer.elapsed());
}

#[test]
fn debug_display_function_call2() {
    fn func_test(arg: &str) -> String {
        format!("arg: {arg}")
    }

    say!(#func_test("test"));
}

#[test]
fn debug_display_expression() {
    let string_vec = vec!["Hello ", "World"];

    say!(#string_vec);
}

#[test]
fn pretty_debug_display() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    say!(Pretty #nested);
}

#[test]
fn inline() {
    say!("This is on a newline. ");
    say!(Inline "This on a newline. ");
    say!(Inline "This is inline with the previous line. ");
}

#[test]
fn italics_with_reset() {
    say!("This is ", Italic "italicised", Reset " text.");
}

#[test]
fn bright() {
    say!(Bright "This BRIGHT TEXT");
    say!("not bright but the", Bright "N IT GETS BLINDING.");
    say!(Inline Bright Blue " INLINE BRIGHT MACRO FUN");
    say!(Bright Red "CRINGE", Yellow " BRIGHT ", Magenta "TEXT");
}

#[test]
fn dim() {
    say!(Dim "This is dimmed text for some reason. Don't know why you would even want this tbh");
}
