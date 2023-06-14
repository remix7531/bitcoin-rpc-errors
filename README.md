# bitcoin_rpc_errors
Parse errors returned by bitcoin core's rpc interface

bitcoin_rpc_errors is a Rust library designed to parse errors returned by [Bitcoin Core's](https://github.com/bitcoin/bitcoin) RPC interface. 
Please note that it is currently in active development and subject to changes.

## How to use it
Given a string containig an error message we can call `parse()` to get the RPCErrorCode enum.
```rust
let error_str: String = r#"RPC_VERIFY_ERROR occured: {"code": -25, "message": "Input not found or already spent"}"#.to_string();
let error: crate::RPCErrorCode = error_str.parse().unwrap();
        
assert_eq!(error, crate::RPCErrorCode::RPC_VERIFY_ERROR(crate::VerifyError::MissingOrSpend))
```

## How it works
Bitcoin Core implements various [RPC errors](https://github.com/bitcoin/bitcoin/blob/427853ab49f610e971b73ea4cc1d5366747e52b1/src/rpc/protocol.h#L23), 
each of which is returned as a JSON object containing a `code` and a `message`.
The `code` represents a broad error category. Every enum variant in `RPCErrorCode` corresponds to a specific error `code`.

```rust
pub enum RPCErrorCode {
  ...
  RPC_VERIFY_ERROR(VerifyError), // General error during transaction or block submission
  ...
}
```
The `RPC_VERIFY_ERROR` itself contains an enum named `VerifyError`. `RPC_VERIFY_ERROR` has error code -25. When you parse an error bitcoin_rpc_errors matches the code and message.
```rust
match error {
  ...
  RpcError { code: -25, message: m, } => RPCErrorCode::RPC_VERIFY_ERROR(VerifyError::from_str(&m).unwrap())
  ...
}
```
`VerifyError` itself is an enum. It contains all the reasons a `RPC_VERIFY_ERROR` may occure.
Not all `RPC_VERIFY_ERROR` contain a message that can be interpreted.
For example in [mining.cpp](https://github.com/bitcoin/bitcoin/blob/427853ab49f610e971b73ea4cc1d5366747e52b1/src/rpc/mining.cpp#L525) the error message just contains the state.
Multiple erros like this may occure. In this case the Generic(string) is used. 
```rust
pub enum VerifyError {
    BlockValidityFailed(String),
    PreviousHeaderMissing(String),
    MissingOrSpend,
    Generic(String),
}
```
The `from_str(s: &str)` for `VerifyError` works as followed: 
```rust
fn from_str(s: &str) -> Result<Self, Self::Err> {
  // https://github.com/bitcoin/bitcoin/blob/master/src/rpc/mining.cpp#L379
  let block_validity_failed: Pattern = "TestBlockValidity failed: {}".into();
  ...
        
  if let Ok(Some(substring)) = block_validity_failed.match_and_extract(s) {
    return Ok(VerifyError::BlockValidityFailed(substring));
  } 
  ...

  Ok(VerifyError::Generic(s.to_string())
}
```
`Pattern` is a struct is a reverse format. 
Given a format string the struct is created. `match_and_extract` returns whatever is inside the `{}` if the string matches. `Pattern` may not contain `{}`,
then it check if the giving strings are equal. This is the case for the `MissingOrSpend` error because it only contain a static message.
```rust
let missing_or_spend: Pattern = "Input not found or already spent".into()
```

## State
The following erorrs parse the message or do not need parsing because they do not return a message:

### General application defined error
- [ ] RPC_MISC_ERROR				// std::exception thrown in command handling
- [x] RPC_TYPE_ERROR				// Unexpected type was passed as parameter
- [ ] RPC_INVALID_ADDRESS_OR_KEY	// Invalid address or key
- [x] RPC_OUT_OF_MEMORY				// Ran out of memory during operation
- [ ] RPC_INVALID_PARAMETER			// Invalid, missing or duplicate parameter
- [ ] RPC_DATABASE_ERROR			// Database error
- [ ] RPC_DESERIALIZATION_ERROR		// Error parsing or validating structure in raw format
- [x] RPC_VERIFY_ERROR				// General error during transaction or block submission
- [ ] RPC_VERIFY_REJECTED			// Transaction or block was rejected by network rules
- [ ] RPC_VERIFY_ALREADY_IN_CHAIN	// Transaction already in chain
- [ ] RPC_IN_WARMUP					// Client still warming up
- [ ] RPC_METHOD_DEPRECATED			// RPC method is deprecated

### P2P client errors
- [ ] RPC_CLIENT_NOT_CONNECTED			// Bitcoin is not connected
- [ ] RPC_CLIENT_IN_INITIAL_DOWNLOAD	// Still downloading initial blocks
- [ ] RPC_CLIENT_NODE_ALREADY_ADDED    	// Node is already added
- [ ] RPC_CLIENT_NODE_NOT_ADDED        	// Node has not been added before
- [ ] RPC_CLIENT_NODE_NOT_CONNECTED    	// Node to disconnect not found in connected nodes
- [ ] RPC_CLIENT_INVALID_IP_OR_SUBNET  	// Invalid IP/Subnet
- [ ] RPC_CLIENT_P2P_DISABLED          	// No valid connection manager instance found
- [ ] RPC_CLIENT_NODE_CAPACITY_REACHED 	// Max number of outbound or block-relay connections already open

### Chain errors
- [ ] RPC_CLIENT_MEMPOOL_DISABLED	// No mempool instance found

### Wallet errors
- [ ] RPC_WALLET_ERROR					// Unspecified problem with wallet (key not found etc.)
- [ ] RPC_WALLET_INSUFFICIENT_FUNDS		// Not enough funds in wallet or account
- [ ] RPC_WALLET_INVALID_LABEL_NAME		// Invalid label name
- [ ] RPC_WALLET_KEYPOOL_RAN_OUT      	// Keypool ran out, call keypoolrefill first
- [ ] RPC_WALLET_UNLOCK_NEEDED        	// Enter the wallet passphrase with walletpassphrase first
- [ ] RPC_WALLET_PASSPHRASE_INCORRECT 	// The wallet passphrase entered was incorrect
- [ ] RPC_WALLET_WRONG_ENC_STATE      	// Command given in wrong wallet encryption state (encrypting an encrypted wallet etc.)
- [ ] RPC_WALLET_ENCRYPTION_FAILED    	// Failed to encrypt the wallet
- [ ] RPC_WALLET_ALREADY_UNLOCKED     	// Wallet is already unlocked
- [ ] RPC_WALLET_NOT_FOUND            	// Invalid wallet specified
- [ ] RPC_WALLET_NOT_SPECIFIED        	// No wallet specified (error when there are multiple wallets loaded)
- [ ] RPC_WALLET_ALREADY_LOADED       	// This same wallet is already loaded
- [ ] RPC_WALLET_ALREADY_EXISTS       	// There is already a wallet with the same name

### Unknown Error
- [x] RPC_UNKOWN_ERROR // Error code is not defined by Bitcoin core

## How to contribute
If you encounter an error that is not yet supported by this library, you can easily add support for it yourself! Here's how you can contribute:

1. Visit the Bitcoin Core GitHub page to find the desired error.
2. Search for all occurrences of the error (e.g., search for RPC_DATABASE_ERROR).
3. Create a new enum to represent the error (e.g., DatabaseError).
4. Implement the From<RpcError> trait for the new enum by following the existing implementations as a reference.
5. For each occurrence of the DatabaseError, create a variant inside the enum:
	1. Create a parser to extract relevant information from the error message.
	2. Add the necessary match_and_extract logic to the from_str function.
6. Submit a pull request (PR) to contribute your changes to the library!

By following these steps, you can extend the functionality of the library to handle new errors that are currently not supported. Your contributions are greatly appreciated!
