use std::mem::swap;

use super::{TableContent, TableCell, column::{Column, ColumnID}};

pub struct RowIterator<'a> {
    table: &'a mut TableContent,
}

impl TableContent {
    pub fn take_rows(&mut self) -> RowIterator {
        RowIterator { table: self }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        let columns = self.table.columns.len();
        if columns > self.table.cells.len() {
            return None;
        }
        let mut tail = self.table.cells.split_off(columns);
        swap(&mut self.table.cells, &mut tail);
        let a = Row {
            cells: tail,
        };
        Some(a)
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    pub cells: Vec<TableCell>,
}

impl Row {
    pub fn create_empty(l: usize) -> Self {
        Self {
            cells: vec![TableCell { content: None }; l]
        }
    }

    pub fn get_cell(&self, columns: &Vec<Column>, id: ColumnID) -> Option<&TableCell> {
        for (i, column) in columns.iter().enumerate() {
            if column.id == id {
                return Some(&self.cells[i])
            }
        }
        None
    }
}