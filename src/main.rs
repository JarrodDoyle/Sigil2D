mod game;
mod gfx;
mod input;
mod scripting;

use anyhow::Result;
use game::{Game, GameConfig};

pub fn main() -> Result<()> {
    env_logger::init();

    Game::new(GameConfig::default())?.run()?;

    Ok(())
}
