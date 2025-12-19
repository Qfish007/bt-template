#![allow(unused)]
fn test() {
    println!("Hello, world!！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        test();
    }
}
