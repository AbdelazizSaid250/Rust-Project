use paperclip::actix::web;
use paperclip::actix::web::ServiceConfig;

use crate::handler::job::{activate_job, add_job, list_paginated_jobs, remove_job_by_id, update_job_api};

pub mod job;

pub fn routes(config: &mut ServiceConfig) {
    config
        .service(
            web::scope("/feature")
                // .wrap(AuthMiddleware)
                .route("", web::get().to(list_paginated_jobs))
                .route("/update", web::put().to(update_job_api))
                .route("/add", web::post().to(add_job))
                .route("/remove/{feature_id}", web::delete().to(remove_job_by_id))
                .route(
                    "/{feature_id}/activate/{is_active}",
                    web::put().to(activate_job),
                ),
        );
}