<div align="center">

# Saying
### Formatted printing macro `say!` for easy, zero-cost ANSI SGR colours and styles

</div>

---


*Not compatible with non-ANSI terminals.*
*Only supports the basic 24-bit colour codes for maximum compatibility and simplicity.*

<br>

```rust
say!(Blue Bold "The most ergonomic printing macro");
```

## Overview
- Ergonomic as possible
- Zero dependencies (only uses std)
- No runtime overhead, just parses into a single println!() at compile time using macro_rules

Prints to stdout only and automatically adds a newline.

Once a colour or style is set, all proceeding arguments will use the same style until it is changed or reset.

```rust
// Basic usage 
say!("Hi");
say!(Green "Hi");
say!(Yellow "Hi");

// Multiple arguments
say!("Hi", " there");

// Mixing different styles on the same line
say!(Red "Hi", Blue " there");

// Multiple styles and multiple arguments
// Using numbered colours for different palettes
say!(
    Red "Hello ", 
    Blue Bold " World", 
    Red "!",
);
```

## Formatting Arguments
Instead of using the println "{}" syntax, you just add your expressions in order.

Any of those expression can have colours / styles in front of them.

You can use # in front of an expression to debug print it. Use the keyword "Pretty" in front of this to also pretty print it.

```rust
// Using Variables in the macro
let subject = "world";
let collection = vec![0, 1, 2];

say!(Red Underline "Hello ", Orange subject, "!");

// With a debug print
say!(Green "Numbers: ", #collection);

// With a pretty debug print
say!(Blue "Numbers: ", Pretty #collection);
```

## Colours
There will be a variety of basic colour keywords:
- Black
- White
- Red
- Green
- Yellow
- Blue
- Magenta
- Cyan

## Styles
In conjunction with colours, basic style keywords can be used.
- Reset (Resets all colours and styles)
- Bold (This can affect brightness in some terminals)
- Underline
- Inline (Removes the automatically inserted newline)
- Invert (Swapped background and foreground colour)
- Italics (Not as widely compatible)

## Highlights
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

<br>

### Future
This may be extended to 256 colours and more advanced styling in the future with backwards compatibility. This will allow opting into higher compatibility risk for the sake of some more pretty colours. Yay.