use regex::{Error, Regex};

pub struct Patterns {
    pub email: Regex,
    pub username: Regex,
    pub password: Regex,
}

impl Patterns {
    pub fn new(email_reg: &str, username_reg: &str, password_reg: &str) -> Result<Self, Error> {
        let email = Regex::new(email_reg)?;
        let username = Regex::new(username_reg)?;
        let password = Regex::new(password_reg)?;
        Ok(Self {
            email,
            username,
            password,
        })
    }
}

// ===================================================================
// 测试：验证生产配置级别的正则表达式行为
// 正则字符串应保持与 config-dev.toml 中的实际配置一致
// ===================================================================

#[cfg(test)]
mod patterns_tests {
    use super::*;

    /// 构造与生产配置一致的正则模式
    fn prod_patterns() -> Patterns {
        Patterns::new(
            // email: 与 config 中的 regexes.email 保持一致
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
            // username: 与 config 中的 regexes.username 保持一致
            r"^[a-zA-Z0-9._%+-]{3,15}$",
            // password: 与 config 中的 regexes.password 保持一致
            r"^[A-Za-z0-9@$!%*?&]{8,12}$",
        )
        .unwrap()
    }

    // ── email ──

    mod email {
        use super::*;

        #[test]
        fn simple_valid() {
            assert!(prod_patterns().email.is_match("alice@example.com"));
        }

        #[test]
        fn with_subdomain() {
            assert!(prod_patterns().email.is_match("alice@mail.example.com"));
        }

        #[test]
        fn missing_at_rejected() {
            assert!(!prod_patterns().email.is_match("aliceexample.com"));
        }

        #[test]
        fn missing_tld_rejected() {
            assert!(!prod_patterns().email.is_match("alice@example"));
        }

        #[test]
        fn empty_rejected() {
            assert!(!prod_patterns().email.is_match(""));
        }

        #[test]
        fn unicode_rejected() {
            assert!(!prod_patterns().email.is_match("用户@example.com"));
        }
    }

    // ── username ──

    mod username {
        use super::*;

        #[test]
        fn pure_letters_valid() {
            assert!(prod_patterns().username.is_match("alice"));
        }

        #[test]
        fn min_length_valid() {
            assert!(prod_patterns().username.is_match("abc"));
        }

        #[test]
        fn max_length_valid() {
            assert!(prod_patterns().username.is_match("123456789012345"));
        }

        #[test]
        fn too_short_rejected() {
            assert!(!prod_patterns().username.is_match("ab"));
        }

        #[test]
        fn too_long_rejected() {
            assert!(!prod_patterns().username.is_match("1234567890123456"));
        }

        #[test]
        fn empty_rejected() {
            assert!(!prod_patterns().username.is_match(""));
        }

        #[test]
        fn unicode_rejected() {
            assert!(!prod_patterns().username.is_match("用户"));
        }
    }

    // ── password ──

    mod password {
        use super::*;

        #[test]
        fn meets_requirements_valid() {
            assert!(prod_patterns().password.is_match("Abcd1234!"));
        }

        #[test]
        fn min_length_valid() {
            assert!(prod_patterns().password.is_match("Abc123!x"));
        }

        #[test]
        fn max_length_valid() {
            assert!(prod_patterns().password.is_match("Abc123!xAbc1"));
        }

        #[test]
        fn empty_rejected() {
            assert!(!prod_patterns().password.is_match(""));
        }

        #[test]
        fn too_short_rejected() {
            assert!(!prod_patterns().password.is_match("Ab1!"));
        }

        #[test]
        fn too_long_rejected() {
            assert!(!prod_patterns().password.is_match(&"A".repeat(13)));
        }

        #[test]
        fn unicode_rejected() {
            assert!(!prod_patterns().password.is_match("Abcd密码123!"));
        }
    }
}
