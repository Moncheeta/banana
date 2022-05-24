use std::result::Result;

use banana::editor::main_loop;

fn main() -> Result<(), &'static str> {
    main_loop();
    Ok(())
}
