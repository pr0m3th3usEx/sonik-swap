use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug)]
pub enum DeezerErrorType {
    Quota,
    ItemsLimitExceeded,
    PERMISSION,
    TokenInvalid,
    Parameter,
    ParameterMissing,
    QueryInvalid,
    ServiceBusy,
    DataNotFound,
    IndividualAccountNotAllowed,
}

impl Display for DeezerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct DeezerErrorPayload {
    #[allow(dead_code)]
    pub error: DeezerError,
}

#[derive(Debug, Deserialize)]
pub struct DeezerError {
    #[allow(dead_code)]
    #[serde(rename = "type")]
    pub error_type: String,
    #[allow(dead_code)]
    pub message: String,
    pub code: Option<u16>,
}

impl TryFrom<DeezerError> for DeezerErrorType {
    type Error = &'static str;

    fn try_from(error: DeezerError) -> Result<Self, Self::Error> {
        if let Some(code) = error.code {
            match code {
                4 => Ok(DeezerErrorType::Quota),
                100 => Ok(DeezerErrorType::ItemsLimitExceeded),
                200 => Ok(DeezerErrorType::PERMISSION),
                300 => Ok(DeezerErrorType::TokenInvalid),
                400 => Ok(DeezerErrorType::Parameter),
                500 => Ok(DeezerErrorType::ParameterMissing),
                600 => Ok(DeezerErrorType::QueryInvalid),
                700 => Ok(DeezerErrorType::ServiceBusy),
                800 => Ok(DeezerErrorType::DataNotFound),
                900 => Ok(DeezerErrorType::IndividualAccountNotAllowed),
                _ => Err("DeezerError: Unknown value"),
            }
        } else {
            match error.error_type.as_str() {
                "Exception" => Ok(DeezerErrorType::Quota),
                // "Exception" => Ok(DeezerErrorType::ItemsLimitExceeded),
                "OAuthException" => Ok(DeezerErrorType::PERMISSION),
                // "OAuthException" => Ok(DeezerErrorType::TokenInvalid),
                "ParameterException" => Ok(DeezerErrorType::Parameter),
                "MissingParameterException" => Ok(DeezerErrorType::ParameterMissing),
                "InvalidQueryException  " => Ok(DeezerErrorType::QueryInvalid),
                // "Exception" => Ok(DeezerErrorType::ServiceBusy),
                "DataException" => Ok(DeezerErrorType::DataNotFound),
                "IndividualAccountChangedNotAllowedException" => {
                    Ok(DeezerErrorType::IndividualAccountNotAllowed)
                }
                _ => Err("DeezerError: Unknown value"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DeezerError;

    #[test]
    fn test_deserialize_error() {
        let json_str = "{\"type\":\"OAuthException\",\"message\":\"An active access token must be used to query information about the current user\",\"code\":200}";
        let json = serde_json::from_str::<DeezerError>(&json_str).expect("valid json");

        assert_eq!(json.error_type, "OAuthException")
    }
}
