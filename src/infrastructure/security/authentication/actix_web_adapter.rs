// use actix_session::Session;
// use actix_web::web::Json;
// use std::ops::Add;
//
// use crate::infrastructure::security::authentication::authentication::{
//     Authentication, AuthenticationError, AuthenticationFailureHandler,
//     AuthenticationSuccessHandler, Authenticator,
// };
// use crate::infrastructure::security::authentication::core::PrincipalType;
//
// struct ActixWebAuthenticationSuccessHandler {}
//
// impl AuthenticationSuccessHandler for ActixWebAuthenticationSuccessHandler {
//     type Output = Session;
//
//     fn handle(&self, authentication: &mut Authentication, output: &mut Self::Output) {
//         authentication.set_authenticated(true);
//         let principal = authentication.get_principal();
//         let username = principal.name();
//         output.insert("session", username).unwrap();
//     }
// }
//
// struct ActixWebAuthenticationFailureHandler {}
//
// impl AuthenticationFailureHandler for ActixWebAuthenticationFailureHandler {
//     type Output = Json<String>;
//
//     fn handle(&self, authentication: &mut Authentication, output: &mut Self::Output) {
//         let msg = String::from(authentication.get_principal().name()).add("-認證失敗了");
//         output.push_str(msg.as_str());
//     }
// }
//
// pub struct UsernamePasswordAuthenticator {}
//
// impl Authenticator for UsernamePasswordAuthenticator {
//     type SuccessOutput = Session;
//     type FailureOutput = Json<String>;
//
//     fn support_principal_type(&self) -> PrincipalType {
//         PrincipalType::UsernamePassword
//     }
//
//     fn authenticate(&self, authentication: &mut Authentication) -> Option<AuthenticationError> {
//         if authentication.get_principal().name() == "LZx" {
//             authentication.set_authenticated(true);
//             return None;
//         }
//         Some(AuthenticationError::UsernameNotFound)
//     }
//
//     fn success_handler(
//         &self,
//     ) -> Box<dyn AuthenticationSuccessHandler<Output = Self::SuccessOutput>> {
//         Box::new(ActixWebAuthenticationSuccessHandler {})
//     }
//
//     fn failure_handler(
//         &self,
//     ) -> Box<dyn AuthenticationFailureHandler<Output = Self::FailureOutput>> {
//         Box::new(ActixWebAuthenticationFailureHandler {})
//     }
// }
