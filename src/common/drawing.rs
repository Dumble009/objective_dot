use std::cell::RefCell;
use std::rc::Rc;

use super::canvas_grid::{CanvasGrid, Grid};
use super::palette::{ObjectPalette, Palette};

pub trait Drawing {
    fn get_grid(&self) -> Result<Box<dyn Grid>, String>;
    fn get_grid_layer(&self, layer_index: usize) -> Option<Rc<RefCell<dyn Grid>>>;
    fn add_grid_layer(&mut self);
    fn get_palette(&self) -> Rc<RefCell<dyn Palette>>;
    fn get_grid_width(&self) -> usize;
    fn get_grid_height(&self) -> usize;
    fn set_grid_width(&mut self, w: usize) -> Result<(), String>;
    fn set_grid_height(&mut self, h: usize) -> Result<(), String>;
    fn get_layer_count(&self) -> usize;
    fn get_active_layer(&self) -> Rc<RefCell<dyn Grid>>;
    fn set_active_layer_idx(&mut self, layer_index: usize) -> Result<(), String>;
    fn move_layer_up(&mut self, layer_index: usize) -> Result<(), String>;
    fn move_layer_down(&mut self, layer_index: usize) -> Result<(), String>;
}

pub struct ObjectDrawing {
    grid_layers: Vec<Rc<RefCell<CanvasGrid>>>,
    palette: Rc<RefCell<ObjectPalette>>,
    width: usize,
    height: usize,
    active_layer_index: usize,
}

impl ObjectDrawing {
    pub fn new() -> Self {
        ObjectDrawing {
            grid_layers: vec![Rc::new(RefCell::new(CanvasGrid::new()))],
            palette: Rc::new(RefCell::new(ObjectPalette::new())),
            width: 10,
            height: 10,
            active_layer_index: 0,
        }
    }
}

impl Drawing for ObjectDrawing {
    fn get_grid(&self) -> Result<Box<dyn Grid>, String> {
        let mut result_grid = CanvasGrid::new();
        result_grid.set_grid_width(self.width)?;
        result_grid.set_grid_height(self.height)?;

        for x in 0..self.width {
            for y in 0..self.height {
                result_grid.set_color(x, y, 0)?;
            }
        }

        for layer in &self.grid_layers {
            let layer_grid = layer.borrow();
            for y in 0..self.height {
                for x in 0..self.width {
                    let color = layer_grid.get_color(x, y)?;
                    if color == 0 {
                        continue;
                    }
                    result_grid.set_color(x, y, color)?;
                }
            }
        }
        Ok(Box::new(result_grid))
    }

    fn get_grid_layer(&self, layer_index: usize) -> Option<Rc<RefCell<dyn Grid>>> {
        if layer_index < self.grid_layers.len() {
            Some(self.grid_layers[layer_index].clone())
        } else {
            None
        }
    }

    fn add_grid_layer(&mut self) {
        let mut grid = CanvasGrid::new();
        grid.set_grid_width(self.width).unwrap();
        grid.set_grid_height(self.height).unwrap();
        self.grid_layers.push(Rc::new(RefCell::new(grid)));
    }

    fn get_palette(&self) -> Rc<RefCell<dyn Palette>> {
        self.palette.clone()
    }

    fn get_grid_width(&self) -> usize {
        self.width
    }

    fn get_grid_height(&self) -> usize {
        self.height
    }

    fn set_grid_width(&mut self, w: usize) -> Result<(), String> {
        for layer in &self.grid_layers {
            layer.borrow_mut().set_grid_width(w)?;
        }
        self.width = w;
        Ok(())
    }

    fn set_grid_height(&mut self, h: usize) -> Result<(), String> {
        for layer in &self.grid_layers {
            layer.borrow_mut().set_grid_height(h)?;
        }
        self.height = h;
        Ok(())
    }

    fn get_layer_count(&self) -> usize {
        self.grid_layers.len()
    }

    fn get_active_layer(&self) -> Rc<RefCell<dyn Grid>> {
        self.grid_layers[self.active_layer_index].clone()
    }

    fn set_active_layer_idx(&mut self, layer_index: usize) -> Result<(), String> {
        if layer_index < self.grid_layers.len() {
            self.active_layer_index = layer_index;
            Ok(())
        } else {
            Err(format!(
                "Layer index {} is out of bounds. Total layers: {}",
                layer_index,
                self.grid_layers.len()
            ))
        }
    }

    fn move_layer_up(&mut self, layer_index: usize) -> Result<(), String> {
        if layer_index + 1 >= self.grid_layers.len() {
            return Err(format!(
                "Cannot move layer {} up. It is either the bottom layer or out of bounds.",
                layer_index
            ));
        }
        self.grid_layers.swap(layer_index, layer_index + 1);
        Ok(())
    }

    fn move_layer_down(&mut self, layer_index: usize) -> Result<(), String> {
        if layer_index == 0 || layer_index >= self.grid_layers.len() {
            return Err(format!(
                "Cannot move layer {} down. It is either the top layer or out of bounds.",
                layer_index
            ));
        }
        self.grid_layers.swap(layer_index, layer_index - 1);
        Ok(())
    }
}

include!("tests/drawing_test.rs");
