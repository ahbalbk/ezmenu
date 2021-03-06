# EZMenu

Fast designing menus for your Rust CLI programs with a simple derive.

This crates provides many procedural macros to easily build menus.
It uses the [`ezmenulib`](https://docs.rs/ezmenulib) library crate in its expansion.

# Example

Here is an example of how to use the `derive(Menu)` macro
(it takes the same example as the documentation of the `ezmenu` library):
```rust
use ezmenu::Menu;

#[derive(Menu)]
#[menu(title = "Hello there!")]
struct MyMenu {
    #[menu(msg = "Give your name")]
    name: String,
    #[menu(msg = "Give a number")]
    number: i32,
}

fn main() {
    let MyMenu { name, number } = MyMenu::from_menu();
    println!("values provided: name={}, number={}", name, number);
}
```

This sample code prints the standard menu like above:

```text
Hello there!
* Give your name: Ahmad
* Give a number: 1000
values provided: name=Ahmad, number=1000
```

## Documentation

You can find all the crate documentation on [Docs.rs](https://docs.rs/ezmenu).
You can also check the [make-license program example](example) to learn with a practical way.

## WIP

This project is still in development.
You can check the [EZMenu project](https://github.com/users/ahbalbk/projects/4) to look at my todolist :D
