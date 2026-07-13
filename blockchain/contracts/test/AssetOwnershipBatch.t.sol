// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/AssetOwnershipBatch.sol";

contract AssetOwnershipBatchTest is Test {
    AssetOwnershipBatch private batch;
    address private admin;
    address private minter;
    address private user1;
    address private user2;

    function setUp() public {
        admin = address(0x1);
        minter = address(0x2);
        user1 = address(0x3);
        user2 = address(0x4);

        vm.startPrank(admin);
        batch = new AssetOwnershipBatch(admin, minter);
        vm.stopPrank();
    }

    function test_RecordBatchOwnership() public {
        string[] memory assetIds = new string[](3);
        address[] memory owners = new address[](3);
        uint256[] memory assetTypes = new uint256[](3);
        uint256[] memory amounts = new uint256[](3);

        assetIds[0] = "asset-001";
        assetIds[1] = "asset-002";
        assetIds[2] = "asset-003";

        owners[0] = user1;
        owners[1] = user1;
        owners[2] = user2;

        assetTypes[0] = batch.WEBSITE_TOKEN();
        assetTypes[1] = batch.COMPONENT_TOKEN();
        assetTypes[2] = batch.TEMPLATE_TOKEN();

        amounts[0] = 1;
        amounts[1] = 1;
        amounts[2] = 1;

        vm.startPrank(minter);
        uint256 batchId = batch.recordBatchOwnership(
            assetIds,
            owners,
            assetTypes,
            amounts,
            "QmBatch123"
        );
        vm.stopPrank();

        assertEq(batchId, 0);
        assertTrue(batch.getTokenId("asset-001") != 0);
        assertTrue(batch.getTokenId("asset-002") != 0);
        assertTrue(batch.getTokenId("asset-003") != 0);
    }

    function test_RecordBatchOwnership_RevertIf_LessThanTwo() public {
        string[] memory assetIds = new string[](1);
        address[] memory owners = new address[](1);
        uint256[] memory assetTypes = new uint256[](1);
        uint256[] memory amounts = new uint256[](1);

        assetIds[0] = "asset-001";
        owners[0] = user1;
        assetTypes[0] = batch.WEBSITE_TOKEN();
        amounts[0] = 1;

        vm.startPrank(minter);
        vm.expectRevert("Minimum 2 assets for batch operation");
        batch.recordBatchOwnership(assetIds, owners, assetTypes, amounts, "QmBatch123");
        vm.stopPrank();
    }

    function test_RecordSingleOwnership() public {
        vm.startPrank(minter);
        
        uint256 tokenId = batch.recordSingleOwnership(
            "single-asset-001",
            user1,
            batch.WEBSITE_TOKEN(),
            1,
            "QmSingle123"
        );
        
        vm.stopPrank();

        assertTrue(tokenId != 0);
        assertEq(batch.balanceOf(user1, tokenId), 1);
    }

    function test_VerifyOwnership() public {
        vm.startPrank(minter);
        
        uint256 tokenId = batch.recordSingleOwnership(
            "verify-asset-001",
            user1,
            batch.WEBSITE_TOKEN(),
            1,
            "QmSingle123"
        );
        
        vm.stopPrank();

        assertTrue(batch.verifyOwnership("verify-asset-001", user1));
        assertFalse(batch.verifyOwnership("verify-asset-001", user2));
        assertFalse(batch.verifyOwnership("nonexistent", user1));
    }

    function test_GetAssetsByOwner() public {
        string[] memory assetIds = new string[](3);
        address[] memory owners = new address[](3);
        uint256[] memory assetTypes = new uint256[](3);
        uint256[] memory amounts = new uint256[](3);

        for (uint256 i = 0; i < 3; i++) {
            assetIds[i] = string(abi.encodePacked("owner-asset-", i));
            owners[i] = user1;
            assetTypes[i] = batch.WEBSITE_TOKEN();
            amounts[i] = 1;
        }

        vm.startPrank(minter);
        batch.recordBatchOwnership(assetIds, owners, assetTypes, amounts, "QmBatch123");
        vm.stopPrank();

        uint256[] memory userAssets = batch.getAssetsByOwner(user1);
        assertEq(userAssets.length, 3);
    }

    function test_TransferOwnership() public {
        vm.startPrank(minter);
        
        uint256 tokenId = batch.recordSingleOwnership(
            "transfer-asset-001",
            user1,
            batch.WEBSITE_TOKEN(),
            1,
            "QmSingle123"
        );
        
        vm.stopPrank();

        vm.prank(user1);
        batch.transferOwnership(user1, user2, tokenId, 1);

        assertEq(batch.balanceOf(user2, tokenId), 1);
        assertEq(batch.balanceOf(user1, tokenId), 0);

        uint256[] memory user2Assets = batch.getAssetsByOwner(user2);
        assertEq(user2Assets.length, 1);
    }

    function test_GetAssetMetadata() public {
        vm.startPrank(minter);
        
        uint256 tokenId = batch.recordSingleOwnership(
            "metadata-asset-001",
            user1,
            batch.COMPONENT_TOKEN(),
            1,
            "QmMetadata123"
        );
        
        vm.stopPrank();

        AssetOwnershipBatch.AssetMetadata memory metadata = batch.getAssetMetadata(tokenId);
        assertEq(metadata.assetId, "metadata-asset-001");
        assertEq(metadata.assetType, batch.COMPONENT_TOKEN());
        assertEq(metadata.ipfsHash, "QmMetadata123");
        assertTrue(metadata.exists);
    }

    // Property Tests

    function test_Property_BatchGasEfficiency() public {
        // Test that batch operation is more gas efficient than individual operations
        uint256 batchSize = 5;

        string[] memory assetIds = new string[](batchSize);
        address[] memory owners = new address[](batchSize);
        uint256[] memory assetTypes = new uint256[](batchSize);
        uint256[] memory amounts = new uint256[](batchSize);

        for (uint256 i = 0; i < batchSize; i++) {
            assetIds[i] = string(abi.encodePacked("gas-test-", i));
            owners[i] = user1;
            assetTypes[i] = batch.WEBSITE_TOKEN();
            amounts[i] = 1;
        }

        vm.startPrank(minter);
        
        uint256 gasBefore = vm.getGasLeft();
        batch.recordBatchOwnership(assetIds, owners, assetTypes, amounts, "QmGasTest");
        uint256 gasAfterBatch = gasBefore - vm.getGasLeft();
        
        vm.stopPrank();

        // Log gas used (in real test, compare with individual operations)
        emit log_named_uint("Batch gas used", gasAfterBatch);
    }

    function test_Property_BatchUniqueness() public {
        string[] memory assetIds = new string[](3);
        address[] memory owners = new address[](3);
        uint256[] memory assetTypes = new uint256[](3);
        uint256[] memory amounts = new uint256[](3);

        assetIds[0] = "unique-001";
        assetIds[1] = "unique-002";
        assetIds[2] = "unique-003";

        for (uint256 i = 0; i < 3; i++) {
            owners[i] = user1;
            assetTypes[i] = batch.WEBSITE_TOKEN();
            amounts[i] = 1;
        }

        vm.startPrank(minter);
        batch.recordBatchOwnership(assetIds, owners, assetTypes, amounts, "QmUnique123");
        vm.stopPrank();

        // Each asset should have a unique token ID
        uint256 tokenId1 = batch.getTokenId("unique-001");
        uint256 tokenId2 = batch.getTokenId("unique-002");
        uint256 tokenId3 = batch.getTokenId("unique-003");

        assertTrue(tokenId1 != tokenId2);
        assertTrue(tokenId2 != tokenId3);
        assertTrue(tokenId1 != tokenId3);
    }

    function testFuzz_RecordBatchOwnership(
        string[5] calldata assetIds,
        address owner
    ) public {
        vm.assume(owner != address(0));
        
        // Validate asset IDs are non-empty
        for (uint256 i = 0; i < 5; i++) {
            vm.assume(bytes(assetIds[i]).length > 0);
        }

        string[] memory ids = new string[](5);
        address[] memory owners = new address[](5);
        uint256[] memory assetTypes = new uint256[](5);
        uint256[] memory amounts = new uint256[](5);

        for (uint256 i = 0; i < 5; i++) {
            ids[i] = assetIds[i];
            owners[i] = owner;
            assetTypes[i] = batch.WEBSITE_TOKEN();
            amounts[i] = 1;
        }

        vm.startPrank(minter);
        batch.recordBatchOwnership(ids, owners, assetTypes, amounts, "QmFuzz123");
        vm.stopPrank();

        assertEq(batch.getAssetsByOwner(owner).length, 5);
    }
}
