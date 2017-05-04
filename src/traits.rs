fn largest<T: PartialOrd>(ts: &[T]) -> &T {
    let mut large = &ts[0];
    for t in ts.iter() {
        if large < t {
            large = t;
        }
    }
    large
}

trait Talkable {
    fn say(&self) -> String {
        String::from("i can talk.. a lot.")
    }
}

struct Human {}
struct Cat {}

impl Talkable for Human {
    fn say(&self) -> String {
        String::from("i am human.. Are you also ?")
    }
}

impl Talkable for Cat {
    fn say(&self) -> String {
        String::from("meow..")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_test() {
        assert_eq!(4, *largest(&[1,2,3,4,2]));
    }

    #[test]
    fn say_my_name() {
        let human = Human{};
        let cat = Cat{};
        assert_ne!(human.say(), cat.say());
    }
}
