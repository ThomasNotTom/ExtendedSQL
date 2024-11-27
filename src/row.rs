use chrono::Utc;

use crate::cell::CellData;
use crate::{cell, constraint, header};
use sha2::{Digest, Sha256};
use std::any::Any;
use std::hash::{Hash, Hasher};

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

    pub fn get_hash(mut self, headers: Vec<header::Header>) -> String {
        let mut has_primary_key: bool = false;
        for header in &headers {
            for constraint in &header.constraint.constraints {
                if constraint.type_id() == constraint::Constraints::PRIMARY_KEY.type_id() {
                    has_primary_key = true;
                }
            }
        }

        let mut out_hash: String = String::from("");

        if (has_primary_key) {
            let mut total: u64 = 0;
            for i in 0..headers.len() {
                if headers[i]
                    .constraint
                    .contains(constraint::Constraints::PRIMARY_KEY)
                {
                    let mut hasher = fnv::FnvHasher::default();
                    if (self.cells[i].data == CellData::Null) {
                        "null".hash(&mut hasher);
                    } else {
                        self.cells[i].data.hash(&mut hasher);
                    }
                    total += hasher.finish()
                }
            }

            let mut hasher = Sha256::new();
            hasher.update(total.to_be_bytes());
            out_hash = format!("{:x}", hasher.finalize());
        } else {
            let mut hasher = Sha256::new();
            hasher.update(Utc::now().timestamp_micros().to_be_bytes());
            out_hash = format!("{:x}", hasher.finalize());
        }

        return out_hash;
    }
}
