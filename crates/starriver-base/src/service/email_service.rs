use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::Mailbox,
    transport::smtp::{Error, authentication::Credentials, response::Response},
};

use crate::{error::ApiError, service::config_service::EmailSmtp};

#[derive(Clone)]
pub struct EmailClient {
    smtp_client: AsyncSmtpTransport<Tokio1Executor>,
    username: String,
}

impl EmailClient {
    pub fn new(cfg: EmailSmtp) -> Result<Self, Error> {
        let creds = Credentials::new(cfg.username.clone(), cfg.password);
        let smtp_client = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&cfg.host)?
            .port(cfg.port)
            .credentials(creds)
            .build();
        Ok(Self {
            smtp_client,
            username: cfg.username,
        })
    }

    /// 发送邮件
    pub async fn send(&self, message: Message) -> Result<Response, Error> {
        self.smtp_client.send(message).await
    }

    /// 检查连接是否正常
    pub async fn test_conn(&self) -> Result<bool, Error> {
        self.smtp_client.test_connection().await
    }

    /// 发送邮件验证邮件
    pub async fn send_email_verification_mail(
        &self,
        to: &str,
        verification_code: String,
    ) -> Result<(), ApiError> {
        let to = to.parse::<Mailbox>().map_err(ApiError::with_inner_error)?;
        let from = self
            .username
            .parse::<Mailbox>()
            .map_err(ApiError::with_inner_error)?;
        let message = Message::builder()
            .subject("Starriver User's Email Verification")
            .from(from)
            .to(to)
            .body(format!("email verification code is {}", verification_code))
            .map_err(ApiError::with_inner_error)?;
        self.smtp_client
            .send(message)
            .await
            .map_err(ApiError::with_inner_error)?;
        Ok(())
    }
}

pub struct StmpConfig {
    ///邮箱服务器主机
    pub host: String,
    /// 邮箱服务器端口
    pub port: u16,
    /// 邮箱地址
    pub username: String,
    /// 邮箱密码
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_send() {
        let client = EmailClient::new(EmailSmtp {
            host: "".to_string(),
            port: 0,
            username: "".to_string(),
            password: "".to_string(),
        });
        assert_eq!(
            client.is_ok(),
            true,
            "测试邮件服务器连接失败，创建客户端失败"
        );
        let result = client.unwrap().test_conn().await;
        assert!(result.is_ok(), "测试邮件服务器连接失败，连接异常");
        let ok = result.unwrap();
        assert!(ok, "测试邮件服务器连接失败，未返回OK");
        assert_eq!(ok, true, "测试邮件服务器连接失败，返回OK，但其值不为真");
    }
}
