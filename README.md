# Saying
Formatted printing adding ANSI SGR parameters.

Not compatible with non-ANSI terminals.

```rust
say!(Blue Bold "The most ergonomic printing macro for ansi");
```

## Goals
- Ergonomic as possible
- Zero dependencies (except for std)
- No runtime overhead, just parses into a single println!() using macro_rules

## Overview
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
    Red1 "Hello ", 
    Blue Bold " World", 
    Red6 "!",
);

// Using Variables in the macro
let subject = "world";
let collection = vec![0, 1, 2];

say!(Red10 Underline "Hello ", Orange2 subject, "!");

// With a debug print
say!(Green "Numbers: ", #collection);

// With a pretty debug print
say!(Blue7 "Numbers: ", Pretty #collection);

```

### Colours
There will be a variety of basic colour keywords, then a number with the range of 1 to 10.

Without the number, the colour is fully saturated.

Numbered colours above 5 become brighter, numbers below 5 become darker.

Colour can also be a highlight for the background of the text. 
You can change the background colour by adding "HL" after the colour name.

```rust
// Basic usage 
say!(RedHL "Hi");
say!(GreenHL8 "Hi");
say!(Black YellowHL1 "Hi");
```

### Styles
In conjunction with colours, basic style keywords can be used.
- Reset (Resets all colours and styles)
- Bold (This can affect brightness in some terminals)
- Underline
- Invert (Swapped background and foreground colour)
- Italics (Not as widely compatible)
