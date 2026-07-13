// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../src/AssetOwnership.sol";
import "../src/AuditTrail.sol";
import "../src/AssetOwnershipBatch.sol";

/**
 * @title VerifyDeployment
 * @dev Script to verify deployed contracts are configured correctly
 */
contract VerifyDeployment is Script {
    function run() external view {
        address assetOwnershipAddr = vm.envAddress("ASSET_OWNERSHIP_ADDRESS");
        address auditTrailAddr = vm.envAddress("AUDIT_TRAIL_ADDRESS");
        address assetOwnershipBatchAddr = vm.envAddress("ASSET_OWNERSHIP_BATCH_ADDRESS");
        address admin = vm.envAddress("ADMIN_ADDRESS");
        address minter = vm.envAddress("MINTER_ADDRESS");
        address recorder = vm.envAddress("RECORDER_ADDRESS");

        console.log("=== Verifying Deployment ===\n");

        // Verify AssetOwnership
        AssetOwnership assetOwnership = AssetOwnership(assetOwnershipAddr);
        require(assetOwnership.hasRole(assetOwnership.DEFAULT_ADMIN_ROLE(), admin), "Admin role not set for AssetOwnership");
        require(assetOwnership.hasRole(assetOwnership.MINTER_ROLE(), minter), "Minter role not set for AssetOwnership");
        console.log("[OK] AssetOwnership roles verified");

        // Verify AuditTrail
        AuditTrail auditTrail = AuditTrail(auditTrailAddr);
        require(auditTrail.hasRole(auditTrail.DEFAULT_ADMIN_ROLE(), admin), "Admin role not set for AuditTrail");
        require(auditTrail.hasRole(auditTrail.RECORDER_ROLE(), recorder), "Recorder role not set for AuditTrail");
        console.log("[OK] AuditTrail roles verified");

        // Verify AssetOwnershipBatch
        AssetOwnershipBatch assetOwnershipBatch = AssetOwnershipBatch(assetOwnershipBatchAddr);
        require(assetOwnershipBatch.hasRole(assetOwnershipBatch.DEFAULT_ADMIN_ROLE(), admin), "Admin role not set for AssetOwnershipBatch");
        require(assetOwnershipBatch.hasRole(assetOwnershipBatch.MINTER_ROLE(), minter), "Minter role not set for AssetOwnershipBatch");
        console.log("[OK] AssetOwnershipBatch roles verified");

        console.log("\n=== All Verifications Passed ===");
    }
}
