// #[derive(Debug)]
// enum Maybe<T> {
//     Nothing,
//     Just(T),
// }

// fn plus_one(there: Maybe<u32>) -> Maybe<u32> {
//     match there {
//         Nothing => Nothing,
//         Just(v) => Just(v+1),
//     }
// }


fn plus_one(there: Option<u32>) -> Option<u32> {
    match there {
        None => None,
        Some(v) => Some(v+1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_nothing() {
        let v = Some(1);
        if let Some(inc_val) = plus_one(v) {
            assert_eq!(inc_val, 2);
        } else {
            assert_eq!(None, v);
        }
    }
}
