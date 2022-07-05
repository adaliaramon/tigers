use std::collections::HashMap;

use crate::{structs::column::Column, structs::row::Row};

#[derive(Debug)]
pub struct DataFrame {
    headers: Vec<String>,
    rows: Vec<Row>,
    columns: HashMap<String, Column>,
}

impl DataFrame {
    fn new(headers: Vec<String>, rows: Vec<Row>) -> DataFrame {
        let columns = DataFrame::get_columns(&headers, &rows);
        DataFrame {
            headers,
            rows,
            columns,
        }
    }

    fn get_columns(headers: &[String], rows: &[Row]) -> HashMap<String, Column> {
        let mut columns = HashMap::new();
        for (i, header) in headers.iter().enumerate() {
            let values: Vec<String> = rows.iter().map(|row| row[i].clone()).collect();
            columns.insert(header.clone(), Column::new(header.clone(), values));
        }
        columns
    }

    pub fn from_csv(path: &str) -> Result<DataFrame, std::io::Error> {
        let mut reader = csv::Reader::from_path(path)?;
        let mut rows: Vec<Row> = Vec::new();
        let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();
        for result in reader.records() {
            let values: Vec<String> = result?.iter().map(|v| v.to_string()).collect();
            rows.push(Row::new(headers.clone(), values));
        }
        let df: DataFrame = DataFrame::new(headers, rows);
        Ok(df)
    }

    pub fn to_csv(&self, path: &str) -> Result<(), std::io::Error> {
        let mut writer = csv::Writer::from_path(path)?;
        writer.write_record(&self.headers)?;
        for row in &self.rows {
            writer.write_record(row.get_values())?;
        }
        Ok(())
    }

    pub fn head(&self, n: usize) -> DataFrame {
        let headers = self.headers.clone();
        let rows = self.rows.iter().take(n).cloned().collect();
        DataFrame::new(headers, rows)
    }
}

impl std::ops::Index<usize> for DataFrame {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl std::ops::Index<&str> for DataFrame {
    type Output = Column;

    fn index(&self, index: &str) -> &Self::Output {
        &self.columns[index]
    }
}

impl std::fmt::Display for DataFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        let w = 4;

        // Compute the width of each column
        // The width of each column is the maximum width of the values in that column,
        // including the header, plus 4 spaces for separation.
        let widths: Vec<usize> = self
            .headers
            .iter()
            .map(|header| {
                let column = &self.columns[header];
                let max_width = column.iter().map(|value| value.len()).max().unwrap();
                std::cmp::max(max_width, header.len())
            })
            .collect();

        // Right-align the headers
        for (i, header) in self.headers.iter().enumerate() {
            s.push_str(&" ".repeat(widths[i] - header.len()));
            s.push_str(header);
            if i < self.headers.len() - 1 {
                s.push_str(&" ".repeat(w));
            }
        }
        s.push('\n');

        // Right-align the values
        for row in &self.rows {
            for (i, value) in row.get_values().iter().enumerate() {
                s.push_str(&" ".repeat(widths[i] - value.len()));
                s.push_str(value);
                if i < row.get_values().len() - 1 {
                    s.push_str(&" ".repeat(w));
                }
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}
