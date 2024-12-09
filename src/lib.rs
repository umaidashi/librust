pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod umaidashi {
    pub fn umaidashi() {
        println!("うまいだし");
    }
}

#[cfg(test)]
pub mod tests {
    //use super::*;

    #[test]
    fn it_works() {
        let result = crate::add(2, 2);
        crate::umaidashi::umaidashi();
        assert_eq!(result, 4);
    }
}
