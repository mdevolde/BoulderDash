use super::{enums::field::Field, interfaces::renderable::Renderable};

pub struct Tile {
    x: i32,
    y: i32,
    field: Field,
}

impl Tile {
    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    
    pub fn get_object_on(&self) -> Option<&Field> {
        match &self.field {
            Field::Entity(_) | Field::Wall(_) | Field::Exit => Some(&self.field),
            _ => None,
        }
    }

    pub fn set_object_on(&mut self, field: Field) {
        self.field = field;
    }
}

impl Renderable for Tile {
    fn render(&self) {
        match &self.get_object_on() {
            Some(Field::Entity(entity)) => entity.render(),
            Some(Field::Wall(wall)) => wall.render(),
            Some(Field::Dirt) => println!("Dirt tile at ({}, {})", self.x, self.y), // Temporary implementation
            Some(Field::Exit) => println!("Exit tile at ({}, {})", self.x, self.y), // Temporary implementation
            Some(Field::Empty) | None => println!("Empty tile at ({}, {})", self.x, self.y), // Temporary implementation
        }
    }
}
