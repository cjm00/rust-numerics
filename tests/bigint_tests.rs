extern crate numerics;


static test_vec: &str = include_str!("./test_vectors/mul_test.txt");


#[test]
fn mul_test_vector_test() {
    use numerics::bigint::BigInt;

    for line in test_vec.lines() {
        let mut ele = line.split_whitespace();
        let (a, b, c) = (
            ele.next().unwrap(),
            ele.next().unwrap(),
            ele.next().unwrap(),
        );
        let a = BigInt::from_str_radix(a, 10).unwrap();
        let b = BigInt::from_str_radix(b, 10).unwrap();
        let c = BigInt::from_str_radix(c, 10).unwrap();
        assert_eq!(a * b, c);
    }
}
