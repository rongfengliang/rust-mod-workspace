#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod user {
    #[derive(Debug)]
    pub struct User {
     pub name: String,
     pub age: u32,
    }
}

pub mod aaa {
    const X: i32 = 10;

    pub fn print_aaa() {
        println!("{}", 42);
    }
}