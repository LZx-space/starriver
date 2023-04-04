// use crate::infrastructure::security::authentication::core::{Principal, PrincipalType};
//
// // get req params
// // 认证
// // 处理认证结果
// /// 认证器
// pub trait Authenticator {
//     type SuccessOutput;
//
//     type FailureOutput;
//
//     /// 返回支持的认证对象的类型
//     fn support_principal_type(&self) -> PrincipalType;
//
//     /// 认证，失败则返回认证异常
//     fn authenticate(&self, authentication: &mut Authentication) -> Option<AuthenticationError>;
//
//     /// 认证成功处理器
//     fn success_handler(&self) -> dyn AuthenticationSuccessHandler<Output = Self::SuccessOutput>;
//
//     /// 认证失败处理器
//     fn failure_handler(&self) -> dyn AuthenticationFailureHandler<Output = Self::FailureOutput>;
// }
//
// /// 认证错误
// pub enum AuthenticationError {
//     /// 用户名未发现
//     UsernameNotFound,
// }
//
// // todo 不应该传入2类结果的写入结果的参数，只要返回一个包含2种结果的构造体即可
// // todo 赋值不会发生heap复制，只会mv，开销小
// /// 认证成功的处理器
// pub trait AuthenticationSuccessHandler {
//     type Output;
//
//     fn handle(&self, authentication: &mut Authentication) -> Self::Output;
// }
//
// /// 认证失败的处理器
// pub trait AuthenticationFailureHandler {
//     type Output;
//
//     fn handle(&self, authentication: &mut Authentication) -> Self::Output;
// }
//
// pub struct DefaultPrincipal<'a> {
//     pub(crate) username: &'a String,
//     pub(crate) password: &'a String,
// }
//
// impl<'a> DefaultPrincipal<'a> {
//     fn new(username: &'a String, password: &'a String) -> DefaultPrincipal<'a> {
//         DefaultPrincipal { username, password }
//     }
// }
//
// impl<'a> Principal for DefaultPrincipal<'a> {
//     fn principal_type(&self) -> PrincipalType {
//         PrincipalType::UsernamePassword
//     }
//
//     fn name(&self) -> &String {
//         self.username
//     }
//
//     fn credentials(&self) -> &String {
//         self.password
//     }
// }
//
// /// 认证器调度员
// pub struct AuthenticatorDispatcher<S, F> {
//     pub(crate) authenticators: Vec<Box<dyn Authenticator<SuccessOutput = S, FailureOutput = F>>>,
// }
//
// impl<S, F> AuthenticatorDispatcher<S, F> {
//     /// 认证
//     pub fn authenticate(
//         self,
//         authentication: &mut Authentication,
//         success_writer: &mut S,
//         failure_writer: &mut F,
//     ) {
//         for authenticator in self.authenticators {
//             let support_principal_type = authenticator.support_principal_type();
//             if support_principal_type == authentication.principal.principal_type() {
//                 let result = authenticator.authenticate(authentication);
//                 if result.is_none() {
//                     let x = authenticator.success_handler();
//                     let s = x.handle(authentication);
//                 } else {
//                     let x = authenticator.failure_handler();
//                     let f = x.handle(authentication);
//                 };
//             }
//         }
//     }
// }
//
// // todo 一个统一认证入口
// // todo 判断是否需要认证
// // todo 判断是否已认证
// // todo 要求认证而未认证请求的的处理
//
// pub fn is_authenticated() -> bool {
//     false
// }
