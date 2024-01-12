use std::str::FromStr;

#[derive(Debug, Clone, thiserror::Error, PartialEq)]
enum InvalidOddNumber {
    #[error("Not a number")]
    NaN,
    #[error("Was even, expected odd")]
    Even,
}

#[serde_with::serde_as]
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct OddNum {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    n: u8,
}
impl TryFrom<u8> for OddNum {
    type Error = InvalidOddNumber;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            return Err(InvalidOddNumber::Even);
        }
        Ok(Self { n: value })
    }
}
impl std::fmt::Display for OddNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.n)
    }
}

impl FromStr for OddNum {
    type Err = InvalidOddNumber;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u8 = s.parse().map_err(|_| InvalidOddNumber::NaN)?;
        Self::try_from(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeeds_for_valid() {
        let json = serde_json::json!({"n": "1"});
        assert_eq!(
            serde_json::from_value::<OddNum>(json).unwrap(),
            OddNum { n: 1 }
        );
    }

    #[test]
    fn failed_for_invalid() {
        let json = serde_json::json!({"n": "2"});
        match serde_json::from_value::<OddNum>(json) {
            Ok(_) => panic!("BAD! I expect this unit test to FAIL, since the number 2 is NOT odd, and the impl FromStr for OddNum uses TryFrom<u8> which returns on Err for even numbers."),
            Err(_) => println!("Good, JSON deserialization failed, as ex expected it to.")
        }
    }
}
