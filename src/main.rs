use ggez::*;
use nalgebra;

// TODO: Lets try and use specs (an ECS): https://github.com/ggez/game-template

fn main() {
  // Make a Context and an EventLoop.
  let (mut ctx, mut event_loop) = ContextBuilder::new("Some Dumb Game", "Tom Smallridge")
    .build()
    .unwrap();

  // Create an instance of your event handler.
  // Usually, you should provide it with the Context object
  // so it can load resources like images during setup.
  let mut my_game = DumbGame::new(&mut ctx);

  // Run!
  match event::run(&mut ctx, &mut event_loop, &mut my_game) {
    Ok(_) => println!("Exited cleanly."),
    Err(e) => println!("Error occured: {}", e),
  }
}

struct DumbGame {
  dt: std::time::Duration,
  // font: graphics::Font,
  text: graphics::Text,
  screen_dimensions: (f32, f32),
}

impl DumbGame {
  pub fn new(ctx: &mut Context) -> DumbGame {
    // Load/create resources here: images, fonts, sounds, etc.
    DumbGame {
      dt: std::time::Duration::new(0, 0),
      text: graphics::Text::new(format!("dt: {}", 0)),
      screen_dimensions: graphics::drawable_size(ctx),
    }
  }
}

impl event::EventHandler for DumbGame {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    // Update code here...
    self.dt = timer::delta(ctx);
    self.text = graphics::Text::new(format!("dt: {:?}", self.dt));
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, graphics::BLACK);

    let my_dest = nalgebra::Point2::new(
      (self.screen_dimensions.0 / 2.0) - (self.text.width(ctx)/2) as f32,
      (self.screen_dimensions.1 / 2.0) - (self.text.height(ctx)/2) as f32,
    );
    graphics::draw(
      ctx,
      &self.text,
      graphics::DrawParam::default().dest(my_dest),
    )?;

    graphics::present(ctx)
  }
}
