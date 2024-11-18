/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};

use super::auth::runners::Password;
use crate::errors::*;
use crate::AppData;

#[my_codegen::post(
    path = "crate::V1_API_ROUTES.account.delete",
    wrap = "crate::CheckLogin"
)]
async fn delete_account(
    id: Identity,
    payload: web::Json<Password>,
    data: AppData,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;
    use sqlx::Error::RowNotFound;

    let username = id.identity().unwrap();

    let rec = sqlx::query_as!(
        Password,
        r#"SELECT password  FROM triox_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await;

    match rec {
        Ok(s) => {
            if Config::verify(&s.password, &payload.password)? {
                sqlx::query!("DELETE FROM triox_users WHERE name = ($1)", &username)
                    .execute(&data.db)
                    .await?;

                // delete storage path of the user
                let path: std::path::PathBuf =
                    [".", "data", "users", &username].iter().collect();

                std::fs::remove_dir_all(path).map_err(|err| {
                    log::error!("STORAGE PATH: {:?}", err);
                    err
                })?;
                id.forget();
                Ok(HttpResponse::Ok())
            } else {
                Err(ServiceError::InvalidCredentials)
            }
        }
        Err(RowNotFound) => Err(ServiceError::AccountNotFound),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(delete_account);
}
