use crate::routers::root;

pub mod controllers;
pub mod routers;

fn main() {
    root::init();
}
