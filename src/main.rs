#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::serde::json::Json;

mod types;
use crate::types::{Lock, State};

static mut STATE: Option<State> = None;
static mut LOCK: Option<Lock> = None;

#[put("/lock", data = "<lock>")]
fn lock<'a>(lock: Json<Lock>) -> (Status, Json<Option<Lock>>) {
    unsafe {
        if LOCK.is_some() {
            return (Status::Conflict, Json::from(LOCK.clone()));
        }

        LOCK = Some(lock.into_inner());
        (Status::Ok, Json::from(LOCK.clone()))
    }
}

#[delete("/lock")]
fn unlock() -> Status {
    unsafe {
        LOCK = None;
    }
    Status::Ok
}

#[get("/state")]
fn get_state() -> (Status, Json<Option<State>>) {
    unsafe {
        if STATE.is_some() {
            dbg!(&STATE);
            return (Status::Ok, STATE.clone().into());
        }
        (Status::NotFound, Json::from(None))
    }
}

#[post("/state?<ID>", data = "<state>")]
#[allow(non_snake_case)] // Cannot be renamed to "id"
fn set_state(ID: String, state: Json<State>) -> Status {
    unsafe {
        if LOCK.is_some() && LOCK.as_ref().unwrap().id != ID {
            return Status::Conflict;
        }

        STATE = Some(state.into_inner());
    }
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![lock, unlock, get_state, set_state])
}
