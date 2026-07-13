// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../src/AuditTrail.sol";

contract AuditTrailTest is Test {
    AuditTrail private auditTrail;
    address private admin;
    address private recorder;
    address private user1;

    function setUp() public {
        admin = address(0x1);
        recorder = address(0x2);
        user1 = address(0x3);

        vm.startPrank(admin);
        auditTrail = new AuditTrail(admin, recorder);
        vm.stopPrank();
    }

    function test_RecordPublish() public {
        vm.startPrank(recorder);
        
        bytes32 contentHash = keccak256("test-content");
        uint256 recordId = auditTrail.recordPublish(
            "project-001",
            contentHash,
            "QmTest123",
            "1.0.0",
            1024
        );
        
        vm.stopPrank();

        assertEq(recordId, 0);
        assertEq(auditTrail.totalPublishCount(), 1);
        
        (bool isValid, uint256 timestamp) = auditTrail.verifyHash("project-001", contentHash);
        assertTrue(isValid);
        assertTrue(timestamp > 0);
    }

    function test_RecordMultiplePublishes() public {
        vm.startPrank(recorder);
        
        auditTrail.recordPublish(
            "project-001",
            keccak256("content-1"),
            "QmTest123",
            "1.0.0",
            1024
        );

        auditTrail.recordPublish(
            "project-001",
            keccak256("content-2"),
            "QmTest456",
            "1.0.1",
            2048
        );

        auditTrail.recordPublish(
            "project-002",
            keccak256("content-3"),
            "QmTest789",
            "1.0.0",
            512
        );
        
        vm.stopPrank();

        assertEq(auditTrail.totalPublishCount(), 3);
        assertEq(auditTrail.getPublishCount("project-001"), 2);
        assertEq(auditTrail.getPublishCount("project-002"), 1);
    }

    function test_VerifyHash() public {
        bytes32 contentHash = keccak256("test-content");

        vm.startPrank(recorder);
        auditTrail.recordPublish(
            "project-001",
            contentHash,
            "QmTest123",
            "1.0.0",
            1024
        );
        vm.stopPrank();

        (bool isValid, uint256 timestamp) = auditTrail.verifyHash("project-001", contentHash);
        assertTrue(isValid);
        assertTrue(timestamp > 0);

        (bool invalid, ) = auditTrail.verifyHash("project-001", keccak256("wrong-hash"));
        assertFalse(invalid);
    }

    function test_GetLatestPublish() public {
        vm.startPrank(recorder);
        
        auditTrail.recordPublish(
            "project-001",
            keccak256("content-1"),
            "QmTest123",
            "1.0.0",
            1024
        );

        vm.warp(block.timestamp + 3600); // Advance 1 hour

        auditTrail.recordPublish(
            "project-001",
            keccak256("content-2"),
            "QmTest456",
            "1.0.1",
            2048
        );
        
        vm.stopPrank();

        AuditTrail.PublishRecord memory latest = auditTrail.getLatestPublish("project-001");
        assertEq(latest.ipfsHash, "QmTest456");
        assertEq(latest.version, "1.0.1");
    }

    function test_GetPublishHistory() public {
        vm.startPrank(recorder);
        
        auditTrail.recordPublish(
            "project-001",
            keccak256("content-1"),
            "QmTest123",
            "1.0.0",
            1024
        );

        auditTrail.recordPublish(
            "project-001",
            keccak256("content-2"),
            "QmTest456",
            "1.0.1",
            2048
        );
        
        vm.stopPrank();

        AuditTrail.PublishRecord[] memory history = auditTrail.getPublishHistory("project-001");
        assertEq(history.length, 2);
        assertEq(history[0].version, "1.0.0");
        assertEq(history[1].version, "1.0.1");
    }

    function test_HashExists() public {
        bytes32 contentHash = keccak256("test-content");

        vm.startPrank(recorder);
        auditTrail.recordPublish(
            "project-001",
            contentHash,
            "QmTest123",
            "1.0.0",
            1024
        );
        vm.stopPrank();

        assertTrue(auditTrail.hashExists("project-001", contentHash));
        assertFalse(auditTrail.hashExists("project-001", keccak256("wrong-hash")));
    }

    function test_CompareHashes_Match() public {
        bytes32 contentHash = keccak256("test-content");

        vm.startPrank(recorder);
        auditTrail.recordPublish(
            "project-001",
            contentHash,
            "QmTest123",
            "1.0.0",
            1024
        );
        vm.stopPrank();

        (bool matches, string memory warning) = auditTrail.compareHashes(
            "project-001",
            contentHash,
            contentHash
        );

        assertTrue(matches);
        assertEq(warning, "");
    }

    function test_CompareHashes_Mismatch() public {
        bytes32 hash1 = keccak256("content-1");
        bytes32 hash2 = keccak256("content-2");

        vm.startPrank(recorder);
        auditTrail.recordPublish("project-001", hash1, "QmTest123", "1.0.0", 1024);
        auditTrail.recordPublish("project-001", hash2, "QmTest456", "1.0.1", 2048);
        vm.stopPrank();

        (bool matches, string memory warning) = auditTrail.compareHashes(
            "project-001",
            hash1,
            hash2
        );

        assertFalse(matches);
        assertTrue(bytes(warning).length > 0);
    }

    function test_RecordPublish_RevertIf_NotRecorder() public {
        vm.startPrank(user1);
        
        vm.expectRevert();
        auditTrail.recordPublish(
            "project-001",
            keccak256("test-content"),
            "QmTest123",
            "1.0.0",
            1024
        );
        
        vm.stopPrank();
    }

    function test_GetAllHashes() public {
        vm.startPrank(recorder);
        
        bytes32 hash1 = keccak256("content-1");
        bytes32 hash2 = keccak256("content-2");
        bytes32 hash3 = keccak256("content-3");

        auditTrail.recordPublish("project-001", hash1, "QmTest123", "1.0.0", 1024);
        auditTrail.recordPublish("project-001", hash2, "QmTest456", "1.0.1", 2048);
        auditTrail.recordPublish("project-001", hash3, "QmTest789", "1.0.2", 512);
        
        vm.stopPrank();

        bytes32[] memory hashes = auditTrail.getAllHashes("project-001");
        assertEq(hashes.length, 3);
    }

    // Property Tests

    function test_Property_HashImmutability() public {
        bytes32 contentHash = keccak256("immutable-content");

        vm.startPrank(recorder);
        auditTrail.recordPublish(
            "project-001",
            contentHash,
            "QmTest123",
            "1.0.0",
            1024
        );
        vm.stopPrank();

        // Hash should still exist after multiple blocks
        vm.warp(block.timestamp + 86400); // 1 day later
        vm.roll(block.number + 1000);

        assertTrue(auditTrail.hashExists("project-001", contentHash));
        
        (bool isValid, ) = auditTrail.verifyHash("project-001", contentHash);
        assertTrue(isValid);
    }

    function test_Property_PublishCountAccuracy() public {
        uint256 expectedCount = 0;

        vm.startPrank(recorder);
        
        for (uint256 i = 0; i < 5; i++) {
            for (uint256 j = 0; j < 3; j++) {
                string memory projectId = string(abi.encodePacked("project-", i));
                auditTrail.recordPublish(
                    projectId,
                    keccak256(abi.encodePacked("content-", i, "-", j)),
                    "QmTest123",
                    "1.0.0",
                    1024
                );
                expectedCount++;
            }
        }
        
        vm.stopPrank();

        assertEq(auditTrail.totalPublishCount(), expectedCount);

        for (uint256 i = 0; i < 5; i++) {
            string memory projectId = string(abi.encodePacked("project-", i));
            assertEq(auditTrail.getPublishCount(projectId), 3);
        }
    }
}
