#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

struct Square {
    side: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_rectangle() {
        let rec = Rectangle{
            width: 4,
            height: 5,
        };

        // println!("rect is {:?}", rec); // uses derive(Debug)
        assert_eq!(20, rec.area());
    }

    #[test]
    fn area_square() {
        let squ = Square{
            side: 5,
        };

        assert_eq!(25, squ.area());
    }
}
