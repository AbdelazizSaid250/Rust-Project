use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::job;
use crate::util::utils::REGEX_FULL_WORD;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Validate, Apiv2Schema, Clone)]
#[table_name = "job"]
pub struct Job {
    pub id: Uuid,
    #[validate(length(min = 3, max = 49, code = "name-length-error"))]
    #[validate(regex = "REGEX_FULL_WORD")]
    pub name: String,
    pub total_size: i32,
    pub downloaded_size: i32,
    pub percent_downloaded: i32,
    pub status: String,
    pub is_active: bool,
    pub creation_date: NaiveDateTime,
    pub expiration_date: Option<NaiveDateTime>,
}

#[derive(Default, Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct NewJob {
    pub name: String,
    pub total_size: i32,
    pub is_active: bool,
}

#[derive(Default, Debug, Serialize, Deserialize, Apiv2Schema)]
pub struct JobInfo {
    pub name: String,
    pub downloaded_size: i32,
    pub remaining_size: i32,
}

