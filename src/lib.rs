mod error;
mod types;
mod resources;
mod desc;
mod handle;
mod slotmap;
mod context;

mod sort_key;
mod commands;
mod graph;
mod camera;
mod pass;
mod debug;
mod renderer;
mod memory;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
