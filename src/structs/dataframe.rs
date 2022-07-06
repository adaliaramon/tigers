use std::collections::HashMap;

use crate::{left_pad, max_length, structs::column::Column, structs::row::Row};

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
        if headers.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No headers found in CSV file",
            ));
        }
        for result in reader.records() {
            let values: Vec<String> = result?.iter().map(|v| v.to_string()).collect();
            if values.len() != headers.len() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "Number of values in row {} does not match number of headers",
                        rows.len() + 1
                    ),
                ));
            }
            rows.push(Row::new(headers.clone(), values));
        }
        if rows.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No rows found in CSV file",
            ));
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

    pub fn rename(&self, map: &HashMap<String, String>) -> DataFrame {
        let headers = self
            .headers
            .iter()
            .map(|h| map.get(h).unwrap_or(h).clone())
            .collect::<Vec<String>>();
        let rows = self.rows.iter().map(|r| r.rename(&headers)).collect();
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
        let w = 4;

        // Compute the width of each column
        let widths: Vec<usize> = self
            .headers
            .iter()
            .map(|header| max_length!(&self.columns[header]).max(header.len()))
            .collect();

        // Right-align the headers
        for (i, header) in self.headers.iter().enumerate() {
            write!(f, "{}", left_pad!(header, widths[i]))?;
            if i < self.headers.len() - 1 {
                write!(f, "{}", " ".repeat(w))?;
            }
        }
        writeln!(f)?;

        // Right-align the values
        for (i, row) in self.rows.iter().enumerate() {
            for (i, value) in row.get_values().iter().enumerate() {
                write!(f, "{}", left_pad!(value, widths[i]))?;
                if i < row.get_values().len() - 1 {
                    write!(f, "{}", " ".repeat(w))?;
                }
            }
            if i < self.rows.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
