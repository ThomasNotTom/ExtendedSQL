#[cfg(test)]
mod table_tests {
    const TABLE_NAME: &str = "test_table";

    use extended_sql::{
        cell::{self, Cell},
        constraint::{self, Constraint, ConstraintDatatypes},
        header::{self, Header},
        row::{self, Row},
        table::{self, Table},
    };

    #[test]
    fn empty_initialisation() {
        Table::new(TABLE_NAME.to_string(), vec![], vec![], None);
    }

    #[test]
    #[should_panic]
    fn order_by_invalid_column() {
        const COLUMN_NAME: &str = "invalid_column";

        Table::new(
            TABLE_NAME.to_string(),
            vec![],
            vec![],
            Some(COLUMN_NAME.to_string()),
        );
    }

    #[test]
    fn get_header() {
        const COLUMN_NAME: &str = "column_1";
        let header: Header = header::Header::new(
            COLUMN_NAME.to_string(),
            Constraint {
                datatype: ConstraintDatatypes::INT,
                constraints: vec![],
            },
        );

        let table = table::Table::new(TABLE_NAME.to_string(), vec![header], vec![], None);

        assert_eq!(table.get_headers().len(), 1);
    }

    #[test]
    fn get_rows() {
        const COLUMN_NAME: &str = "column";
        let header: Header = header::Header::new(
            COLUMN_NAME.to_string(),
            constraint::Constraint {
                datatype: constraint::ConstraintDatatypes::STRING,
                constraints: vec![],
            },
        );

        let cell: Cell = cell::Cell::new(cell::CellData::Text("1".to_string()));
        let row: Row = row::Row::new(vec![cell]);
        let table = table::Table::new(TABLE_NAME.to_string(), vec![header], vec![row], None);

        println!("{}", table.to_string());

        assert_eq!(table.get_rows().len(), 1);
    }
}
