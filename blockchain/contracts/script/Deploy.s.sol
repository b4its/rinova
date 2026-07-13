// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/AssetOwnership.sol";
import "../src/AuditTrail.sol";
import "../src/AssetOwnershipBatch.sol";

/**
 * @title DeployContracts
 * @dev Deployment script for Rinova smart contracts
 */
contract DeployContracts is Script {
    // Environment variables
    address private admin;
    address private minter;
    address private recorder;
    
    // Deployed contract addresses
    address public assetOwnershipAddress;
    address public auditTrailAddress;
    address public assetOwnershipBatchAddress;

    function setUp() public {
        admin = vm.envAddress("ADMIN_ADDRESS");
        minter = vm.envAddress("MINTER_ADDRESS");
        recorder = vm.envAddress("RECORDER_ADDRESS");
    }

    function run() external {
        vm.startBroadcast();

        // Deploy AssetOwnership (ERC-721)
        AssetOwnership assetOwnership = new AssetOwnership(admin, minter);
        assetOwnershipAddress = address(assetOwnership);
        console.log("AssetOwnership deployed at:", assetOwnershipAddress);

        // Deploy AuditTrail
        AuditTrail auditTrail = new AuditTrail(admin, recorder);
        auditTrailAddress = address(auditTrail);
        console.log("AuditTrail deployed at:", auditTrailAddress);

        // Deploy AssetOwnershipBatch (ERC-1155)
        AssetOwnershipBatch assetOwnershipBatch = new AssetOwnershipBatch(admin, minter);
        assetOwnershipBatchAddress = address(assetOwnershipBatch);
        console.log("AssetOwnershipBatch deployed at:", assetOwnershipBatchAddress);

        vm.stopBroadcast();

        // Output deployment info
        console.log("\n=== Deployment Summary ===");
        console.log("Admin:", admin);
        console.log("Minter:", minter);
        console.log("Recorder:", recorder);
        console.log("AssetOwnership:", assetOwnershipAddress);
        console.log("AuditTrail:", auditTrailAddress);
        console.log("AssetOwnershipBatch:", assetOwnershipBatchAddress);
    }
}
