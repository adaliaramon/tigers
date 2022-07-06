use crate::{left_pad, max_length};

#[derive(Debug)]
pub struct Column {
    header: String,
    values: Vec<String>,
}

impl Column {
    pub(crate) fn new(header: String, values: Vec<String>) -> Column {
        Column { header, values }
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<String> {
        self.values.iter()
    }
}

impl std::fmt::Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let index_width = (self.values.len() - 1).to_string().len();
        let value_width = max_length!(self.values).max(self.header.len());
        let w = 4;

        write!(
            f,
            "{}",
            left_pad!(&self.header, index_width + value_width + w)
        )?;
        writeln!(f)?;

        for (i, value) in self.values.iter().enumerate() {
            write!(f, "{}", left_pad!(&i.to_string(), index_width))?;
            write!(f, "{}", " ".repeat(w))?;
            write!(f, "{}", left_pad!(value, value_width))?;
            if i < self.values.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
