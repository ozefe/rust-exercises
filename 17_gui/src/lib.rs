pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button {:?} has been drawn!", &self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_create() {
        Button {
            width: 10,
            height: 20,
            label: String::from("Button"),
        };
    }

    #[test]
    fn button_draw() {
        let button = Button {
            width: 10,
            height: 20,
            label: String::from("Button"),
        };

        button.draw();
    }

    #[test]
    fn screen_create() {
        Screen {
            components: vec![Box::new(Button {
                width: 10,
                height: 20,
                label: String::from("Button"),
            })],
        };
    }

    #[test]
    fn screen_run() {
        let screen = Screen {
            components: vec![Box::new(Button {
                width: 10,
                height: 20,
                label: String::from("Button"),
            })],
        };

        screen.run();
    }
}
