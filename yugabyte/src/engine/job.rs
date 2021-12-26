use diesel::{ExpressionMethods, PgSortExpressionMethods};
use diesel::{associations::HasTable, RunQueryDsl};
use diesel::{PgConnection, QueryResult};
use diesel::QueryDsl;
use uuid::Uuid;

use crate::{errors::Error, model::job::NewJob};
use crate::model::general::PaginationDTO;
use crate::model::job::{Job, JobInfo};
use crate::schema::job::dsl::*;
use crate::schema::job::dsl::id as job_primary_id;

impl NewJob {
    pub fn add_job(&self, connection: &PgConnection) -> Result<Job, Error> {
        // add the new job to the db.
        let new_job = Job {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            total_size: self.total_size,
            downloaded_size: 0,
            percent_downloaded: 0,
            status: "Active".to_string(),
            is_active: self.is_active,
            creation_date: chrono::offset::Utc::now().naive_local(),
            expiration_date: Option::None,
        };

        diesel::insert_into(job::table())
            .values(&new_job)
            .get_result::<Job>(connection)
            .map_err(|_err| Error::DuplicationError)
    }
}

pub fn create_bulk_jobs(
    other_jobs: &Vec<Job>,
    connection: &PgConnection,
) -> Result<Vec<Job>, Error> {
    diesel::insert_into(job::table())
        .values(other_jobs)
        .get_results::<Job>(connection)
        .map_err(|_| Error::DuplicationError)
}

pub fn get_all_jobs(connection: &PgConnection) -> QueryResult<Vec<Job>> {
    job::table().load::<Job>(connection)
}

pub fn get_all_paginated_jobs(
    pagination_dto: &PaginationDTO,
    connection: &PgConnection,
) -> Result<Vec<Job>, Error> {
    job::table()
        .order_by(creation_date.desc().nulls_last())
        .limit(pagination_dto.page_size)
        .offset(pagination_dto.offset)
        .load::<Job>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn count_jobs(connection: &PgConnection) -> Result<i64, Error> {
    job::table()
        .count()
        .get_result(connection)
        .map_err(|e| Error::DBError(e))
}

pub fn delete_job_by_id(
    other_job_id: &Uuid,
    connection: &PgConnection,
) -> Result<Job, Error> {
    // Step 1: delete the required job from the db.
    let deleted_job =
        diesel::delete(job::table().filter(job_primary_id.eq(other_job_id)))
            .load::<Job>(connection)
            .map_err(|e| Error::DBError(e))?
            .pop()
            .ok_or(Error::DBError(diesel::result::Error::NotFound))?;

    Ok(deleted_job)
}

// Activate/Deactivate the job.
pub fn set_activate_job(
    other_job_id: &Uuid,
    activate: bool,
    connection: &PgConnection,
) -> Result<usize, Error> {
    let res = diesel::update(job::table().filter(job_primary_id.eq(other_job_id)))
        .set((
            is_active.eq(activate),
        ))
        .execute(connection)
        .map_err(|e| Error::DBError(e));
    res
}

pub fn delete_all_jobs(connection: &PgConnection) -> Result<usize, Error> {
    diesel::delete(job::table())
        .execute(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn find_job_by_id(other_job_id: &Uuid, connection: &PgConnection) -> Result<Job, Error> {
    job::table()
        .find(other_job_id)
        .get_result::<Job>(connection)
        .map_err(|err| Error::DBError(err))
}

pub fn update_job(
    incoming_job: &Job,
    connection: &PgConnection,
) -> Result<Job, Error> {
    diesel::update(job.find(&incoming_job.id))
        .set((
            name.eq(&incoming_job.name),
            total_size.eq(&incoming_job.total_size),
            is_active.eq(&incoming_job.is_active),
        ))
        .get_result::<Job>(connection)
        .map_err(|e| Error::DBError(e))
}

pub fn get_job_info(other_job_id: &Uuid, connection: &PgConnection) -> Result<JobInfo, Error> {
    match find_job_by_id(&other_job_id, connection) {
        Ok(found_job) => Ok(JobInfo {
            name: found_job.name,
            downloaded_size: found_job.downloaded_size,
            remaining_size: found_job.total_size - found_job.downloaded_size,
        }),
        Err(e) => Err(e),
    }
}