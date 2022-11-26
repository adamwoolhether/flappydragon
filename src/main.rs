use bracket_lib::prelude::*;

// state stores the game's current status.
struct State {}

impl GameState for State {
    // tick acts as a bridge between the game engine and our game.
    // &mut self allows changing the state instance.
    // ctx enables interaction with the game display.
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls(); // clear the window.
        ctx.print(1, 1, "Hello, Bracket Terminal");
    }
}

fn main() -> BError { // Returning BError, which is a `Result` type.
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?; // ? passes errors up to the parent function.

    main_loop(context, State{}) // implicit error returning, no semicolon needed.
}
