mod automaton;
mod config;
mod render;
use config::*;
use render::{render_loop, Init};

pub fn main() {
    let load_preset = false;
    if load_preset{
        render_loop(Init::Cells(Vec::from(TRAFFIC_LIGHT)));
        render_loop(Init::Cells(Vec::from(FROG)));
        render_loop(Init::Cells(Vec::from(PLANE)));
    }else{
        render_loop(Init::Random);
    }
}
