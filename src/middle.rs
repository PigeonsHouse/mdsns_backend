use actix_web::HttpRequest;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceResponse, ServiceRequest};
use actix_web_lab::middleware::Next;
use fireauth::FireAuth;
use log::debug;
use crate::db::establish_connection;
use crate::cruds::{search_user_from_db, register_user};

#[derive(Debug)]
pub enum CheckFirebaseErr {
    TokenDoeNotExist,
    UserFirebaseNotFound,
    UserDbNotFound,
    InternalDbError,
}

pub type CheckFirebaseResult = Result<bool, CheckFirebaseErr>;

pub async fn check_firebase(request: &HttpRequest) -> CheckFirebaseResult {
    let api_key: String = std::env::var("FIREBASE_API").expect("FIREBASE_API does not exist !");
    let auth = FireAuth::new(api_key);
    // Authorization Header check
    let bearer = match request.headers().get("Authorization") {
        Some(bearer) => bearer,
        None => return Err(CheckFirebaseErr::TokenDoeNotExist),
    };
    debug!("bearer: {:?}", bearer);
    // Exist on firebase check
    let user_local_id = match auth.get_user_info(bearer.to_str().unwrap()).await {
        Ok(user) => user.local_id,
        Err(_) => return Err(CheckFirebaseErr::UserFirebaseNotFound),

    };
    // Exist on local db check
    debug!("local_id: {:?}", user_local_id);
    let mut conn = establish_connection();
    match search_user_from_db(&mut conn, user_local_id.clone()) {
        true => (),
        false => {
            // copy UserData from Firebase
            let user_info = match auth.get_user_info(bearer.to_str().unwrap()).await {
                Ok(user) => user,
                Err(_) => return Err(CheckFirebaseErr::UserFirebaseNotFound),
            };
            register_user(&mut conn, &user_info.local_id, &user_info.local_id, &user_info.email);
        },
    };

    Ok(true)
}

pub async fn middle_auth(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    debug!("req: {:?}", req);
    match check_firebase(req.request()).await {
        Err(e) => match e {
            CheckFirebaseErr::TokenDoeNotExist => return Err(actix_web::error::ErrorUnauthorized("missing token header")),
            CheckFirebaseErr::UserFirebaseNotFound => return Err(actix_web::error::ErrorUnauthorized("token does not exist on Firebase")),
            CheckFirebaseErr::InternalDbError => return Err(actix_web::error::ErrorInternalServerError("register at local DB is failed")),
            CheckFirebaseErr::UserDbNotFound => return Ok(next.call(req).await?)
        },
        Ok(_) => return Ok(next.call(req).await?)
    }
}

