use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Image, DrawParam};
use ggez::event::{self, KeyCode, KeyMods};
use ggez::input::keyboard;

struct MainState {
    sprite: Image,
    position: [f32; 2], //x, y position of the sprite
    velocity: [f32; 2], //x, y velocity of the sprite
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sprite = Image::new(ctx, "/cat_pet.png")?;
        Ok(MainState { sprite,
            position: [100.0, 100.0], // Initial position
            velocity: [0.0, 0.0], // Initial velocity
        })
    }
    
    fn update_position(&mut self){
        // Update position based on velocity
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
    } 
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.velocity = [0.0, 0.0]; // Reset velocity

        if keyboard::is_key_pressed(_ctx, KeyCode::Left) {
            self.velocity[0] -= 5.0; // Move left
        }

        if keyboard::is_key_pressed(_ctx, KeyCode::Right) {
            self.velocity[0] += 5.0; // Move velocity}
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::Up) {
            self.velocity[1] -= 5.0; // Move up
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::Down) {
            self.velocity[1] += 5.0; // Move down
        }
        self.update_position(); // Update position based on velocity
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);
        graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))?;
        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Argrarian Adventures", "Chris").window_setup(ggez::conf::WindowSetup::default().title("Agrarian Adventures")).build()?; // Add ? at the end
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

