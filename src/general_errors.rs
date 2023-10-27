use proc_macro_magic::EnumError;

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum TypeError {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L459
    #[patterns("^Block header missing$")]
    BlockHeaderMissing,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L465
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L586
    #[patterns(
        "^In prune mode, only blocks that the node has already synced previously can be fetched from a peer$",
        "^Block not available (pruned data)$"
    )]
    BlockUnavailablePrunedNode,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L470
    #[patterns("^Block already downloaded$")]
    BlockAlreadyDownloaded,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L594
    #[patterns("^Block not found on disk$")]
    BlockNotFound,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L610
    #[patterns("^Undo data not available (pruned data)$")]
    UndoUnavailablePrunedNode,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L615
    #[patterns("^Can't read undo data from disk$")]
    UndoNotFound,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L781
    #[patterns("^Cannot prune blocks because node is not in prune mode.$")]
    PruneForbidden,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L807
    #[patterns("^Blockchain is too short for pruning.$")]
    ChainToShort,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L2181
    #[patterns("^scanobjects argument is required for the start action$")]
    ArgumentMissing,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L2383
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L2523
    #[patterns("^Index is not enabled for filtertype (.*)")]
    IndexFilterForbidden(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L2400
    #[patterns("^Invalid start_height$")]
    InvalidStartHeight,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L2406
    #[patterns("^Invalid stop_height$")]
    InvalidStopHeight,

    // TODO: Refactoring Core might be a good idea here
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L2551
    #[patterns("^Filter not found. Block filters are still in the process of being indexed.$")]
    IndexNotReady,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/external_signer.cpp#L43
    #[patterns("^Error: restart bitcoind with -signer=<cmd>$")]
    SignerArgMissing,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mempool.cpp#L331
    #[patterns("^Transaction is not in mempool$")]
    TxNotInMempool,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mempool.cpp#L742
    #[patterns("^The mempool was not loaded yet$")]
    MempoolNotLoaded,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mempool.cpp#L748
    #[patterns("^Unable to dump mempool to disk$")]
    CanNotDumpMempool,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L387
    #[patterns("^Failed to make block.$")]
    BlockGenerationFailed,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/output_script.cpp#L293
    #[patterns("^Unexpected empty result$")]
    EmptyAddresses,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L286
    #[patterns("^Block not available$")]
    BlockNotFound,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/addresses.cpp#L781
    #[patterns("^Failed to display address$")]
    FailedToDisplayAddress,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L91
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L1405
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L1699
    #[patterns("^Rescan aborted by user.$")]
    RescanUserAborted,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L1226
    #[patterns("^Missing required fields$")]
    MissingFields,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L1425
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L1723
    #[patterns("^Rescan failed for key with creation timestamp (.*). There was an error reading a \
                block from time (.*), which is after or within (.*) seconds of key creation, and \
                could contain transactions pertaining to the key. As a result, transactions \
                and coins using this key may not appear in the wallet. This error could be \
                caused by pruning or data corruption (see bitcoind log for details) and could \
                be dealt with by downloading and rescanning the relevant blocks (see -reindex \
                option and rescanblockchain RPC).$")]
    RescanFailed(String, String, String),
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/transactions.cpp#L907
    #[patterns("^Can't rescan beyond pruned data. Use RPC call getblockchaininfo to determine your pruned height.$")]
    ScanBeyondPrune,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/transactions.cpp#L919
    #[patterns("^Rescan failed. Potentially corrupted data files.$")]
    RescanFailed,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/transactions.cpp#L921
    #[patterns("^Rescan aborted.$")]
    RescanAborted,
    
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/wallet.cpp#L487
    #[patterns("^Requested wallet already unloaded$")]
    WalletUnloaded,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L474
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/external_signer.cpp#L56
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/server.cpp#510
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/spend.cpp#1125
    Generic(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum TypeError {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/signmessage.cpp#L49
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/signmessage.cpp#L55
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L675
    #[patterns("^Address does not refer to key$", "^Address does not refer to a key$")]
    AddressNoKey,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/signmessage.cpp#L51
    #[patterns("^Malformed base64 encoding$")]
    MalformedBase64,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L45
    #[patterns("^JSON value of type (.*) for field (.*) is not of expected type (.*)")]
    WrongJsonType(String, String),

    // TODO: Look for more like this
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L54
    #[patterns("^Unexpected key (.*)")]
    UnexpectedKey(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L68
    #[patterns("^Amount is not a number or string$")]
    AmountNotNumberOrString,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L71
    #[patterns("^Invalid amount$")]
    AmountInvalid,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L72
    #[patterns("^Amount out of range$")]
    AmountOutOfRange,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L568
    #[patterns("^Wrong type passed:\n(.*)")]
    WrongPassed(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L667
    #[patterns("^Missing data String key for proposal$")]
    MissingKeyForProposal,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L1246
    #[patterns("^Expected number or \"now\" timestamp value for key. got type (.*)")]
    WrongTimestamp(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L1248
    #[patterns("^Missing required timestamp field for key$")]
    MissingTimestamp,

    // // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction_util.cpp#L308
    #[patterns("^Missing amount for (.*)")]
    MissingAmountForCoins(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L46
    // TODO: The capture can be an arbitrary string. We can not distinguish it from
    // MissingKeyForProposal, MissingTimestamp or MissingAmountForCoins
    // This only works because the variants are checked in order!
    #[patterns("^Missing (.*)")]
    Missing(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum OutOfMemoryError {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L759
    #[patterns("^Out of memory$")]
    OutOfMemory,
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum DatabaseError {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/server_util.cpp#L46
    #[patterns("^Error: Ban database not loaded$")]
    BanNotLoaded,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L1594
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L1513
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/blockchain.cpp#L1554
    Generic(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum DeserializationError {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L350
    #[patterns("^Transaction decode failed for (.*). Make sure the tx has at least one input.$")]
    TxNoInput(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L634
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L960
    #[patterns("^Block decode failed$")]
    BlockDecodeFailed,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L964
    #[patterns("^Block does not start with a coinbase$")]
    BlockNoCoinbase,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L1024
    #[patterns("^Block header decode failed$")]
    BlockDecodeHeaderFailed,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction_util.cpp#L176
    #[patterns(r#"^expected object with \{"txid'","vout","scriptPubKey"\}$"#)]
    ObjectNoTxidVoutScriptPubKey,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction_util.cpp#L192
    #[patterns("^vout cannot be negative$")]
    NegativeVout,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction_util.cpp#L205
    #[patterns("^Previous output scriptPubKey mismatch:\n(.*)\nvs:\n(.*)")]
    PrevScriptPubKeyMismatch(String, String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mempool.cpp#L73
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L701
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/backup.cpp#L334
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/spend.cpp#L929
    #[patterns("^TX decode failed. Make sure the tx has at least one input.$")]
    TxNoOutput,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mempool.cpp#L174
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mempool.cpp#L819
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L573
    #[patterns(
        "^TX decode failed: (.*) Make sure the tx has at least one input.$",
        "^TX decode failed for tx (.*). Make sure the tx has at least one input.$"
    )]
    TxNoOutput2(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L980
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L1398
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L1445 
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L1606
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L1705
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L1821
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/spend.cpp#L1577
    #[patterns("^TX decode failed (.*)")]
    TxDecodeFailed(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L402 
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L1549
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/spend.cpp#L844
    #[patterns("^TX decode failed$")]
    TxDecodeFailed2,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L578
    #[patterns("^Missing transactions$")]
    TxMissing,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L1555
    #[patterns("^Inputs must not have scriptSigs and scriptWitnesses$")]
    SigsWitnessMissing,

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/wallet.cpp#L694
    #[patterns("^Transaction hex string decoding failure.$")]
    TxHexDecodeFailed,

    // No Pattern
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L345
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/util.cpp#L347
    Generic(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum VerifyError {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L379
    #[patterns("^TestBlockValidity failed: (.*)")]
    BlockValidityFailed(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L1030
    #[patterns("^Must submit previous header \((.*)\) first$")]
    PreviousHeaderMissing(String),

    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/rawtransaction.cpp#L612
    #[patterns("^Input not found or already spent$")]
    MissingOrSpend,

    // No Pattern
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L491
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L1038
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/mining.cpp#L1040
    Generic(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum WarmupError {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/rpc/server.cpp#L483
    Generic(String),
}

#[derive(Clone, Debug, PartialEq, EnumError)]
pub enum MethodDeprecated {
    // https://github.com/bitcoin/bitcoin/blob/v25.0/src/wallet/rpc/coins.cpp#L198
    #[patterns(r#"^dummy first argument must be excluded or set to "*".$"#)]
    WrongDummyArgument,
}
