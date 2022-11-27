use bracket_lib::prelude::*;

// GameMode is the games state machine, tells the game
// what to do on the current tick.
enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

struct Player {
    x: i32,        // position of player's progress through the level.
    y: i32,        // vertical position of player in screen-space.
    velocity: f32, // vertical velocity.
}
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    // render will render the player, aa '@' on the screen with `ctx.set`.
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            // apply gravity if downward momentum is less than 2.
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    // flap sets velocity to a negative number to move the character upwards.
    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

// State stores the game's current status.
struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}
impl State {
    fn new() -> Self {
        State {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // specify background color
        self.frame_time += ctx.frame_time_ms; // slow the game down, as tick() actually runs over 60 times ps, too fast.

        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);

        ctx.print(0,0,"Press SPACE to flap");
        if self.player.y > SCREEN_HEIGHT { // play fell off the screen.
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
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
