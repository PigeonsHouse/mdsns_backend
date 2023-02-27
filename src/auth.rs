use actix_web::{HttpRequest, HttpResponse};
use fireauth::FireAuth;

#[derive(Debug)]
pub enum MinimalAuthErr {
    TokenNotFound,
    UserFirebaseNotFound,
    UserDbNotFound,
}

pub type MinimalAuthResult = Result<String, MinimalAuthErr>;


pub async fn minimal_auth(request: &HttpRequest) -> MinimalAuthResult {
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    // Authorization check
    let bearer = match request.headers().get("Authorization") {
        Some(bearer) => bearer,
        None => return Err(MinimalAuthErr::TokenNotFound),
    };
    // user exist check
    let user_local_id = match auth.get_user_info(bearer.to_str().unwrap()).await {
        Ok(user) => user.local_id,
        Err(_) => return Err(MinimalAuthErr::UserFirebaseNotFound),

    };
    // TODO: diesel
    /*
    let conn = establish_connection();
    match db_sign_in(&conn, user_local_id.clone()) {
        true => (),
        false => return Err(MinimalAuthErr::UserDbNotFound),
    };
    */

    Ok(user_local_id)
}
