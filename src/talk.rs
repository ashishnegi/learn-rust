enum TrafficLight {
    Red,
    Yello,
    Green
}

struct Color {
    red : u32,
    green : u32,
    blue : u32
}

struct Coordinate {
    x : u32,
    y : u32
}

enum Event {
    ClickEvent(Coordinate),
    PaintEvent(Color)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handleEvent(event : Event) {
        match event {
            Event::ClickEvent(coordinate) => handleClickEvent(coordinate),
            Event::PaintEvent(color) => handlePaintEvent(color)
        }
    }

    fn handleClickEvent(coordinate : Coordinate) {
        println!("In handle click event")
    }

    fn handlePaintEvent(color : Color) {
        println!("In handle Print event")
    }

    #[test]
    fn talk_handleevent() {
        handleEvent(Event::ClickEvent (Coordinate { x : 10, y : 20} ))
    }
}