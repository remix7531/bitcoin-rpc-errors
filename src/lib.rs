mod general_errors;

#[cfg(test)]
mod test;

use crate::general_errors::{TypeError, VerifyError};
use std::str::FromStr;

// https://github.com/bitcoin/bitcoin/blob/master/src/rpc/protocol.h
#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Debug, PartialEq)]
pub enum RPCErrorCode {
    // General application defined errors
    RPC_MISC_ERROR,                // std::exception thrown in command handling
    RPC_TYPE_ERROR(TypeError),     // Unexpected type was passed as parameter
    RPC_INVALID_ADDRESS_OR_KEY,    // Invalid address or key
    RPC_OUT_OF_MEMORY,             // Ran out of memory during operation - No sub erros needed
    RPC_INVALID_PARAMETER,         // Invalid, missing or duplicate parameter
    RPC_DATABASE_ERROR,            // Database error
    RPC_DESERIALIZATION_ERROR,     // Error parsing or validating structure in raw format
    RPC_VERIFY_ERROR(VerifyError), // General error during transaction or block submission
    RPC_VERIFY_REJECTED,           // Transaction or block was rejected by network rules
    RPC_VERIFY_ALREADY_IN_CHAIN,   // Transaction already in chain
    RPC_IN_WARMUP,                 // Client still warming up
    RPC_METHOD_DEPRECATED,         // RPC method is deprecated

    // P2P client errors
    RPC_CLIENT_NOT_CONNECTED,         // Bitcoin is not connected
    RPC_CLIENT_IN_INITIAL_DOWNLOAD,   // Still downloading initial blocks
    RPC_CLIENT_NODE_ALREADY_ADDED,    // Node is already added
    RPC_CLIENT_NODE_NOT_ADDED,        // Node has not been added before
    RPC_CLIENT_NODE_NOT_CONNECTED,    // Node to disconnect not found in connected nodes
    RPC_CLIENT_INVALID_IP_OR_SUBNET,  // Invalid IP/Subnet
    RPC_CLIENT_P2P_DISABLED,          // No valid connection manager instance found
    RPC_CLIENT_NODE_CAPACITY_REACHED, // Max number of outbound or block-relay connections already open

    // Chain errors
    RPC_CLIENT_MEMPOOL_DISABLED, // No mempool instance found

    // Wallet errors
    RPC_WALLET_ERROR,                // Unspecified problem with wallet (key not found etc.)
    RPC_WALLET_INSUFFICIENT_FUNDS,   // Not enough funds in wallet or account
    RPC_WALLET_INVALID_LABEL_NAME,   // Invalid label name
    RPC_WALLET_KEYPOOL_RAN_OUT,      // Keypool ran out, call keypoolrefill first
    RPC_WALLET_UNLOCK_NEEDED,        // Enter the wallet passphrase with walletpassphrase first
    RPC_WALLET_PASSPHRASE_INCORRECT, // The wallet passphrase entered was incorrect
    RPC_WALLET_WRONG_ENC_STATE,      // Command given in wrong wallet encryption state (encrypting an encrypted wallet etc.)
    RPC_WALLET_ENCRYPTION_FAILED,    // Failed to encrypt the wallet
    RPC_WALLET_ALREADY_UNLOCKED,     // Wallet is already unlocked
    RPC_WALLET_NOT_FOUND,            // Invalid wallet specified
    RPC_WALLET_NOT_SPECIFIED,        // No wallet specified (error when there are multiple wallets loaded)
    RPC_WALLET_ALREADY_LOADED,       // This same wallet is already loaded
    RPC_WALLET_ALREADY_EXISTS,       // There is already a wallet with the same name

    // Unknown Error
    RPC_UNKOWN_ERROR(RpcError),
}

#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct RpcError {
    code: i32,
    message: String,
}

impl From<RpcError> for RPCErrorCode {
    fn from(error: RpcError) -> Self {
        match error {
            // General application defined errors
            RpcError {
                code: -3,
                message: m,
            } => RPCErrorCode::RPC_TYPE_ERROR(m.parse().unwrap()),
            RpcError { code: -5, .. } => RPCErrorCode::RPC_INVALID_ADDRESS_OR_KEY,
            RpcError { code: -7, .. } => RPCErrorCode::RPC_OUT_OF_MEMORY,
            RpcError { code: -8, .. } => RPCErrorCode::RPC_INVALID_PARAMETER,
            RpcError { code: -20, .. } => RPCErrorCode::RPC_DATABASE_ERROR,
            RpcError { code: -22, .. } => RPCErrorCode::RPC_DESERIALIZATION_ERROR,
            RpcError {
                code: -25,
                message: m,
            } => RPCErrorCode::RPC_VERIFY_ERROR(m.parse().unwrap()),
            RpcError { code: -26, .. } => RPCErrorCode::RPC_VERIFY_REJECTED,
            RpcError { code: -27, .. } => RPCErrorCode::RPC_VERIFY_ALREADY_IN_CHAIN,
            RpcError { code: -28, .. } => RPCErrorCode::RPC_IN_WARMUP,
            RpcError { code: -32, .. } => RPCErrorCode::RPC_METHOD_DEPRECATED,

            // P2P client errors
            RpcError { code: -9, .. } => RPCErrorCode::RPC_CLIENT_NOT_CONNECTED,
            RpcError { code: -10, .. } => RPCErrorCode::RPC_CLIENT_IN_INITIAL_DOWNLOAD,
            RpcError { code: -23, .. } => RPCErrorCode::RPC_CLIENT_NODE_ALREADY_ADDED,
            RpcError { code: -24, .. } => RPCErrorCode::RPC_CLIENT_NODE_NOT_ADDED,
            RpcError { code: -29, .. } => RPCErrorCode::RPC_CLIENT_NODE_NOT_CONNECTED,
            RpcError { code: -30, .. } => RPCErrorCode::RPC_CLIENT_INVALID_IP_OR_SUBNET,
            RpcError { code: -31, .. } => RPCErrorCode::RPC_CLIENT_P2P_DISABLED,
            RpcError { code: -34, .. } => RPCErrorCode::RPC_CLIENT_NODE_CAPACITY_REACHED,

            // Wallet errors
            RpcError { code: -4, .. } => RPCErrorCode::RPC_WALLET_ERROR,
            RpcError { code: -6, .. } => RPCErrorCode::RPC_WALLET_INSUFFICIENT_FUNDS,
            RpcError { code: -11, .. } => RPCErrorCode::RPC_WALLET_INVALID_LABEL_NAME,
            RpcError { code: -12, .. } => RPCErrorCode::RPC_WALLET_KEYPOOL_RAN_OUT,
            RpcError { code: -13, .. } => RPCErrorCode::RPC_WALLET_UNLOCK_NEEDED,
            RpcError { code: -14, .. } => RPCErrorCode::RPC_WALLET_PASSPHRASE_INCORRECT,
            RpcError { code: -15, .. } => RPCErrorCode::RPC_WALLET_WRONG_ENC_STATE,
            RpcError { code: -16, .. } => RPCErrorCode::RPC_WALLET_ENCRYPTION_FAILED,
            RpcError { code: -17, .. } => RPCErrorCode::RPC_WALLET_ALREADY_UNLOCKED,
            RpcError { code: -18, .. } => RPCErrorCode::RPC_WALLET_NOT_FOUND,
            RpcError { code: -19, .. } => RPCErrorCode::RPC_WALLET_NOT_SPECIFIED,
            RpcError { code: -35, .. } => RPCErrorCode::RPC_WALLET_ALREADY_LOADED,
            RpcError { code: -36, .. } => RPCErrorCode::RPC_WALLET_ALREADY_EXISTS,

            // Unknown Error
            _ => RPCErrorCode::RPC_UNKOWN_ERROR(error),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    JsonError(serde_json::Error),
    StartOfJsonMissing,
}

impl FromStr for RPCErrorCode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let json_str = match s.find('{') {
            Some(i) => &s[i..],
            None => return Err(ParseError::StartOfJsonMissing),
        };

        match serde_json::from_str::<RpcError>(json_str) {
            Ok(error) => Ok(error.into()),
            Err(e) => Err(ParseError::JsonError(e)),
        }
    }
}
