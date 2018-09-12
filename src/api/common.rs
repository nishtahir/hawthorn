#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub limit: i32,
    pub offset: i32,
    pub data: Vec<T>,
}
