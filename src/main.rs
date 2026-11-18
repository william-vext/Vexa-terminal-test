mod vext_test;
use vext_test::{Vexa,Vexas};
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";
fn main() -> std::io::Result<()> {
    let title = format!(r#"
__     __                  
\ \   / /  ___  __  __  __ _ 
 \ \ / /  / _ \ \ \/ / / _` |
  \ V /  |  __/  >  < | (_| |
   \_/    \___| /_/\_\ \__,_|
   "#);
    println!("\n{YELLOW}{title}{RESET}\n");
    println!("Welcome to Vexa\n");
    loop {
        if !Vexa::vexa()? {
            break;
        }
    }
    Ok(())
}
