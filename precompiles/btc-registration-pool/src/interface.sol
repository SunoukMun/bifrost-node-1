// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.0;

/**
 * @title The interface through which solidity contracts will interact with Btc Registration Pool
 * We follow this same interface including four-byte function selectors, in the precompile that
 * wraps the pallet
 * Address :    0x0000000000000000000000000000000000000100
 */

interface BtcRegistrationPool {
    /// @dev A new user registered to the pool and requested a vault address.
    /// @custom:selector
    /// @param user_bfc_address The registered Bifrost address.
    /// @param refund_address The registered refund address.
    event VaultPending(address user_bfc_address, string refund_address);

    /// @dev A pending vault address has been generated.
    /// @custom:selector
    /// @param user_bfc_address The registered Bifrost address.
    /// @param refund_address The registered refund address.
    /// @param vault_address The generated vault address.
    event VaultGenerated(
        address user_bfc_address,
        string refund_address,
        string vault_address
    );

    /// @dev Returns the current registration pool
    /// @custom:selector e9db6a30
    /// @return The list of the current registration pool (0: Bifrost addresses, 1: refund addresses, 2: vault addresses)
    function registration_pool()
        external
        view
        returns (address[] memory, string[] memory, string[] memory);

    /// @dev Returns the current bonded vault addresses
    /// @custom:selector 557bca49
    /// @return The list of the current bonded vault addresses
    function vault_addresses() external view returns (string[] memory);

    /// @dev Returns the current bonded refund addresses
    /// @custom:selector 135ca504
    /// @return The list of the current bonded refund addresses
    function refund_addresses() external view returns (string[] memory);

    /// @dev Returns the bonded vault address mapped to the Bifrost address
    /// @custom:selector d2534116
    /// @param user_bfc_address the address that we want to check
    /// @return A Bitcoin vault address
    function vault_address(
        address user_bfc_address
    ) external view returns (string memory);

    /// @dev Returns the bonded refund address mapped to the Bifrost address
    /// @custom:selector 6dcd31db
    /// @param user_bfc_address the address that we want to check
    /// @return A Bitcoin refund address
    function refund_address(
        address user_bfc_address
    ) external view returns (string memory);

    /// @dev Join the registration pool and request a Bitcoin vault address.
    /// @custom:selector
    /// @param refund_address The Bitcoin refund address
    function request_vault(string memory refund_address) external;
}
