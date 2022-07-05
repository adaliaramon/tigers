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
