fn main(){
    println!("Hello World!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_main(){
        main();
    }
}