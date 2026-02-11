<div align="center">

# Saying
Formatted printing macro `say!` for easy, zero-cost ANSI SGR colours and styles

</div>

---
<em style="color: gold;">
<br>

Not compatible with non-ANSI terminals.
Currently only supports the basic 24-bit colour codes.

</em>

<br>

```rust
say!(Blue Bold "The most ergonomic printing macro");
```

## Overview
- Zero dependencies (only uses std)
- No runtime overhead, just parses into a single println!() at compile time using macro_rules
- Ergonomic and readable

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
Instead of using the println "{}" syntax, you add your expressions in order.
Positional formatting is not supported.

Any expression can have colours / styles in front of them.

You can use # in front of an expression to debug print it. Use the keyword "Pretty" in front of this to also pretty print it.

```rust
// Using Variables in the macro
let subject = "world";
let collection = vec![0, 1, 2];

say!(Red Underline "Hello ", Orange subject, "!");

// Using expressions in the macro
say!(Dark Magenta "Extreme Mathematics ", Bright {2 + 1});

// With a debug print
say!(Green "Numbers: ", #collection);

// With a pretty debug print
say!(Blue "Numbers: ", Pretty #collection);



```

## Colours
All the basic colour keywords:
- <p style="color: white">White</p>
- <p style="color: black">Black</p>
- <p style="color: red">Red</p>
- <p style="color: green">Green</p>
- <p style="color: yellow">Yellow</p>
- <p style="color: blue">Blue</p>
- <p style="color: magenta">Magenta</p>
- <p style="color: cyan">Cyan</p>

## Styles
In conjunction with colours, basic style keywords can be used.

**Styling**
- Bold (This can affect brightness in some terminals)
- Underline
- Invert (Swapped background and foreground colour)
- Italics (Not as widely compatible)
- Bright (Makes the colours brighter)
- Dark (Makes the colours darker but can be inconsistent in different terminals)

**Formatting**
- Reset (Resets all colours and styles)
- Inline (Keeps this print inline with the previous one, removing the automatically inserted newline)

## Highlights

Colour can also be a highlight for the background of the text. 
You can change the background colour by adding "HL" after the colour name:
- <mark style="background-color: black; color: white;">BlackHL</mark>
- <mark style="background-color: white; color: black;">WhiteHL</mark>
- <mark style="background-color: red;">RedHL</mark>
- <mark style="background-color: green;">GreenHL</mark>
- <mark style="background-color: yellow;">YellowHL</mark>
- <mark style="background-color: blue;">BlueHL</mark>
- <mark style="background-color: magenta;">MagentaHL</mark>
- <mark style="background-color: cyan;">CyanHL</mark>

```rust
// Basic usage 
say!(RedHL "Hi");
say!(GreenHL "Hi");
say!(Black YellowHL "Hi");
```

<br>

### Future
This may be extended to 256 colours and more advanced styling in the future with backwards compatibility. This will allow opting into higher compatibility risk for the sake of some more pretty colours. Yay.