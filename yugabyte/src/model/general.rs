use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Apiv2Schema, Debug)]
pub struct PaginationDTO {
    pub page_size: i64,
    pub offset: i64,
}

#[derive(Default, Serialize, Apiv2Schema, Debug)]
pub struct PaginatedResponseDTO<T> {
    pub paginated_list: Vec<T>,
    pub count: i64,
}