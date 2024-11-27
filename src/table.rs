pub mod table {
    use std::collections::BTreeMap;

    use crate::cell::{Cell, CellData};
    use crate::header::Header;
    use crate::row::{self, Row};
    use crate::{constraint, header};

    impl Clone for Table {
        fn clone(&self) -> Self {
            return Table {
                name: self.name.clone(),
                headers: self.headers.clone(),
                body: self.body.clone(),
            };
        }
    }

    pub struct Table {
        name: String,
        headers: Vec<header::Header>,
        body: BTreeMap<String, row::Row>,
    }

    impl Default for Table {
        fn default() -> Self {
            Table {
                name: String::from(""),
                headers: vec![],
                body: BTreeMap::<String, Row>::new(),
            }
        }
    }

    impl ToString for Table {
        fn to_string(&self) -> String {
            let mut out: String = String::from("");
            let mut widths: Vec<usize> = vec![];
            for i in 0..self.headers.len() {
                let header: &header::Header = &self.headers[i];
                let datatype = constraint::constraint_to_string(&header.constraint.datatype);
                let mut final_width: usize = header.name.len() + datatype.len() + 3;
                if !header
                    .constraint
                    .contains(constraint::Constraints::NOT_NULL)
                {
                    final_width += 1;
                }
                for j in &self.body {
                    let check_cell_size: usize;
                    match &j.1.cells[i].data {
                        CellData::Null => check_cell_size = "null".len(),
                        CellData::Text(text) => check_cell_size = text.len(),
                    }
                    if check_cell_size > final_width {
                        final_width = check_cell_size;
                    }
                }

                widths.push(final_width);
            }

            let mut line_length = 1 + 12;

            let mut line: String = String::from("");

            let immut_widths: Vec<usize> = widths.clone();

            for width in immut_widths {
                line_length += width + 3
            }

            for _ in 0..line_length {
                line.push('-');
            }

            out.push_str(line.as_str());
            out.push_str("\n");

            out.push_str("| ");

            for _ in 0..(line_length - &self.name.len() - 3) / 2 {
                out.push_str(" ");
            }
            out.push_str("\x1b[4m");
            out.push_str("\x1b[1m");
            out.push_str(&self.name);
            out.push_str("\x1b[0m");
            for _ in (line_length - &self.name.len() - 3) / 2..(line_length - &self.name.len() - 3)
            {
                out.push_str(" ");
            }

            out.push_str("|\n");

            out.push_str(line.as_str());
            out.push_str("\n");

            let mut header_data = String::from("|");

            header_data.push_str("    Hash   |");
            for i in 0..self.headers.len() {
                header_data.push_str(" ");

                let header: &Header = &self.headers[i];
                let length = header.name.len();
                let datatype = constraint::constraint_to_string(&header.constraint.datatype);
                let widths_copy = widths.clone();
                let max_width = widths_copy[i];
                let mut offset = 0;
                if header
                    .constraint
                    .contains(constraint::Constraints::PRIMARY_KEY)
                {
                    header_data.push_str("\x1b[4m");
                }
                header_data.push_str(&header.name);
                if !header
                    .constraint
                    .contains(constraint::Constraints::NOT_NULL)
                {
                    header_data.push_str("?");
                    offset += 1;
                }
                if header
                    .constraint
                    .contains(constraint::Constraints::PRIMARY_KEY)
                {
                    header_data.push_str("\x1b[0m");
                }
                header_data.push_str(" (");
                header_data.push_str("\x1b[1m");
                header_data.push_str(&datatype);
                header_data.push_str("\x1b[0m");
                header_data.push_str(")");

                for _ in 0..(max_width - length - datatype.len() - offset - 3) {
                    header_data.push_str(" ");
                }
                // out.push_str(&header_data);
                header_data.push_str(" |");
            }

            out.push_str(&header_data.as_str());
            out.push_str("\n");
            out.push_str(&line.as_str());
            out.push_str("\n");

            let mut body_data = String::from("");

            for row in &self.body {
                body_data.push_str("| ");
                body_data.push_str(&row.1.clone().get_hash(self.headers.clone()).split_at(6).0);
                body_data.push_str("... ");

                body_data.push_str("|");
                for i in 0..row.1.cells.len() {
                    body_data.push_str(" ");
                    let cell: &Cell = &row.1.cells[i];
                    let length;
                    match &cell.data {
                        CellData::Null => length = "null".len(),
                        CellData::Text(text) => length = text.len(),
                    }
                    let widths_copy = widths.clone();
                    let max_width = widths_copy[i];
                    match &cell.data {
                        CellData::Null => {
                            body_data.push_str("\x1b[3m\x1b[38;5;242m");
                            body_data.push_str("null");
                            body_data.push_str("\x1b[0m");
                        }
                        CellData::Text(text) => body_data.push_str(&text),
                    }

                    for _ in 0..(max_width - length) {
                        body_data.push_str(" ");
                    }
                    // out.push_str(&header_data);
                    body_data.push_str(" |");
                }
                body_data.push_str("\n");
            }

            out.push_str(&body_data);

            out.push_str(&line.as_str());
            out.push_str("\n");

            return out.to_string();
        }
    }

    impl Table {
        pub fn new(
            name: String,
            headers: Vec<header::Header>,
            rows: Vec<row::Row>,
            order_by: Option<String>,
        ) -> Self {
            let mut sort_by_header: Option<Header> = None;
            if let Some(order) = order_by {
                for header in &headers {
                    if header.name == order {
                        sort_by_header = Some(header.clone());
                        break;
                    }
                }

                // After the loop, check if the column was found
                if sort_by_header.is_none() {
                    panic!("Column, {:?}, not found in headers", order);
                }
            }

            let mut out_body: BTreeMap<String, row::Row> = BTreeMap::new();

            for row in rows {
                let hash = row.clone().get_hash(headers.clone());
                out_body.insert(hash, row);
            }

            Table {
                name: name,
                headers: headers,
                body: out_body,
            }
        }

        pub fn get_headers(&self) -> &Vec<Header> {
            return &self.headers;
        }

        pub fn get_rows(&self) -> &BTreeMap<String, Row> {
            return &self.body;
        }

        pub fn get_name(&self) -> &String {
            return &self.name;
        }
    }
}
