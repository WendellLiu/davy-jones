#[derive(Serialize, Deserialize)]
pub struct CustomResponse<T> {
  pub status: String,
  pub data: T
}