use self::column::{Column, ColumnID};

pub mod column;
pub mod row;

#[derive(Debug, Default)]
pub struct TableContent {
    pub columns: Vec<Column>,
    pub cells: Vec<TableCell>,
}

impl TableContent {
    pub fn column(&self, id: ColumnID) -> Option<&Column> {
        for column in &self.columns {
            if column.id == id {
                return Some(column)
            }
        }
        None
    }
    pub fn cell(&self, column_id: ColumnID, row: usize) -> Option<&TableCell> {
        for (i, column) in self.columns.iter().enumerate() {
            if column.id == column_id {
                return Some(&self.cells[row * self.columns.len() + i])
            }
        }
        None
    }
    pub fn width(&self) -> usize {
        self.columns.len()
    }
    pub fn height(&self) -> usize {
        self.cells.len() / self.columns.len()
    }
    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }
    pub fn insert_row(&mut self, before: usize) {
        let columns = self.columns.len();
        let i = before * columns;
        for _ in 0..columns {
            self.cells.insert(i, TableCell { content: None });
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableCell {
    pub content: Option<String>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellRef {
    pub column: ColumnID,
    pub row: usize,
}