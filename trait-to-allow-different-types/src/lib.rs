pub trait Drawable {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Drawable>>
}

impl Screen {
    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Drawable for Button {
    
    fn draw(&self) {
        println!("{}", "-".repeat(self.label.len() + 2));
        println!("-{}-", self.label);    
        println!("{}", "-".repeat(self.label.len() + 2));
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Drawable for SelectBox {
    fn draw(&self) {
        println!("Select Box");
        println!("width: {}", self.width);
        println!("height: {}", self.height);
        println!("Options: ");
        for option in &self.options {
            println!("{}", option);
        }
    }
}

impl Drawable for String {
    fn draw(&self) {
        println!("{}", self);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
