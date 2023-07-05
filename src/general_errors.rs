use proc_macro_magic::EnumError;

#[derive(Clone, Debug, PartialEq, EnumError)]
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

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum OutOfMemoryError {
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mining.cpp#L793
    #[patterns("^Out of memory$")]
    OutOfMemory,
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum DatabaseError {
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/server_util.cpp#L47
    #[patterns("^Error: Ban database not loaded$")]
    BanNotLoaded,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/blockchain.cpp#L1594
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/blockchain.cpp#L1513
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/blockchain.cpp#L1554
    Generic(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum DeserializationError {
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mining.cpp#L350
    #[patterns("^Transaction decode failed for (.*). Make sure the tx has at least one input.$")]
    TxNoInput(String),

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mining.cpp#L668
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mining.cpp#L994
    #[patterns("^Block decode failed$")]
    BlockDecodeFailed,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mining.cpp#L998
    #[patterns("^Block does not start with a coinbase$")]
    BlockNoCoinbase,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mining.cpp#L1058
    #[patterns("^Block header decode failed$")]
    BlockDecodeHeaderFailed,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction_util.cpp#L176
    #[patterns(r#"^expected object with \{"txid'","vout","scriptPubKey"\}$"#)]
    ObjectNoTxidVoutScriptPubKey,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction_util.cpp#L192
    #[patterns("^vout cannot be negative$")]
    NegativeVout,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction_util.cpp#L205
    #[patterns("^Previous output scriptPubKey mismatch:\n(.*)\nvs:\n(.*)")]
    PrevScriptPubKeyMismatch(String, String),

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mempool.cpp#L73
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L786
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/wallet/rpc/backup.cpp#L335
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/wallet/rpc/spend.cpp#L929
    #[patterns("^TX decode failed. Make sure the tx has at least one input.$")]
    TxNoOutput,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mempool.cpp#L174
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/mempool.cpp#L819
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L658
    #[patterns(
        "^TX decode failed: (.*) Make sure the tx has at least one input.$",
        "^TX decode failed for tx (.*). Make sure the tx has at least one input.$"
    )]
    TxNoOutput2(String),

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1483
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L181
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1065
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1483
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1530
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1744
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1860
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/wallet/rpc/spend.cpp#L1577
    #[patterns("^TX decode failed (.*)$")]
    TxDecodeFailed(String),

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L487
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1634
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/wallet/rpc/spend.cpp#L844
    #[patterns("^TX decode failed$")]
    TxDecodeFailed2,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L663
    #[patterns("^Missing transactions$")]
    TxMissing,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/rawtransaction.cpp#L1640
    #[patterns("^Inputs must not have scriptSigs and scriptWitnesses$")]
    SigsWitnessMissing,

    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/wallet/rpc/wallet.cpp#L689
    #[patterns("^Transaction hex string decoding failure.$")]
    TxHexDecodeFailed,

    // No Pattern
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/util.cpp#L345
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/util.cpp#L347
    Generic(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
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

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum WarmupError {
    // https://github.com/bitcoin/bitcoin/blob/bc4f6b13feb29146b7e10e86f93dc7f6fb6937f2/src/rpc/server.cpp#L515
    Generic(String),
}
