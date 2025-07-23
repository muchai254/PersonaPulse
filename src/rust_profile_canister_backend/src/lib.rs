use candid::{CandidType, Decode, Encode, Principal};
use ic_cdk::api::caller;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    ic_cdk::export_candid!();
    String::new()
}


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct UserDetails {
    firstName: String,
    lastName: String,
    skills: String,
    clientReviews: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
enum QueryField {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "firstName")]
    FirstName,
    #[serde(rename = "lastName")]
    LastName,
    #[serde(rename = "skills")]
    Skills,
    #[serde(rename = "clientReviews")]
    ClientReviews,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
enum ReadResult {
    #[serde(rename = "all")]
    All(UserDetails),
    #[serde(rename = "field")]
    Field(String),
    #[serde(rename = "notFound")]
    NotFound,
    #[serde(rename = "error")]
    Error(String),
}

type UserRegistry = BTreeMap<String, UserDetails>;

thread_local! {
    static STATE: RefCell<UserRegistry> = RefCell::new(UserRegistry::new());
}

#[ic_cdk::update]
fn write(passkey: String, details: UserDetails) -> String {
    if passkey.is_empty() {
        return "Error: Passkey cannot be empty.".to_string();
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.insert(passkey, details);
    });
    "User details successfully saved.".to_string()
}

#[ic_cdk::query]
fn read(passkey: String, field: Option<QueryField>) -> ReadResult {
    STATE.with(|state| {
        let state = state.borrow();
        match state.get(&passkey) {
            None => ReadResult::NotFound,
            Some(details) => {
                let query = field.unwrap_or(QueryField::All);
                match query {
                    QueryField::All => ReadResult::All(details.clone()),
                    QueryField::FirstName => ReadResult::Field(details.firstName.clone()),
                    QueryField::LastName => ReadResult::Field(details.lastName.clone()),
                    QueryField::Skills => ReadResult::Field(details.skills.clone()),
                    QueryField::ClientReviews => ReadResult::Field(details.clientReviews.clone()),
                }
            }
        }
    })
}

#[ic_cdk::update]
fn get_user_details_by_passkey(passkey: String) -> Option<UserDetails> {
    STATE.with(|state| {
        let state = state.borrow();
        state.get(&passkey).cloned()
    })
}


candid::export_service!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn did_generation() {
        let did = __export_service();
        std::fs::write("rust_profile_canister_backend.did", did).unwrap();
    }
}