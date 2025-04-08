fn main() {
    println!("Hello, world!");
}

mod test {
    #[test]
    fn test() {
        let a = 5;

        assert_eq!(a, 5);
    }
}
