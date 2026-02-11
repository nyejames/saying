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
    let messages = vec![1, 2, 3];
    say!(Yellow "There are ", messages.len(), " messages");
    say!(Blue "Count: ", messages.len().to_string(), "!");
    say!(Blue "Count: ", messages.len().to_string(), "!");
}

#[test]
fn style_before_expression() {
    let message = "yo";
    say!(Cyan message, ", whats up?");
}

#[test]
fn debug_display_expression() {
    let string_vec = vec!["Hello ", "World"];

    say!(#string_vec);
}


#[test]
fn debug_display_function_call() {
    fn func_test(arg: &str) -> String {
        format!("arg: {arg}")
    }

    say!(#func_test("test"));
}

