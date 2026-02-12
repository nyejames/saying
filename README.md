<div align="center">

# Saying
Formatted printing macro `say!` for easy, zero-cost ANSI SGR colours and styles

[![crates.io](https://img.shields.io/crates/v/saying.svg)](https://crates.io/crates/saying)
[![crates.io](https://img.shields.io/crates/l/saying)](https://crates.io/crates/saying)
[![crates.io](https://img.shields.io/deps-rs/saying/latest)](https://crates.io/crates/saying)
[![crates.io](https://img.shields.io/crates/size/saying)](https://crates.io/crates/saying)


</div>

---

<em style="color: gold;">

- Not compatible with non-ANSI terminals.
- Currently only supports the basic 8 colours + 8 HL variants.
- Does not respect NO_COLOR env var or perform runtime checks for compatibility.

</em>

<br>

```rust
say!(Blue Bold "The most ergonomic printing macro.", Italic " Is finally here");
```

<br>
<div align="center" >
<code style="font-size: 1.5em">cargo add saying</code>
</div>

## Overview
- Zero dependencies (only uses std)
- No runtime overhead, just parses into a single println!() at compile time using macro_rules (no proc macros)
- Ergonomic and readable
- Any expression can just be used as an argument in position

Prints to stdout only and automatically adds a newline (unless you use the Inline style).

Once a colour or style is set, all proceeding arguments will use the same style until it is changed or reset.

Just chuck some styles in front of your expressions (no need for anything between them), and you're good to go.

```rust
// Basic usage 
say!("Hi");
say!(Green "Hi");
say!(Yellow 2 + 2);

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
Positional formatting is not supported. This is an arguably a more readable alternative.

Any expression can have colours / styles in front of them.
Even if it's not a string literal.

You can use # in front of an expression to debug print it. 
Use the keyword "Pretty" to pretty print it.

```rust
// Using Variables in the macro
let subject = "world";
let collection = vec![0, 1, 2];

say!(Red Underline "Hello ", Cyan subject, "!");

// Using expressions in the macro -
// You can optionally wrap expressions in parentheses or brackets if you really want.
// But due to BIG BRAIN PARSING™ this is not required. 
// You can just yeet expressions straight into the macro.
say!(Dark Magenta "Extreme Mathematics ", Bright 2 + 1);

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

## Debug Printing
You can use the # keyword to debug print expressions.
The "Pretty" keyword pretty prints expressions.

The macro also supports various other formatting prefixes that start with a #:
```rust
let var = String::new();
say!(Green "Hex: ", #x 255);                 // 0xff
say!(Blue "Binary: ", #b 15);                // 0b1111
say!(Red "Octal: ", #o 64);                  // 0o100
say!(Yellow "Pointer: ", #p &var);           // 0x7fff...
say!(Green "Scientific: ", #e 123456.789);   // 1.23456789e5
say!(Blue "Scientific: ", #E 0.000123);      // 1.23E-4
```

<br>

**Loads of usage examples can be found in the [test file](https://github.com/nyejames/saying/blob/master/src/tests.rs).**

## Errors
You will get all the normal print!() errors if your arguments aren't valid.
But the macro does throw a CompileError when you have styles in a macro, but no expressions.

## Future
This may be extended for advanced styling in the future with backwards compatibility. 
This will allow opting into higher compatibility risk for the sake of some more pretty colours. Yay.

There may also be an additional macro from this library in the future `yell()` that does have some runtime overhead, 
so it can make sure the terminal supports more advanced colours before adding the codes.
