# Saying
Formatted printing adding ANSI SGR parameters.

Not compatible with non-ANSI terminals.

```rust
say!(Blue Bold "The most ergonomic printing macro");
```

## Overview
- Ergonomic as possible
- Zero dependencies (except for std)
- No runtime overhead, just parses into a single println!() using macro_rules

Exports one macro. This prints to stdout only and automatically adds a newline.

Once a colour or style is set, all proceeding arguments will use the same style until it is changed or reset.

```rust
// Basic usage 
say!("Hi");
say!(Green "Hi");
say!(Yellow "Hi");

// Multiple arguments
say!("Hi", " there");

// Mixing different styles on the same line
say!(Red "Hi", Blue" there");

// Multiple styles and multiple arguments
// Using numbered colours for different palettes
say!(
    Red "Hello ", 
    Blue Bold " World", 
    Red "!",
);

// Using Variables in the macro
let subject = "world";
let collection = vec![0, 1, 2];

say!(Red Underline "Hello ", Orange subject, "!");

// With a debug print
say!(Green "Numbers: ", #collection);

// With a pretty debug print
say!(Blue "Numbers: ", Pretty #collection);

```

### Colours
There will be a variety of basic colour keywords:
- Black
- White
- Red
- Green
- Yellow
- Blue
- Magenta
- Cyan

### Highlights
Colour can also be a highlight for the background of the text. 
You can change the background colour by adding "HL" after the colour name:
- BlackHL
- WhiteHL
- RedHL
- GreenHL
- YellowHL
- BlueHL
- MagentaHL
- CyanHL

```rust
// Basic usage 
say!(RedHL "Hi");
say!(GreenHL "Hi");
say!(Black YellowHL "Hi");
```

### Styles
In conjunction with colours, basic style keywords can be used.
- Reset (Resets all colours and styles)
- Bold (This can affect brightness in some terminals)
- Underline
- Invert (Swapped background and foreground colour)
- Italics (Not as widely compatible)
