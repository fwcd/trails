# Trails

A small web browser written from scratch using nothing but an HTTP library.

<img width="912" alt="Screenshot" src="screenshots/app.png">

## Roadmap

- Meta/Packaging
    - [ ] Design and add proper app icon
- GUI
    - [x] Very basic setup
    - [x] Proper factoring of views into different modules
    - [x] Look into Piet and custom Druid widgets for the rendering engine
- Networking
    - [x] Very basic setup
    - [ ] Persistence/cookies
    - [ ] Async/await
- Parsing
    - [x] Very basic, recursive-descent HTML parser (still has a number of bugs, in particular the attributes aren't really parsed correctly yet...)
    - [ ] CSS parser
- Rendering
    - [x] Very basic rendering engine setup (with custom widget and a Piet drawing context)
    - [x] Very basic HTML markup rendering
    - [ ] Very basic CSS rendering
- JS, Security, ...
    - Let's not get ahead of ourselves here...

## Running

The brave souls willing to try this highly experimental app may use `cargo run` to run it.
