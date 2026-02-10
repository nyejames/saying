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
