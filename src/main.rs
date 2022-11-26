use bracket_lib::prelude::*;

// GameMode is the games state machine, tells the game
// what to do on the current tick.
enum GameMode {
    Menu,
    Playing,
    End,
}

// State stores the game's current status.
struct State {
    mode: GameMode,
}
impl State {
    fn new() -> Self {
        State{
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stub
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // let Some is a shortcut for match against a single case.
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {} // ignore any options we didnt' list.
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    // tick acts as a bridge between the game engine and our game,
    // directing program flow based on the current mode.
    // &mut self allows changing the state instance.
    // ctx enables interaction with the game display
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    // Returning BError, which is a `Result` type.
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?; // ? passes errors up to the parent function.

    main_loop(context, State::new()) // implicit error returning, no semicolon needed.
}
