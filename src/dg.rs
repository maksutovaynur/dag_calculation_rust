use super::parsing::{CellKind, parse};

use std::collections::HashMap;


#[derive(Debug)]
pub struct Cell {
    pub kind: CellKind,
    refs: Vec<usize>,
    outer_refs: Vec<usize>,
    number: usize
}

impl Cell {
    pub fn new (number: &usize) -> Cell {
        Cell {
            kind: CellKind::VALUE(0),
            outer_refs: Vec::new(),
            refs: Vec::new(),
            number: number.clone()
        }
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
        let mut refs: Vec<usize> = Vec::new();
        for x in input.refs.into_iter() {
            refs.push(self.get_or_create_cell(x).number.clone());
        }
        let cell = self.get_or_create_cell(input.name);
        cell.refs = refs.clone();
        cell.kind = input.kind;
        let number = cell.number.clone();
        for c in self.cells.iter_mut() {
            c.outer_refs.push(number.clone());
        }
    }

    pub fn size(&self) -> usize {
        return self.cells.len();
    }

    fn get_or_create_cell(&mut self, name: String) -> &mut Cell {
        let len: usize = self.cells.len();
        let mut number: usize = self.cells_map.entry(name).or_insert(len).clone();
        if number >= len {
            let cell = Cell::new(& number);
            self.cells.push(cell);
            number = len;
        }
        self.cells.get_mut(number).expect("strange error - no cell with such number")
    }
}

