extern crate numerics;


static MUL_TEST_VECTOR: &str = include_str!("./test_vectors/mul_test.txt");
static SUB_TEST_VECTOR: &str = include_str!("./test_vectors/sub_test.txt");
static ADD_TEST_VECTOR: &str = include_str!("./test_vectors/add_test.txt");
static DIV_TEST_VECTOR: &str = include_str!("./test_vectors/div_test.txt");

#[test]
fn mul_test_vector_test() {
    use numerics::bigint::BigInt;
    use std::str::FromStr;

    for line in MUL_TEST_VECTOR.lines() {
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

#[test]
fn sub_test_vector_test() {
    use numerics::bigint::BigInt;
    use std::str::FromStr;

    for line in SUB_TEST_VECTOR.lines() {
        let mut ele = line.split_whitespace();
        let (a, b, c) = (
            ele.next().unwrap(),
            ele.next().unwrap(),
            ele.next().unwrap(),
        );
        let a = BigInt::from_str(a).unwrap();
        let b = BigInt::from_str(b).unwrap();
        let c = BigInt::from_str(c).unwrap();
        assert_eq!(a - b, c);
    }
}

#[test]
fn add_test_vector_test() {
    use numerics::bigint::BigInt;
    use std::str::FromStr;

    for line in ADD_TEST_VECTOR.lines() {
        let mut ele = line.split_whitespace();
        let (a, b, c) = (
            ele.next().unwrap(),
            ele.next().unwrap(),
            ele.next().unwrap(),
        );
        let a = BigInt::from_str(a).unwrap();
        let b = BigInt::from_str(b).unwrap();
        let c = BigInt::from_str(c).unwrap();
        assert_eq!(a + b, c);
    }
}

#[test]
fn div_test_vector_test() {
    use numerics::bigint::BigInt;
    use std::str::FromStr;

    for line in DIV_TEST_VECTOR.lines() {
        let mut ele = line.split_whitespace();
        let (x, y, q, r) = (
            ele.next().unwrap(),
            ele.next().unwrap(),
            ele.next().unwrap(),
            ele.next().unwrap(),
        );
        let x = BigInt::from_str(x).unwrap();
        let y = BigInt::from_str(y).unwrap();
        let q = BigInt::from_str(q).unwrap();
        let r = BigInt::from_str(r).unwrap();
        assert_eq!(x.div_mod(&y), (q, r));
    }
}