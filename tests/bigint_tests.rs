extern crate numerics;


static TEST_VECTOR: &str = include_str!("./test_vectors/mul_test.txt");


#[test]
fn mul_test_vector_test() {
    use numerics::bigint::BigInt;
    use std::str::FromStr;

    for line in TEST_VECTOR.lines() {
        let mut ele = line.split_whitespace();
        let (a, b, c) = (
            ele.next().unwrap(),
            ele.next().unwrap(),
            ele.next().unwrap(),
        );
        let a = BigInt::from_str(a).unwrap();
        let b = BigInt::from_str(b).unwrap();
        let c = BigInt::from_str(c).unwrap();
        assert_eq!(a * b, c);
    }
}
