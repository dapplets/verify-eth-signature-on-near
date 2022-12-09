// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

contract EIP712 {

    struct LinkingAccount {
        string origin_id;
        string account_id;
    }

    struct LinkingAccounts {
        LinkingAccount account_a;
        LinkingAccount account_b;
    }

    // It's not a mistake! https://github.com/ethereum/EIPs/issues/5447#issuecomment-1214827320
    string private constant LINKING_ACCOUNTS_TYPE = "LinkingAccounts(LinkingAccount account_a,LinkingAccount account_b)LinkingAccount(string origin_id,string account_id)";
    string private constant LINKING_ACCOUNT_TYPE = "LinkingAccount(string origin_id,string account_id)";
    bytes32 constant LINKING_ACCOUNTS_TYPEHASH = keccak256(bytes(LINKING_ACCOUNTS_TYPE));
    bytes32 constant LINKING_ACCOUNT_TYPEHASH = keccak256(bytes(LINKING_ACCOUNT_TYPE));

    uint256 constant chainId = 5;
    address constant verifyingContract = 0x0000000000000000000000000000000000000000;
    string private constant EIP712_DOMAIN = "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)";
    bytes32 constant EIP712_DOMAIN_TYPEHASH = keccak256(bytes(EIP712_DOMAIN));
    bytes32 private constant DOMAIN_SEPARATOR = keccak256(abi.encode(
        EIP712_DOMAIN_TYPEHASH,
        keccak256(bytes("Connected Accounts")),
        keccak256(bytes("1")),
        chainId,
        verifyingContract
    ));

    function hashLinkingAccount(LinkingAccount memory linkingAccount) public pure returns (bytes32) {
        return keccak256(abi.encode(
            LINKING_ACCOUNT_TYPEHASH,
            keccak256(bytes(linkingAccount.origin_id)),
            keccak256(bytes(linkingAccount.account_id))
        ));
    }

    function hashLinkingAccounts(LinkingAccounts memory linkingAccounts) public pure returns (bytes32){
        
        return keccak256(abi.encodePacked(
            "\x19\x01",
            DOMAIN_SEPARATOR,
            hashLinkingAccountsNoDomain(linkingAccounts)
        ));
    }

    function hashLinkingAccountsNoDomain(LinkingAccounts memory linkingAccounts) public pure returns (bytes32){
        return keccak256(abi.encode(
            LINKING_ACCOUNTS_TYPEHASH,
            hashLinkingAccount(linkingAccounts.account_a),
            hashLinkingAccount(linkingAccounts.account_b)
        ));
    }

    function domainSeparator() public pure returns (bytes32) {
        return DOMAIN_SEPARATOR;
    }

    function eth_ecrecover(bytes32 message, uint8 v, bytes32 r, bytes32 s) public pure returns (address) {
        return ecrecover(message, v, r, s);
    }

    function eth_ecrecover_2(LinkingAccounts memory linkingAccounts, bytes32 r, bytes32 s,uint8 v) public pure returns (address) {
        bytes32 hash = hashLinkingAccounts(linkingAccounts);
        return ecrecover(hash, v, r, s);
    }
}