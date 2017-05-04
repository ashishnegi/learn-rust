fn largest<T: PartialOrd>(ts: &[T]) -> &T {
    let mut large = &ts[0];
    for t in ts.iter() {
        if large < t {
            large = t;
        }
    }
    large
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_test() {
        assert_eq!(4, *largest(&[1,2,3,4,2]));
    }
}
