use std::{any::Any, rc::Rc};

use crate::game::tile::Tile;

use super::{action::Action, enums::{field::Field, movement::Movement}, grid::Grid, interfaces::{collidable::Collidable, entity::Entity, fallable::Fallable, movable::Movable, renderable::Renderable}, player::Player};

#[derive(Clone)]
pub struct Diamond {
    position: (i32, i32),
    falling_since: i32,
}

impl Diamond {
    pub fn new(x: i32, y: i32) -> Self {
        Diamond {
            position: (x, y),
            falling_since: 0,
        }
    }
}

impl Movable for Diamond {
    fn move_to(&self, ax: i32, ay: i32, nx: i32, ny: i32) -> Vec<Action> {
        let mut actions = Vec::new();
        actions.push(Action::new((ax, ay), Field::Empty));
        actions.push(Action::new((nx, ny), Field::Entity(Rc::new(self.clone()))));
        actions
    }
}

impl Collidable for Diamond {
    fn check_collision(&self, other: &dyn Collidable, grid: &Grid) -> bool {
        self.get_future_position(grid) == other.get_position()
    }

    fn get_position(&self) -> (i32, i32) {
        self.position
    }

    fn get_future_position(&self, grid: &Grid) -> (i32, i32) {
        if let Some(direction) = self.is_falling(grid) {
            direction.edit_position(self.position)
        } else {
            self.position
        }
    }
}

impl Renderable for Diamond {
    fn render(&self) {
        println!("Diamond at {:?}", self.position); // Temporary implementation
    }
}

impl Entity for Diamond {
    fn get_type(&self) -> String {
        String::from("Diamond")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn update(&self, grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        let (px, py) = grid.get_player_position();
        let player_tile = grid.get_tile(px, py).unwrap();
        if let Some(Field::Entity(entity)) = player_tile.get_object_on() {
            let player = entity.as_any().downcast_ref::<Player>().unwrap();
            if self.check_collision(player, grid) {
                //TODO: Implement the explosion rendering
            }
        }
        actions.extend(self.fall(grid));
        actions
    }

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Diamond at {:?}", self.position)
    }
}

impl Fallable for Diamond { // Temporary implementation
    fn fall(&self, grid: &Grid) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut self_clone = self.clone();
        if let Some(movement) = self.is_falling(grid) {
            let (x, y) = movement.edit_position(self.position);
            self_clone.falling_since += 1;
            actions.extend(self_clone.move_to(self.position.0, self.position.1, x, y));
        } else {
            self_clone.falling_since = 0;
            actions.push(Action::new(self.position, Field::Entity(Rc::new(self_clone))));
        }
        actions
    }

    fn is_falling(&self, grid: &Grid) -> Option<Movement> {
        fn can_move_to(tile: Option<&Tile>, movement: Movement, falling_since: i32) -> Option<Movement> {
            match tile {
                Some(tile) => match tile.get_object_on() {
                    Some(Field::Entity(entity)) => {
                        if entity.get_type().as_str() == "Player" && falling_since > 0 && movement == Movement::MoveDown {
                            Some(movement)
                        } else {
                            None
                        }
                    },
                    Some(Field::Wall(_)) | Some(Field::Dirt) | Some(Field::Exit) => None,
                    Some(Field::Empty) | None => Some(movement),
                },
                None => None,
            }
        }

        if let Some(movement) = can_move_to(
            grid.get_nearest_tile(self.position.0, self.position.1, Movement::MoveDown),
            Movement::MoveDown,
            self.falling_since,
        ) {
            return Some(movement);
        }
    
        if let Some(movement) = can_move_to(
            grid.get_nearest_tile(self.position.0, self.position.1, Movement::MoveLeft),
            Movement::MoveLeft,
            self.falling_since,
        ) {
            return Some(movement);
        }
    
        if let Some(movement) = can_move_to(
            grid.get_nearest_tile(self.position.0, self.position.1, Movement::MoveRight),
            Movement::MoveRight,
            self.falling_since,
        ) {
            return Some(movement);
        }
    
        None
    }
    
}
