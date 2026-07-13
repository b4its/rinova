// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/AssetOwnership.sol";

contract AssetOwnershipTest is Test {
    AssetOwnership private ownership;
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
        ownership = new AssetOwnership(admin, minter);
        vm.stopPrank();
    }

    function test_RecordOwnership() public {
        vm.startPrank(minter);
        
        uint256 tokenId = ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project",
            "QmTest123"
        );
        
        vm.stopPrank();

        assertEq(tokenId, 0);
        assertEq(ownership.ownerOf(tokenId), user1);
        
        (
            string memory assetId,
            string memory assetType,
            string memory projectName,
            uint256 createdAt,
            uint256 updatedAt,
            string memory ipfsHash,
            bool isTransferred
        ) = ownership.getOwnershipMetadata(tokenId);

        assertEq(assetId, "asset-uuid-001");
        assertEq(assetType, "website");
        assertEq(projectName, "Test Project");
        assertTrue(createdAt > 0);
        assertTrue(updatedAt > 0);
        assertEq(ipfsHash, "QmTest123");
        assertFalse(isTransferred);
    }

    function test_RecordOwnership_RevertIf_AlreadyRegistered() public {
        vm.startPrank(minter);
        
        ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project",
            "QmTest123"
        );

        vm.expectRevert("Asset already registered");
        ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project 2",
            "QmTest456"
        );
        
        vm.stopPrank();
    }

    function test_RecordOwnership_RevertIf_NotMinter() public {
        vm.startPrank(user1);
        
        vm.expectRevert();
        ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project",
            "QmTest123"
        );
        
        vm.stopPrank();
    }

    function test_GetTokenByAssetId() public {
        vm.startPrank(minter);
        
        uint256 tokenId = ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project",
            "QmTest123"
        );
        
        vm.stopPrank();

        assertEq(ownership.getTokenByAssetId("asset-uuid-001"), tokenId);
        assertEq(ownership.getTokenByAssetId("nonexistent"), 0);
    }

    function test_VerifyOwnership() public {
        vm.startPrank(minter);
        
        ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project",
            "QmTest123"
        );
        
        vm.stopPrank();

        assertTrue(ownership.verifyOwnership("asset-uuid-001", user1));
        assertFalse(ownership.verifyOwnership("asset-uuid-001", user2));
        assertFalse(ownership.verifyOwnership("nonexistent", user1));
    }

    function test_GetAssetCountByOwner() public {
        vm.startPrank(minter);
        
        ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project 1",
            "QmTest123"
        );
        
        ownership.recordOwnership(
            user1,
            "asset-uuid-002",
            "component",
            "Test Project 2",
            "QmTest456"
        );
        
        vm.stopPrank();

        assertEq(ownership.getAssetCountByOwner(user1), 2);
        assertEq(ownership.getAssetCountByOwner(user2), 0);
    }

    function test_UpdateMetadata() public {
        vm.startPrank(minter);
        
        uint256 tokenId = ownership.recordOwnership(
            user1,
            "asset-uuid-001",
            "website",
            "Test Project",
            "QmTest123"
        );

        ownership.updateMetadata(tokenId, "QmNewHash");
        
        vm.stopPrank();

        (
            ,,,,,,
            string memory ipfsHash,
        ) = ownership.getOwnershipMetadata(tokenId);

        assertEq(ipfsHash, "QmNewHash");
    }

    // Property Tests

    function testFuzz_RecordOwnership(
        address to,
        string calldata assetId,
        string calldata assetType,
        string calldata projectName,
        string calldata ipfsHash
    ) public {
        vm.assume(to != address(0));
        vm.assume(bytes(assetId).length > 0 && bytes(assetId).length <= 100);
        vm.assume(bytes(assetType).length > 0);
        vm.assume(keccak256(bytes(assetId)) != keccak256(bytes("")));

        vm.startPrank(minter);
        
        uint256 tokenId = ownership.recordOwnership(to, assetId, assetType, projectName, ipfsHash);
        
        vm.stopPrank();

        assertEq(ownership.ownerOf(tokenId), to);
        assertEq(ownership.getTokenByAssetId(assetId), tokenId);
    }

    function test_Property_MultipleAssetsSameOwner() public {
        vm.startPrank(minter);
        
        for (uint256 i = 0; i < 10; i++) {
            string memory assetId = string(abi.encodePacked("asset-", i));
            ownership.recordOwnership(
                user1,
                assetId,
                "website",
                "Test Project",
                "QmTest123"
            );
        }
        
        vm.stopPrank();

        assertEq(ownership.getAssetCountByOwner(user1), 10);
    }

    function test_Property_AssetIdUniqueness() public {
        vm.startPrank(minter);
        
        uint256 tokenId1 = ownership.recordOwnership(
            user1,
            "unique-asset-001",
            "website",
            "Test Project 1",
            "QmTest123"
        );

        uint256 tokenId2 = ownership.recordOwnership(
            user1,
            "unique-asset-002",
            "website",
            "Test Project 2",
            "QmTest456"
        );
        
        vm.stopPrank();

        assertTrue(tokenId1 != tokenId2);
        assertEq(ownership.getTokenByAssetId("unique-asset-001"), tokenId1);
        assertEq(ownership.getTokenByAssetId("unique-asset-002"), tokenId2);
    }
}
