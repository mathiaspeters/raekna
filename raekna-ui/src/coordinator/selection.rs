use std::cmp::Ordering;

use crate::graphics::controls::caret_position::CaretPosition;

#[derive(Copy, Clone, Debug)]
pub enum Selection {
    None(CaretPosition),
    Some {
        caret_position: CaretPosition,
        root_position: CaretPosition,
    },
}

impl Selection {
    pub fn new(position: CaretPosition) -> Self {
        Self::None(position)
    }

    pub fn caret_position(&self) -> CaretPosition {
        match self {
            Selection::None(caret_position) | Selection::Some { caret_position, .. } => {
                *caret_position
            }
        }
    }

    pub fn root_position(&self) -> Option<CaretPosition> {
        match self {
            Selection::None(_) => None,
            Selection::Some { root_position, .. } => Some(*root_position),
        }
    }

    pub fn as_ordered(&self) -> Option<(CaretPosition, CaretPosition)> {
        self.root_position().map(|root_position| {
            let caret_position = self.caret_position();
            match caret_position.line.cmp(&root_position.line) {
                Ordering::Equal if caret_position.column <= root_position.column => {
                    (caret_position, root_position)
                }
                Ordering::Equal => (root_position, caret_position),
                Ordering::Less => (caret_position, root_position),
                Ordering::Greater => (root_position, caret_position),
            }
        })
    }

    pub fn set_position(&mut self, position: CaretPosition) {
        let mut cp = position;
        cp.set_position(position.line, position.column); // To make sure column and actual_column have the same value
        *self = Self::None(cp)
    }

    pub fn set_root(&mut self, root_position: Option<CaretPosition>) {
        match root_position {
            Some(root) => match self {
                Selection::None(caret_position) => {
                    *self = Self::Some {
                        caret_position: *caret_position,
                        root_position: root,
                    }
                }
                Selection::Some { root_position, .. } => {
                    *root_position = root;
                }
            },
            None => match self {
                Selection::None(_) => {}
                Selection::Some { caret_position, .. } => {
                    *self = Self::None(*caret_position);
                }
            },
        }
    }

    pub fn set_selection(&mut self, root_position: CaretPosition, caret_position: CaretPosition) {
        *self = Self::Some {
            caret_position,
            root_position,
        }
    }

    pub fn update_position<F>(&mut self, mut op: F)
    where
        F: FnMut(&mut CaretPosition),
    {
        match self {
            Selection::None(caret_position) | Selection::Some { caret_position, .. } => {
                op(caret_position)
            }
        }
    }
}

impl Default for Selection {
    fn default() -> Self {
        Self::None(Default::default())
    }
}
