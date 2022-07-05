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
}

impl std::ops::Index<usize> for Row {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        let header_width = self
            .headers
            .iter()
            .map(|header| header.len())
            .max()
            .unwrap();
        let value_width = self.values.iter().map(|value| value.len()).max().unwrap();
        let w = 4;

        for (i, header) in self.headers.iter().enumerate() {
            s.push_str(&" ".repeat(header_width - header.len()));
            s.push_str(header);
            s.push_str(&" ".repeat(w));
            s.push_str(&" ".repeat(value_width - self.values[i].len()));
            s.push_str(&self.values[i]);
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}
