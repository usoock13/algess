#[derive(Clone)]
pub struct DataTable {
    pub label: String,
    pub data: Vec<Data2D>
}

#[derive(Clone)]
pub struct Data2D {
    pub x: f64,
    pub y: f64
}