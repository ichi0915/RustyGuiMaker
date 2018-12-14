#![allow(non_snake_case)]

pub fn hello() {
    println!("Rusty GUI Maker is alive!");
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//cargo doc --open
