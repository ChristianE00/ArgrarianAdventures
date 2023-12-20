use ggez::event::{self, KeyCode, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use nalgebra::ComplexField;

trait Collidable {
    fn get_position(&self) -> [f32; 2];
    fn get_dimensions(&self) -> (f32, f32); // Returns width and height
}

struct Grass {
    image: Image,
    position: [f32; 2],

}

impl Collidable for Grass {
    fn get_position(&self) -> [f32; 2] {
        self.position
    }

    fn get_dimensions(&self) -> (f32, f32) {
        (self.image.width() as f32, self.image.height() as f32)
    }
}


struct MainState {
    grass: Grass,
    sprite: Image,
    position: [f32; 2], //x, y position of the sprite
    velocity: [f32; 2], //x, y velocity of the sprite
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let sprite = Image::new(ctx, "/cat_pet.png")?;
        //let grass = Image::new(ctx, "/grass.png")?;
        let grass = Grass {
            image: Image::new(ctx, "/grass.png")?,
            position: [0.0, 0.0],
        };
        Ok(MainState {
            grass,
            sprite,
            position: [100.0, 100.0], // Initial position
            velocity: [0.0, 0.0],     // Initial velocity
        })
    }

    fn update_position(&mut self) {
        // Update position based on velocity

        if self.velocity[0].abs() + self.velocity[1].abs() > 5.0 {
            self.velocity[0] = self.velocity[0] / 1.8;
            self.velocity[1] = self.velocity[1] / 1.8;
        }
        if self.is_colliding_with_grass() {
            self.velocity[0] = 0.0;
            self.velocity[1] = 0.0;
            println!("Collided with grass: {:?}", self.position);
        }
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
    }

    fn is_colliding_with_grass(&self) -> bool {
        let sprite_l = self.position[0] + self.velocity[0];
        let sprite_r = self.position[0] + self.sprite.width() as f32 + self.velocity[0];
        let sprite_t = self.position[1] + self.velocity[1];
        let sprite_b = self.position[1] + self.sprite.height() as f32 + self.velocity[1];

        let grass_l = 0.0;
        let grass_r = self.grass.image.width() as f32;
        let grass_t = 0.0;
        let grass_b = self.grass.image.height() as f32;

        sprite_r > grass_l && sprite_l < grass_r && sprite_b > grass_t && sprite_t < grass_b
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.velocity = [0.0, 0.0]; // Reset velocity
        if keyboard::is_key_pressed(_ctx, KeyCode::Left) {
            self.velocity[0] -= 10.0; // Move left
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::Right) {
            self.velocity[0] += 10.0; // Move velocity}
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::Up) {
            self.velocity[1] -= 50.0; // Move up
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::Down) {
            self.velocity[1] += 50.0; // Move down
        }
        self.update_position(); // Update position based on velocity
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::WHITE);
        graphics::draw(ctx, &self.grass.image, DrawParam::new().dest(self.grass.position))?;
        graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))?;

        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Argrarian Adventures", "Chris")
        .window_setup(ggez::conf::WindowSetup::default().title("Agrarian Adventures"))
        .build()?; // Add ? at the end
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
