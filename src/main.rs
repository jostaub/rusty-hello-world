fn main(){
    println!("Hello rusty World!");
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_main(){
        main();
    }
}