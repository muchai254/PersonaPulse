use candid::{CandidType, Principal};
use ic_cdk_macros::{query, update};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::BTreeMap;


#[query(name = "__get_candid_interface_tmp_hack")]
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

type Passphrase = String;
type Username = String;
type Registry = BTreeMap<Passphrase, Vec<Username>>;

thread_local! {
    static STATE: RefCell<Registry> = RefCell::new(Registry::new());
}

#[update]
fn register_passphrase(passphrase: Passphrase, usernames: Vec<Username>) -> Result<(), String> {
    if passphrase.is_empty() {
        return Err("Passphrase cannot be empty.".to_string());
    }
    if usernames.is_empty() {
        return Err("Usernames list cannot be empty.".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.insert(passphrase, usernames);
    });

    Ok(())
}

#[query]
fn get_usernames(passphrase: Passphrase) -> Option<Vec<Username>> {
    STATE.with(|state| {
        let state = state.borrow();
        state.get(&passphrase).cloned()
    })
}

#[update]
async fn get_profile_by_username(username: Username) -> Result<UserDetails, String> {
    let passphrase = STATE.with(|state| {
        let state = state.borrow();
        for (p, u_vec) in state.iter() {
            if u_vec.contains(&username) {
                return Some(p.clone());
            }
        }
        None
    });

    if passphrase.is_none() {
        return Err("Username not found in registry.".to_string());
    }

    let found_passphrase = passphrase.unwrap();

   
    let profile_canister_id = Principal::from_text("uxrrr-q7777-77774-qaaaq-cai")
        .expect("Could not decode the principal.");

    let call_result: Result<(Option<UserDetails>,), _> = ic_cdk::call(
        profile_canister_id,
        "get_user_details_by_passkey",
        (found_passphrase,),
    )
    .await;

    match call_result {
        Ok((details_option,)) => match details_option {
            Some(details) => Ok(details),
            None => Err("User found in registry, but no details found in profile canister.".to_string()),
        },
        Err((code, msg)) => Err(format!("Inter-canister call failed: ({:?}) {}", code, msg)),
    }
}
