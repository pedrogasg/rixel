use bevy::prelude::{Component, EventReader, EventWriter, Input, KeyCode, Query, Res, Resource};
use ndarray::{prelude::*, Slice};

use crate::{cell::CellPosition, Agent};

pub enum Direction {
    TOP,
    LEFT,
    BOTTOM,
    RIGHT,
}

pub struct Movement {
    direction: Direction,
}

impl Movement {
    pub fn new(direction: Direction) -> Self {
        Self { direction }
    }
}
pub struct Shifts {
    pub top: u8,
    pub left: u8,
    pub bottom: u8,
    pub right: u8,
}

impl Shifts {
    pub fn new(top: u8, left: u8, bottom: u8, right: u8) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }
    const TOP: (usize, usize) = (1, 0);
    const LEFT: (usize, usize) = (0, 1);
    const BOTTOM: (usize, usize) = (1, 2);
    const RIGTH: (usize, usize) = (2, 1);
}

impl From<Array2<u8>> for Shifts {
    fn from(slice: Array2<u8>) -> Self {
        //assert!(slice.dim() == (3, 3));

        Shifts::new(
            slice[Shifts::TOP],
            slice[Shifts::LEFT],
            slice[Shifts::BOTTOM],
            slice[Shifts::RIGTH],
        )
    }
}
#[derive(Resource, Component, Default, Clone, Debug, Hash)]
pub struct Actions {
    pub action_grid: Array<u8, Dim<[usize; 2]>>,
}

impl Actions {
    pub fn new(grid: Array<u8, Dim<[usize; 2]>>) -> Self {
        let (x, y) = grid.dim();
        let mut action_grid = Array2::<u8>::zeros((x + 2, y + 2));
        let mut original_grid = action_grid.view_mut();
        original_grid.slice_axis_inplace(Axis(0), Slice::from(1..x + 1));
        original_grid.slice_axis_inplace(Axis(1), Slice::from(1..y + 1));
        original_grid.assign(&grid);
        Self { action_grid }
    }

    pub fn empty(height: u32, width: u32) -> Self {
        let base = Array2::<u8>::ones(Dim([height as usize, width as usize]));
        Actions::new(base)
    }

    pub fn get_shifts(&self, x: u8, y: u8) -> Shifts {
        let xs = x as usize;
        let ys = y as usize;
        let xe = (xs + 3) as usize;
        let ye = (ys + 3) as usize;
        let action = self.action_grid.slice(s![xs..xe, ys..ye]);
        Shifts::from(action.to_owned())
    }
}

pub fn keyboard_movement(
    mut movement_event: EventWriter<Movement>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        movement_event.send(Movement::new(Direction::LEFT));
    } else if keyboard_input.just_pressed(KeyCode::D) {
        movement_event.send(Movement::new(Direction::RIGHT));
    } else if keyboard_input.just_pressed(KeyCode::Z) {
        movement_event.send(Movement::new(Direction::TOP));
    } else if keyboard_input.just_pressed(KeyCode::S) {
        movement_event.send(Movement::new(Direction::BOTTOM));
    }
}

pub fn movement(
    actions: Res<Actions>,
    mut movement_event: EventReader<Movement>,
    mut agent_query: Query<(&Agent, &mut CellPosition)>,
) {
    for dir in movement_event.iter() {
        for (_agent, mut position) in agent_query.iter_mut() {
            let shifts = actions.get_shifts(position.x as u8, position.y as u8);
            match dir.direction {
                Direction::TOP => {
                    position.y -= shifts.top as u32;
                }
                Direction::LEFT => {
                    position.x -= shifts.left as u32;
                }
                Direction::BOTTOM => {
                    position.y += shifts.bottom as u32;
                }
                Direction::RIGHT => {
                    position.x += shifts.right as u32;
                }
            }
        }
    }
}
