#[derive(Debug)]
pub struct IdepmotencyKey(String);

impl TryFrom<String> for IdepmotencyKey {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            anyhow::bail!("The idempotency key cannot be empty.");
        }

        let max_length = 50;
        if value.len() >= max_length {
            anyhow::bail!("The idempotency key must be shorter than {max_length} characters.");
        }

        Ok(Self(value))
    }
}

impl From<IdepmotencyKey> for String {
    fn from(value: IdepmotencyKey) -> Self {
        value.0
    }
}

impl AsRef<str> for IdepmotencyKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
