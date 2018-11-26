# OrbTk

[![Build status](https://gitlab.redox-os.org/redox-os/orbtk/badges/master/build.svg)](https://gitlab.redox-os.org/redox-os/orbtk/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.2.27-orange.svg)](https://crates.io/crates/orbtk)
[![docs.rs](https://docs.rs/orbtk/badge.svg)](https://docs.rs/orbtk)

> OrbTk 0.3.0 is under heavy development and it's not compatible to earlier releases.

The Orbital Widget Toolkit is a multi platform (G)UI toolkit for building scalable user interfaces with the programming language Rust. It's based
on the [Entity Component System Pattern](https://en.wikipedia.org/wiki/Entity%E2%80%93component%E2%80%93system) and provides a functional-reactive API. 

The main goals of OrbTk are fast performance, easy to use and the platform independence.

<img alt="Redox" height="300" src="https://gitlab.redox-os.org/redox-os/assets/raw/master/screenshots/Calculator.png">

## Features:

* Modern [Flutter](https://flutter.io/), [React](https://reactjs.org/), [Redux](https://redux.js.org/) like API
* Uses the Entity Component System library [DCES](https://gitlab.redox-os.org/redox-os/dces-rust) for widget and properties handling
* Updating instead of rebuilding subtrees
* Small DSL for custom widget creation on macro base (WIP)
* Flexible event system
* Widget state management
* Platform Independent: Redox OS, Linux, macOS, Windows
* CSS theming

## Usage

To include OrbTk in your project, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = "0.2.27"
```

To use OrbTk 0.3, just add the dependency
line to your `Cargo.toml` file:

```text
orbtk = { git = https://gitlab.redox-os.org/redox-os/orbtk.git }
```

However you also need to have the SDL2 libraries installed on your
system.  The best way to do this is documented [by the SDL2
crate](https://github.com/AngryLawyer/rust-sdl2#user-content-requirements).

## Minimal Example

> A macro to simplify the definition of widgets is in development.

```rust
extern crate orbtk;
use orbtk::*;
use std::rc::Rc;

struct MainView;

impl Widget for MainView {
    fn template(&self) -> Template {
        Template::Single(Rc::new(Container {
            child: Some(Rc::new(TextBlock {
                label: Property::new(Label(String::from("OrbTk"))),
                ..Default::default()
            })),
                ..Default::default()
        }))
    }
}

fn main() {
    let mut application = Application::default();
    application
        .create_window()
        .with_bounds(Rect::new(0, 0, 420, 730))
        .with_title("Orbtk")
        .with_root(MainView)
        .build();
    application.run();
}
```

## Additional Examples

You find the examples in the `examples/` directory.

You can start the widgets example by executing the following command:

```text
cargo run --example widgets
```

## Planned features

* Style guide
* More default widgets
* More examples
* Book
* Animations
* Exchange views / widgets / screens on runtime
* Split application in modules
* Theme update
* Support for Android, iOS and WebAssembly
* Vulkan / OpenGL Support 

## Inspirations

* [Flutter](https://flutter.io/)
* [React](https://reactjs.org/)
* [Yew](https://github.com/DenisKolodin/yew)

