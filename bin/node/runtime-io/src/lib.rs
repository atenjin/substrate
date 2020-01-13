#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime_interface::{runtime_interface};

#[runtime_interface]
pub trait Local {
    fn print_ext(utf8: &[u8]) {
        if let Ok(data) = std::str::from_utf8(utf8) {
            println!("{}", data)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
