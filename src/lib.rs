mod general_errors;

#[cfg(test)]
mod test;

use crate::general_errors::{TypeError, VerifyError};
use regex::Regex;
use std::str::FromStr;

// https://github.com/bitcoin/bitcoin/blob/master/src/rpc/protocol.h
#[allow(non_camel_case_types)]
#[rustfmt::skip]
#[derive(Debug, PartialEq)]
pub enum Error {
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
    RPC_UNKOWN_ERROR(i32, String),
}

impl FromStr for Error {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"\{"code": (-?\d+), "message": "(.*?)"\}"#).unwrap();
        let captures = regex.captures(s).ok_or(())?;

        let code: i32 = captures
            .get(1)
            .ok_or(())?
            .as_str()
            .parse()
            .map_err(|_| ())?;
        let message: String = captures.get(2).ok_or(())?.as_str().to_string();

        match (code, &message) {
            // General application defined errors
            (-3, m) => Ok(Error::RPC_TYPE_ERROR(m.parse().unwrap())),
            (-5, _) => Ok(Error::RPC_INVALID_ADDRESS_OR_KEY),
            (-7, _) => Ok(Error::RPC_OUT_OF_MEMORY),
            (-8, _) => Ok(Error::RPC_INVALID_PARAMETER),
            (-20, _) => Ok(Error::RPC_DATABASE_ERROR),
            (-22, _) => Ok(Error::RPC_DESERIALIZATION_ERROR),
            (-25, m) => Ok(Error::RPC_VERIFY_ERROR(m.parse().unwrap())),
            (-26, _) => Ok(Error::RPC_VERIFY_REJECTED),
            (-27, _) => Ok(Error::RPC_VERIFY_ALREADY_IN_CHAIN),
            (-28, _) => Ok(Error::RPC_IN_WARMUP),
            (-32, _) => Ok(Error::RPC_METHOD_DEPRECATED),

            // P2P client errors
            (-9, _) => Ok(Error::RPC_CLIENT_NOT_CONNECTED),
            (-10, _) => Ok(Error::RPC_CLIENT_IN_INITIAL_DOWNLOAD),
            (-23, _) => Ok(Error::RPC_CLIENT_NODE_ALREADY_ADDED),
            (-24, _) => Ok(Error::RPC_CLIENT_NODE_NOT_ADDED),
            (-29, _) => Ok(Error::RPC_CLIENT_NODE_NOT_CONNECTED),
            (-30, _) => Ok(Error::RPC_CLIENT_INVALID_IP_OR_SUBNET),
            (-31, _) => Ok(Error::RPC_CLIENT_P2P_DISABLED),
            (-34, _) => Ok(Error::RPC_CLIENT_NODE_CAPACITY_REACHED),

            // Wallet errors
            (-4, _) => Ok(Error::RPC_WALLET_ERROR),
            (-6, _) => Ok(Error::RPC_WALLET_INSUFFICIENT_FUNDS),
            (-11, _) => Ok(Error::RPC_WALLET_INVALID_LABEL_NAME),
            (-12, _) => Ok(Error::RPC_WALLET_KEYPOOL_RAN_OUT),
            (-13, _) => Ok(Error::RPC_WALLET_UNLOCK_NEEDED),
            (-14, _) => Ok(Error::RPC_WALLET_PASSPHRASE_INCORRECT),
            (-15, _) => Ok(Error::RPC_WALLET_WRONG_ENC_STATE),
            (-16, _) => Ok(Error::RPC_WALLET_ENCRYPTION_FAILED),
            (-17, _) => Ok(Error::RPC_WALLET_ALREADY_UNLOCKED),
            (-18, _) => Ok(Error::RPC_WALLET_NOT_FOUND),
            (-19, _) => Ok(Error::RPC_WALLET_NOT_SPECIFIED),
            (-35, _) => Ok(Error::RPC_WALLET_ALREADY_LOADED),
            (-36, _) => Ok(Error::RPC_WALLET_ALREADY_EXISTS),

            // Unknown Error
            _ => Ok(Error::RPC_UNKOWN_ERROR(code, message)),
        }
    }
}
