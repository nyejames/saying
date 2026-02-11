use crate::say;

#[test]
fn empty() {
    say!();
}

#[test]
fn basic() {
    say!("Hello World!");
}

#[test]
fn bold() {
    say!(Red Bold "Hello");
}

#[test]
fn underline() {
    say!(Underline "Hello");
}

#[test]
fn mixed_args() {
    let name = "World";
    say!(Green "Hello ", name, "!");
}

#[test]
fn mixed_styles() {
    say!(Green "Hello ", Bold "! ", Dark Blue 1 + 1, Yellow " = 2");
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
fn expressions() {
    let messages = [1, 2, 3];
    say!(Yellow "There are ", messages.len(), " messages");
    say!(Blue "Count: ", messages.len().to_string(), "!");
    say!(Magenta "Count: ", messages.len().to_string(), White " - a different colour at the end");
}

#[test]
fn complex_expressions() {
    // Expression that needs to be evaluated
    // Using curly braces to avoid the expression being parsed as a style keyword
    say!(Dark Magenta "Line ", Bright 2 + 1);

    let length_of_underline = 6;
    say!(Red "^".repeat(length_of_underline));

    say!(".".repeat(length_of_underline));

    let string = "*";
    say!(string.repeat(5 * 2));

    // Curly braces for expression as the first argument
    // Previously failed when others succeeded
    say!(Bold Blue 2 + 1);
    say!(27 * 42);
}

#[test]
fn style_before_expression() {
    let message = "yo";
    say!(Cyan message, ", whats up?");
}

#[test]
fn expression_with_ref() {
    let num = 5;

    // Fine this way around
    say!(Cyan 1 + num, ":D");

    // BUT
    say!(Cyan num + 1, "????");
}

#[test]
fn method_calls() {
    say!(Red "^".repeat(5));
    say!(Blue "hello".to_uppercase());
    say!(Green "  test  ".trim());
    say!("The best number is ", Cyan 42.to_string(), " because it is the answer to everything.");
}

#[test]
fn debug_display_function_call1() {
    let timer = std::time::Instant::now();
    say!(#timer.elapsed());
}

#[test]
fn debug_display_function_call2() {
    fn func_test(arg: &str) -> String {
        format!("string from function: {arg}")
    }

    say!(#func_test("test"), "ing this method");
}

#[test]
fn debug_display_expression() {
    let string_vec = vec!["Hello ", "World"];

    say!(#string_vec);
}

#[test]
fn pretty_debug_display() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    say!(Pretty nested);
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
    say!(Dark "This is dimmed text for some reason. Don't know why you would even want this tbh");
}

#[test]
fn escaped_braces() {
    // Test that `{}` and `{:?}` in string literals are treated as literal text
    say!("This has {} braces");
    say!(Red "Debug format {:?} should work");
    say!(Green "Multiple ", Blue "{} and {:?} and {:#?}", Yellow " braces");
}

#[test]
fn accidental_func_call() {
    say!(Dark(String::from("ambiguous") + " whoops"));
}

#[test]
fn accidental_struct() {
    say!(Dark {123 + 456});
}

#[test]
fn path() {
    say!(#std::path::PathBuf::from("/tmp/test.txt"));
}

#[test]
fn macro_in_macro() {
    say!(#say!("Hello from inside a macro!"));
}

// These should always fail
// #[test]
// fn bad_syntax() {
//     say!(Red);
//     say!(, Red);
//     say!(,,, Green);
//     say!(Green,,Red,);
// }

// Prints a load of complex strings
// So can easily see them and make sure all the formatting is correct
#[test]
fn yap() {
    say!("debug: ", #[0,2,4,6], "pretty: ", Pretty [0, 1, 2, 3]);
    say!(Inline Yellow " YELLOW INLINE");
    say!("Some empty newlines: ");
    say!();
    say!();
    say!('\n');
    say!();
    // Hex / Binary formatters
    let var = String::new();
    say!(Green "Hex: ", #x 255); // 0xff
    say!(Blue "Binary: ", #b 15); // 0b1111
    say!(Red "Octal: ", #o 64); // 0o100
    say!(Yellow "Pointer: ", #p &var); // 0x7fff...
    say!(Green "Scientific: ", #e 123456.789); // 1.23456789e5
    say!(Blue "Scientific: ", #E 0.000123); // 1.23E-4

    say!();

    // Colours
    say!(Red "Red", Blue "Blue", Green "Green", Yellow "Yellow", Magenta "Magenta", White "White");
    say!(Red "Red", Bright Blue "Bright Blue", Bright Green "Bright Green", Bright Yellow "Bright Yellow", Bright Magenta "Bright Magenta", Bright White "Bright White");
    say!(Red "Red", Dark Blue "Dark Blue", Dark Green "Dark Green", Dark Yellow "Dark Yellow", Dark Magenta "Dark Magenta", Dark White "Dark White");
    say!(Red "Red", Italic "Italic", Underline "Underline", Invert "Invert");
    say!(Red "Red", Bold "Bold", Dark "Dark", Italic "Italic", Underline "Underline", Invert "Invert");
    say!(Red "Red", Bright Bold "Bright Bold", Dark Bold "Dark Bold", Italic Bold "Italic Bold", Underline Bold "Underline Bold", Invert Bold "Invert Bold");

    // Highlights
    say!(Black YellowHL "YellowHL");
    say!(Black YellowHL "YellowHL", Bright "Bright ", Dark "Dark ", Italic "Italic ", Underline "Underline ", Invert "Invert ");
    say!(Black YellowHL "YellowHL", Bright Bold "Bright Bold ", Dark Bold "Dark Bold ", Italic Bold "Italic Bold ", Underline Bold "Underline Bold ", Invert Bold "Invert Bold ");
    say!(Black YellowHL "YellowHL", Bright Blue "Bright Blue ", Dark Blue "Dark Blue ", Italic Blue "Italic Blue ", Underline Blue "Underline Blue ", Invert Blue "Invert Blue ");
    say!(Black YellowHL "YellowHL", Bright Green "Bright Green ", Dark Green "Dark Green ", Italic Green "Italic Green ", Underline Green "Underline Green ", Invert Green "Invert Green ");
    say!(Black YellowHL "YellowHL", Bright Yellow "Bright Yellow ", Dark Yellow "Dark Yellow ", Italic Yellow "Italic Yellow ", Underline Yellow "Underline Yellow ", Invert Yellow "Invert Yellow ");
}
