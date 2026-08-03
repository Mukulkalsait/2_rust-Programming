use std::fs::OpenOptions;

pub mod combinators;
pub mod filters;
pub mod interceptors;
pub mod operators;

fn main() {
    // operators::operators();
    operators::peekker();
    operators::consicitive_duplicate_detector();
    // combinators::combinators();
    // filters::filter();
    // interceptors::interceptors();
}
