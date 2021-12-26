use paperclip::actix::{
    api_v2_operation,
    web::{self, Query},
};
use paperclip::actix::web::Json;
use uuid::Uuid;

use yugabyte::db_connection::{CoreDBPool, pgdata_to_pgconnection};
use yugabyte::engine::job::{count_jobs, delete_job_by_id, find_job_by_id, get_all_paginated_jobs, set_activate_job, update_job, get_job_info};
use yugabyte::errors::{Error, Errors};
use yugabyte::errors::StateCode::{
    DBError, DuplicationError, InternalServerError, NotFound, PaginationError,
};
use yugabyte::model::general::{PaginatedResponseDTO, PaginationDTO};
use yugabyte::model::job::{Job, NewJob, JobInfo};

#[api_v2_operation]
pub(crate) fn add_job(
    new_job: web::Json<NewJob>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<Job>, Errors> {
    // Step 1: get the connection from pool data
    let connection = pgdata_to_pgconnection(pool);

    // Step 2: add the new job.
    match new_job.add_job(&connection) {
        // Step 3: fire the response
        Ok(job) => Ok(Json(job)),
        Err(e) => {
            match e {
                Error::DuplicationError => Err(Errors::InternalServerError(DuplicationError.into())),
                _ => Err(Errors::InternalServerError(InternalServerError.into())),
            }
        }
    }
}

#[api_v2_operation]
pub(crate) fn list_paginated_jobs(
    Query(pagination_dto): Query<PaginationDTO>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<PaginatedResponseDTO<Job>>, Errors> {
    // Step 1: get the connection from pool data
    let connection = pgdata_to_pgconnection(pool);

    // Step 2: count all jobs in the db.
    match count_jobs(&connection) {
        Ok(jobs_count) => {
            // Step 3: get the job list.
            match get_all_paginated_jobs(&pagination_dto, &connection) {
                Ok(paginated_list) => {
                    let response = PaginatedResponseDTO {
                        paginated_list,
                        count: jobs_count,
                    };
                    // Step 4: fire the response
                    Ok(Json(response))
                }
                Err(_) => {
                    Err(Errors::BadRequest(PaginationError.into()))
                }
            }
        }
        Err(_) => {
            Err(Errors::InternalServerError(DBError.into()))
        }
    }
}

#[api_v2_operation]
pub(crate) fn remove_job_by_id(
    job_id: web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<Job>, Errors> {
    // Step 1: get the connection from pool data
    let connection = pgdata_to_pgconnection(pool);

    // Step 2: delete the job from the db.
    match delete_job_by_id(&job_id, &connection) {
        Ok(deleted_job) => Ok(Json(deleted_job)),
        Err(_) => Err(Errors::InternalServerError(DBError.into())),
    }
}


#[api_v2_operation]
pub(crate) fn update_job_api(
    incoming_job: web::Json<Job>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<Job>, Errors> {
    // Step 1: get the connection from pool data.
    let connection = pgdata_to_pgconnection(pool);

    let job_id = &incoming_job.id.clone();

    // Step 2: search in the database for the required job.
    match find_job_by_id(job_id, &connection) {
        Ok(mut found_job) => {
            found_job.total_size = incoming_job.total_size;
            found_job.downloaded_size = incoming_job.downloaded_size;
            found_job.percent_downloaded = incoming_job.percent_downloaded;
            found_job.status = incoming_job.status.clone();
            found_job.is_active = incoming_job.is_active;
            found_job.creation_date = incoming_job.creation_date;
            found_job.expiration_date = incoming_job.expiration_date;

            // Step 3: update the job, then send response to the client.
            match update_job(&found_job, &connection) {
                Ok(updated_job) => Ok(Json(updated_job)),
                Err(_) => Err(Errors::InternalServerError(DBError.into())),
            }
        }
        Err(_) => {
            Err(Errors::NotFound(NotFound.into()))
        }
    }
}

#[api_v2_operation]
pub(crate) fn activate_job(
    web::Path((job_id, is_active)): web::Path<(Uuid, bool)>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<bool>, Errors> {
    // Step 1: get the connection from pool data
    let connection = pgdata_to_pgconnection(pool);

    // Step 2: activate/deactivate the job.
    match set_activate_job(&job_id, is_active, &connection) {
        Ok(state) => {
            if state == 1 {
                // Step 4: fire the response
                Ok(Json(true))
            } else {
                Err(Errors::NotFound(NotFound.into()))
            }
        }
        Err(_) => {
            Err(Errors::InternalServerError(DBError.into()))
        }
    }
}

#[api_v2_operation]
pub(crate) fn download_info(
    web::Path(job_id): web::Path<Uuid>,
    pool: web::Data<CoreDBPool>,
) -> Result<Json<JobInfo>, Errors> {
    // Step 1: get the connection from pool data
    let connection = pgdata_to_pgconnection(pool);

    // Step 2: Get the download info.
    match get_job_info(&job_id, &connection) {
        Ok(job_info) => Ok(Json(job_info)),
        Err(_) => Err(Errors::InternalServerError(NotFound.into()))
    }
}