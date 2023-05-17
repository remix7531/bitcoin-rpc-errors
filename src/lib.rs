#[derive(Clone, Debug, PartialEq)]
/// The reason the backend rejected the transaction we tried to broadcast.
pub enum BroadcastTxError {
    /// The transaction failed to verify (e.g. it had an invalid signature)
    // #[error("the transaction was rejected by network rules ({0})")]
    VerifyRejected(String),
    /// The transaction was generally invalid (e.g. one of the inputs it is spending from doesn't exist)
    // #[error("there was a general error while verifying the transaction ({0})")]
    VerifyError(String),
    /// That transaction has already been confirmed.
    // #[error("the transaction has already been broadcast")]
    AlreadyInChain,
    /// The wallet contained an input from a coinbase transaction that wasn't matured yet.
    // #[error("premature spend of coinbase output")]
    PrematureSpendOfCoinbase,
    /// The transaction conflicts with one that is already in the mempool (and that one is not
    /// replaceable).
    // #[error("the transaction conflicts with one that is already in the mempool")]
    ConflictsWithMempool,
    /// The transaction has an input that is missing or spent.
    // #[error("the transaction has an input that is missing or spent")]
    MissingOrSpent,
    /// At least one of the inputs had a witness  or script pubkey that did not satisfy the script pubkey
    // #[error("the witness or scriptsig was invalid for one of the inputs (#{0})")]
    ScriptPubkeyNotSatisfied(String),
}

impl BroadcastTxError {
    /// parses a bitcoin core rpc `sendrawtransaction` call error response.
    pub fn from_core_rpc_response(text: &str) -> Option<Self> {
        text.strip_prefix("sendrawtransaction RPC error: ")
            .and_then(|text| match serde_json::from_str::<RpcError>(&text) {
                Ok(rpc_error) => Some(match rpc_error.code {
                    -25 => {
                        if rpc_error
                            .message
                            .starts_with("bad-txns-inputs-missingorspent")
                        {
                            BroadcastTxError::MissingOrSpent
                        } else {
                            BroadcastTxError::VerifyError(rpc_error.message)
                        }
                    }
                    -26 => {
                        if rpc_error
                            .message
                            .starts_with("bad-txns-premature-spend-of-coinbase")
                        {
                            BroadcastTxError::PrematureSpendOfCoinbase
                        } else if rpc_error.message.starts_with("txn-mempool-conflict") {
                            BroadcastTxError::ConflictsWithMempool
                        } else if let Some(remaining) = rpc_error
                            .message
                            .strip_prefix("non-mandatory-script-verify-flag")
                        {
                            let remaining =
                                remaining.trim_start_matches(" (").trim_end_matches(")");
                            BroadcastTxError::ScriptPubkeyNotSatisfied(remaining.into())
                        } else {
                            BroadcastTxError::VerifyRejected(rpc_error.message)
                        }
                    }
                    -27 => BroadcastTxError::AlreadyInChain,
                    _ => return None,
                }),
                Err(_e) => None,
            })
    }
}

#[derive(serde::Deserialize, Debug)]
struct RpcError {
    code: i32,
    message: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
