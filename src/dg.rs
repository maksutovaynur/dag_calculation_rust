use super::parsing::{CellKind, parse};

use std::collections::HashMap;


#[derive(Debug)]
pub struct Cell {
    pub kind: CellKind,
    refs: Vec<usize>,
    outer_refs: Vec<usize>,
}

impl Cell {
    pub fn new () -> Cell {
        Cell { kind: CellKind::VALUE(0), outer_refs: Vec::new(), refs: Vec::new() }
    }
}



pub struct Graph {
   cells_map: HashMap<String, usize>,
   cells: Vec<Cell>
}


impl Graph {
    pub fn new() -> Graph {
        return Graph {
            cells_map: HashMap::new(),
            cells: Vec::new()
        }
    }

    pub fn add_cell(&mut self, line: &str) {
        let input_result = parse(line);
        let input = match input_result {
            Some(v) => v,
            None => return
        };
        println!("Wtf? {:?}", input);
        let cell = self.get_or_create_cell(input.name);

    }

    fn get_or_create_cell(&mut self, name: String) -> &mut Cell {
        let len: usize = self.cells.len();
        let mut number: usize = self.cells_map.entry(name).or_insert(len).clone();
        if number >= len {
            let cell = Cell::new();
            self.cells.push(cell);
            number = len;
        }
        self.cells.get_mut(number).expect("strange error - no cell with such number")
    }

    pub fn size(&self) -> usize {
        return self.cells.len();
    }
}
