pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod umaidashi {
    pub fn umaidashi() {
        println!("うまいだし");
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

#[cfg(test)]
pub mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        let result = crate::add(2, 2);
        assert_eq!(result, 4);
    }
}
