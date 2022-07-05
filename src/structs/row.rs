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
