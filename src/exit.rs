use crossterm::style::Stylize;

use crate::TaskError;

pub fn success(comps: &[&dyn std::fmt::Display]) -> Result<(), TaskError> {
    print!("{}", "SUCCESS!".on_green());
    for comp in comps {
        print!(" {}", comp.to_string().green())
    }
    println!();
    Ok(())
}

pub fn print_error(err: &TaskError) {
    eprintln!("{} {}", "ERROR!".on_dark_red(), err.to_string().dark_red());
}
