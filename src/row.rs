use chrono::Utc;

use crate::cell::CellData;
use crate::{cell, constraint, header};
use sha2::{Digest, Sha256};
use std::any::Any;

pub struct Row {
    pub cells: Vec<cell::Cell>,
}

impl Default for Row {
    fn default() -> Self {
        Row { cells: vec![] }
    }
}

impl Clone for Row {
    fn clone(&self) -> Self {
        return Row {
            cells: self.cells.clone(),
        };
    }
}

impl Row {
    pub fn new(cells: Vec<cell::Cell>) -> Self {
        Row { cells: cells }
    }

    pub fn get_rows(self: Row) -> Vec<cell::Cell> {
        return self.cells;
    }

    pub fn set_rows(mut self, cells: Vec<cell::Cell>) {
        self.cells = cells;
    }

    pub fn get_hash(self, headers: Vec<header::Header>) -> String {
        let mut has_primary_key: bool = false;
        for header in &headers {
            for constraint in &header.constraint.constraints {
                if constraint.type_id() == constraint::Constraints::PrimaryKey.type_id() {
                    has_primary_key = true;
                }
            }
        }

        let out_hash;

        if has_primary_key {
            let mut total_hash = Sha256::new();
            for i in 0..headers.len() {
                if headers[i]
                    .constraint
                    .contains(constraint::Constraints::PrimaryKey)
                {
                    let mut hasher = Sha256::new();
                    match &self.cells[i].data {
                        CellData::Null => hasher.update("null"),
                        CellData::Text(text) => hasher.update(text),
                    }
                    total_hash.update(hasher.finalize());
                }
            }

            out_hash = format!("{:x}", total_hash.finalize());
        } else {
            let mut hasher = Sha256::new();
            hasher.update(Utc::now().timestamp_micros().to_be_bytes());
            out_hash = format!("{:x}", hasher.finalize());
        }

        return out_hash;
    }
}
