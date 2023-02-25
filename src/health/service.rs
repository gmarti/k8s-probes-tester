use actix_web::web;

use super::alive;
use super::ready;

pub fn service() -> actix_web::Scope {
    web::scope("/health").service(alive::service()).service(ready::service())
}
