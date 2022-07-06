use crate::{left_pad, max_length};

#[derive(Debug, Clone)]
pub struct Row {
    headers: Vec<String>,
    values: Vec<String>,
}

impl Row {
    pub(crate) fn new(headers: Vec<String>, values: Vec<String>) -> Row {
        Row { headers, values }
    }

    pub(crate) fn get_values(&self) -> &Vec<String> {
        &self.values
    }

    pub(crate) fn rename(&self, new_headers: &[String]) -> Row {
        Row {
            headers: new_headers.to_owned(),
            values: self.values.clone(),
        }
    }
}

impl std::ops::Index<usize> for Row {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let header_width = max_length!(self.headers);
        let value_width = max_length!(self.values);
        let w = 4;

        for (i, header) in self.headers.iter().enumerate() {
            write!(f, "{}", left_pad!(header, header_width))?;
            write!(f, "{}", " ".repeat(w))?;
            write!(f, "{}", left_pad!(&self.values[i], value_width))?;
            if i < self.headers.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
