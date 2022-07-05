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
        let mut s = String::new();
        let index_width = (self.values.len() - 1).to_string().len();
        let value_width = self
            .values
            .iter()
            .map(|value| value.len())
            .max()
            .unwrap()
            .max(self.header.len());
        let w = 4;

        s.push_str(&" ".repeat(index_width + value_width + w - self.header.len()));
        s.push_str(&self.header);
        s.push('\n');

        for (i, value) in self.values.iter().enumerate() {
            s.push_str(&" ".repeat(index_width - i.to_string().len()));
            s.push_str(&i.to_string());
            s.push_str(&" ".repeat(w));
            s.push_str(&" ".repeat(value_width - value.len()));
            s.push_str(value);
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}
