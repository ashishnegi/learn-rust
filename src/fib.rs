
pub fn fib(nth : i32) -> i32 {
    if nth < 1 {
        0
    } else {
        let (mut a, mut b, mut count) = (0, 1, 1);
        while count < nth {
            let c = a + b;
            a = b;
            b = c;
            count = count + 1;
        }

        a
    }
}
