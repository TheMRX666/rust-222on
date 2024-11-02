#[test]
fn test1611() {
    fn main(){
        let s1 = "hello";
        let s = format!("{}, {}!", s1, "world");
        assert_eq!(s, "hello, world!");
    }
    main()
}