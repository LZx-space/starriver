use crate::infrastructure::model::page::PageQuery;
// use crate::infrastructure::security::authentication::actix_web_adapter::UsernamePasswordAuthenticator;
use crate::infrastructure::security::authentication::core::ClientDetails;
use actix_session::Session;
use actix_web::web::{Json, Query};
use actix_web::{get, post, web, Handler, Responder};

#[get("/sessions")]
pub async fn login_in(mut session: Session, params: Query<PageQuery>) -> impl Responder {
    let p = params.into_inner();
    // println!("{:?}", p);
    // let x = Box::new(UsernamePasswordAuthenticator {});
    // let dispatcher = AuthenticatorDispatcher {
    //     authenticators: vec![x],
    // };
    // let principal = DefaultPrincipal {
    //     username: &"LZx".to_string(),
    //     password: &"".to_string(),
    // };
    // let mut authentication = Authentication::new(&principal, ClientDetails {});
    // let mut json = Json(String::new());
    // dispatcher.authenticate(&mut authentication, &mut session, &mut json);
    Json("123")
}
