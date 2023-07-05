use proc_macro_magic::ErrorEnum;

#[derive(Clone, Debug, PartialEq, ErrorEnum)]
pub enum TypeError {
    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/signmessage.cpp#L49
    // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/signmessage.cpp#L55
    // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/backup.cpp#L679
    #[patterns("Address does not refer to key", "Address does not refer to a key")]
    AddressNoKey,

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/signmessage.cpp#L51
    #[patterns("Malformed base64 encoding")]
    MalformedBase64,

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L45
    #[patterns("JSON value of type (.*)")]
    WrongJsonType(String),

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L55
    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L55
    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L55
    #[patterns("Unexpected key (.*)")]
    UnexpectedKey(String),

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L64
    #[patterns("Amount is not a number or string")]
    AmountNotNumberOrString,

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L67
    #[patterns("Invalid amount")]
    AmountInvalid,

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L69
    #[patterns("Amount out of range")]
    AmountOutOfRange,

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L589
    #[patterns("Wrong type passed:\n(.*)")]
    WrongPassed(String),

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L664
    #[patterns("Missing data String key for proposal")]
    MissingKeyForProposal,

    // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/backup.cpp#L1246
    #[patterns("Expected number or \"now\" timestamp value for key. got type (.*)")]
    WrongTimestamp(String),

    // https://github.com/bitcoin/bitcoin/blob/master/src/wallet/rpc/backup.cpp#L1248
    #[patterns("Missing required timestamp field for key")]
    MissingTimestamp,

    // // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/rawtransaction_util.cpp#L308
    #[patterns("Missing amount for (.*)")]
    MissingAmountForCoins(String),

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/util.cpp#L42
    // TODO: The capture can be an arbitrary string. We can not distinguish it from
    // MissingKeyForProposal, MissingTimestamp or MissingAmountForCoins
    // This only works because the variants are checked in order!
    #[patterns("Missing (.*)")]
    Missing(String),
}

#[derive(Clone, Debug, PartialEq, ErrorEnum)]
pub enum VerifyError {
    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L379
    #[patterns("TestBlockValidity failed: (.*)")]
    BlockValidityFailed(String),

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L1064
    #[patterns(r"Must submit previous header \((.*)\) first")]
    PreviousHeaderMissing(String),

    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/rawtransaction.cpp#L697
    #[patterns("Input not found or already spent")]
    MissingOrSpend,

    // No Pattern
    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L525
    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L1072
    // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L1074
    Generic(String),
}
