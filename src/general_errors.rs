use crate::util::Pattern;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeError {
    AddressNoKey,
    MalformedBase64,
    Missing(String),
    WrongJsonType(String), // TODO: Pattern needs to accept multiple blanks
    UnexpectedKey(String),
    AmountNotNumberOrString,
    AmountInvalid,
    AmountOutOfRange,
    WrongPassed(String),
    MissingKeyForProposal,
    WrongTimestamp(String),
    MissingTimestamp,
    MissingAmountForCoins(String),
    Generic(String),
}

impl FromStr for TypeError {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/signmessage.cpp#L49
        // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/signmessage.cpp#L55
        let address_no_key: Pattern = "Address does not refer to key".into();

        // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/backup.cpp#L679
        let address_no_key2: Pattern = "Address does not refer to a key".into();

        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/signmessage.cpp#L51
        let malformed_base_64: Pattern = "Malformed base64 encoding".into();

        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L42 
        let missing: Pattern = "Missing {}".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L45
        let wrong_json_type: Pattern = "JSON value of type {}".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L55 
        let unexpected_key: Pattern = "Unexpected key {}".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L64
        let amount_not_num_string: Pattern = "Amount is not a number or string".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L67
        let amount_invalid: Pattern = "Invalid amount".into();

        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L69
        let amount_out_of_range: Pattern = "Amount out of range".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L589
        let wrong_passed: Pattern = "Wrong type passed:\n{}".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L664
        let missing_key_for_proposal: Pattern = "Missing data String key for proposal".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/backup.cpp#L1246
        let wrong_timestamp: Pattern = "Expected number or \"now\" timestamp value for key. got type {}".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/backup.cpp#L1248
        let missing_timestamp: Pattern = "Missing required timestamp field for key".into();
        
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/rawtransaction_util.cpp#L308
        let missing_amount_coins: Pattern = "Missing amount for {}".into();

        // No Pattern
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/server.cpp#L540
        
        if let Ok(None) = address_no_key.match_and_extract(s) {
            return Ok(TypeError::AddressNoKey);
        } else if let Ok(None) = address_no_key2.match_and_extract(s) {
            return Ok(TypeError::AddressNoKey);
        } else if let Ok(None) = malformed_base_64.match_and_extract(s) {
            return Ok(TypeError::MalformedBase64);
        } else if let Ok(Some(substring)) = missing.match_and_extract(s) {
            return Ok(TypeError::Missing(substring));
        } else if let Ok(Some(_)) = wrong_json_type.match_and_extract(s) {
            return Ok(TypeError::WrongJsonType(s.to_string()));
        } else if let Ok(Some(substring)) = unexpected_key.match_and_extract(s) {
            return Ok(TypeError::UnexpectedKey(substring));
        } else if let Ok(None) = amount_not_num_string.match_and_extract(s) {
            return Ok(TypeError::AmountNotNumberOrString);
        } else if let Ok(None) = amount_invalid.match_and_extract(s) {
            return Ok(TypeError::AmountInvalid);
        } else if let Ok(None) = amount_out_of_range.match_and_extract(s) {
            return Ok(TypeError::AmountOutOfRange);
        } else if let Ok(Some(substring)) = wrong_passed.match_and_extract(s) {
            return Ok(TypeError::WrongPassed(substring));
        } else if let Ok(None) = missing_key_for_proposal.match_and_extract(s) {
            return Ok(TypeError::MissingKeyForProposal);
        } else if let Ok(Some(substring)) = wrong_timestamp.match_and_extract(s) {
            return Ok(TypeError::WrongTimestamp(substring));
        } else if let Ok(None) = missing_timestamp.match_and_extract(s) {
            return Ok(TypeError::MissingTimestamp);
        } else if let Ok(Some(substring)) = missing_amount_coins.match_and_extract(s) {
            return Ok(TypeError::MissingAmountForCoins(substring));
        }

        Ok(TypeError::Generic(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum VerifyError {
    BlockValidityFailed(String),
    PreviousHeaderMissing(String),
    MissingOrSpend,
    Generic(String),
}

impl FromStr for VerifyError {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L379
        let block_validity_failed: Pattern = "TestBlockValidity failed: {}".into();

        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L1064
        let previous_header_missing: Pattern = "Must submit previous header ({}) first".into();

        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/rawtransaction.cpp#L697 
        let missing_or_spend: Pattern = "Input not found or already spent".into();
        
        // No Pattern
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L525
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L1072
        // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L1074
    
        if let Ok(Some(substring)) = block_validity_failed.match_and_extract(s) {
            return Ok(VerifyError::BlockValidityFailed(substring));
        } else if let Ok(Some(substring)) = previous_header_missing.match_and_extract(s) {
            return Ok(VerifyError::PreviousHeaderMissing(substring));
        } else if let Ok(None) = missing_or_spend.match_and_extract(s) {
            return Ok(VerifyError::MissingOrSpend);
        }

        Ok(VerifyError::Generic(s.to_string()))
    }
}
