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

use serde::{Deserialize, Serialize};

pub mod delete;
pub mod email;
#[cfg(test)]
pub mod test;
pub mod username;

pub use super::auth;

pub mod routes {

    pub struct Account {
        pub delete: &'static str,
        pub email_exists: &'static str,
        pub update_email: &'static str,
        pub username_exists: &'static str,
    }

    impl Account {
        pub const fn new() -> Account {
            let delete = "/api/v1/account/delete";
            let email_exists = "/api/v1/account/email/exists";
            let username_exists = "/api/v1/account/username/exists";
            let update_email = "/api/v1/account/email/update";
            Account {
                delete,
                email_exists,
                update_email,
                username_exists,
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckPayload {
    pub val: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckResp {
    pub exists: bool,
}

pub fn services(cfg: &mut actix_web::web::ServiceConfig) {
    delete::services(cfg);
    email::services(cfg);
    username::services(cfg);
}
