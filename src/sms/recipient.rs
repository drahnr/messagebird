use super::*;

// requires manual Serialize/Deserialize impl
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename = "msisdn")]
pub struct Msisdn(u64);

impl Deref for Msisdn {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Mobile Subscriber Integrated Services Digital Network Number
///
/// A worldwide unique phone number. This does not require a `+` or `00` prefix before the country code.
impl Msisdn {
    pub fn new(raw: u64) -> Result<Self, MessageBirdError> {
        if raw != 0 {
            Ok(Msisdn(raw))
        } else {
            Err(MessageBirdError::TypeError {
                msg: format!("Invalid phone number: {}", raw),
            })
        }
    }
}

impl FromStr for Msisdn {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s).map_err(|_e| MessageBirdError::ParseError)
    }
}

impl ToString for Msisdn {
    fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Unknown,
    Scheduled,
    Sent,
    Buffered,
    Delivered,
    Expired,
    DeliveryFailed,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Scheduled => "scheduled",
            Status::Sent => "sent",
            Status::Buffered => "buffered",
            Status::Delivered => "delivered",
            Status::Expired => "expired",
            Status::DeliveryFailed => "delivery_failed",
            _ => "invalid",
        }
    }
}

impl FromStr for Status {
    type Err = MessageBirdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_plain::from_str::<Self>(s).map_err(|_e| MessageBirdError::ParseError)
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        serde_plain::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct Recipient {
    #[serde(rename = "recipient")]
    msisdn: Msisdn,
    #[serde(rename = "status")]
    status: Option<Status>,
    #[serde(rename = "statusDatetime")]
    status_datetime: Option<DateTime>,
}

impl Recipient {
    pub fn new(number: u64) -> Self {
        Self {
            msisdn: Msisdn(number),
            status: None,
            status_datetime: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    serde_roundtrip!(recipient_serde, Recipient, Recipient::new(1234512345));

    static RAW_1: &str = r#"{
"recipient": 23747,
"status": "delivery_failed",
"statusDatetime" : "2016-05-03T14:26:57+00:00"
}"#;
    deser_roundtrip!(recipient_sample1_deser, Recipient, RAW_1);

    //only one or the other can work at the same time, since serialize can only work one way at a time
    //     static RAW_2: &str = r#"{
    // "recipient": 23747,
    // }"#;
    //deser_roundtrip!(recipient_sample3_deser, Recipient, RAW_2);

    static RAW_3: &str = r#"{
    "recipient": 23747,
    "status": null,
    "statusDatetime" : null
    }"#;

    deser_roundtrip!(recipient_sample2_deser, Recipient, RAW_3);
}
