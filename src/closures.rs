fn add_one<F>(closure: F, y: u32) -> u32
    where F: Fn(u32) -> u32
{
    closure(y) + 1
}

fn add(x: u32, y: u32) -> u32 {
    let (mut i, mut sum) = (0, y);
    while i < x {
        sum = add_one(|z| z, sum);
        i = i + 1;
    }
    sum
}


struct Even {
    num: u32,
    last: u32
}

impl Iterator for Even {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last > self.num {
            let num = self.num;
            self.num += 2;
            Some(num)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        assert_eq!(add(4,5), 9);
    }

    #[test]
    fn even_iter() {
        let x : u32 = Even{num: 0, last: 100}
            .map(|x| x + 1) // make odd..
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(867, x);
    }
}
