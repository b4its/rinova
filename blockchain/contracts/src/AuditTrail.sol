// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

/**
 * @title AuditTrail
 * @dev Contract for recording publish events and maintaining immutable audit trail
 * Requirements: 9.2, 9.3, 9.5
 */
contract AuditTrail is AccessControl {
    using EnumerableSet for EnumerableSet.Bytes32Set;

    bytes32 public constant RECORDER_ROLE = keccak256("RECORDER_ROLE");

    // Mapping from project ID to publish records
    mapping(string => PublishRecord[]) private _projectPublishHistory;

    // Mapping from project ID to hash set for quick verification
    mapping(string => EnumerableSet.Bytes32Set) private _projectHashes;

    // Total publish count
    uint256 public totalPublishCount;

    // Record ID counter
    uint256 private _recordIdCounter;

    struct PublishRecord {
        uint256 recordId;           // Unique record identifier
        string projectId;           // Project UUID
        bytes32 contentHash;        // SHA-256 hash of website bundle
        string ipfsHash;            // IPFS reference for distributed storage
        address publisher;          // Address that triggered publish
        uint256 timestamp;          // Block timestamp
        string version;             // Version string (semver)
        uint256 blockSize;          // Size of the published bundle in bytes
    }

    // Events for off-chain indexing
    event PublishRecorded(
        uint256 indexed recordId,
        string indexed projectId,
        bytes32 contentHash,
        string ipfsHash,
        address indexed publisher,
        uint256 timestamp
    );

    event HashVerified(
        string indexed projectId,
        bytes32 contentHash,
        bool isValid
    );

    constructor(address admin, address recorder) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(RECORDER_ROLE, recorder);
    }

    /**
     * @dev Record a publish event
     * @param projectId Project UUID
     * @param contentHash SHA-256 hash of the website bundle
     * @param ipfsHash IPFS reference for the content
     * @param version Version string
     * @param blockSize Size of the published bundle
     * Requirements: 9.2, 9.3
     */
    function recordPublish(
        string calldata projectId,
        bytes32 contentHash,
        string calldata ipfsHash,
        string calldata version,
        uint256 blockSize
    ) external onlyRole(RECORDER_ROLE) returns (uint256) {
        require(bytes(projectId).length > 0, "Project ID required");
        require(contentHash != bytes32(0), "Content hash required");
        require(bytes(ipfsHash).length > 0, "IPFS hash required");

        uint256 recordId = _recordIdCounter++;
        totalPublishCount++;

        PublishRecord memory record = PublishRecord({
            recordId: recordId,
            projectId: projectId,
            contentHash: contentHash,
            ipfsHash: ipfsHash,
            publisher: msg.sender,
            timestamp: block.timestamp,
            version: version,
            blockSize: blockSize
        });

        _projectPublishHistory[projectId].push(record);
        _projectHashes[projectId].add(contentHash);

        emit PublishRecorded(recordId, projectId, contentHash, ipfsHash, msg.sender, block.timestamp);

        return recordId;
    }

    /**
     * @dev Verify if a content hash exists for a project
     * @param projectId Project UUID
     * @param contentHash Hash to verify
     * @return isValid Whether the hash is valid
     * @return timestamp When the hash was recorded (0 if not found)
     * Requirements: 9.6, 9.7
     */
    function verifyHash(string calldata projectId, bytes32 contentHash) 
        external 
        view 
        returns (bool isValid, uint256 timestamp) 
    {
        if (!_projectHashes[projectId].contains(contentHash)) {
            return (false, 0);
        }

        PublishRecord[] storage history = _projectPublishHistory[projectId];
        for (uint256 i = 0; i < history.length; i++) {
            if (history[i].contentHash == contentHash) {
                return (true, history[i].timestamp);
            }
        }

        return (false, 0);
    }

    /**
     * @dev Get publish history for a project
     * @param projectId Project UUID
     * @return records Array of publish records
     */
    function getPublishHistory(string calldata projectId) 
        external 
        view 
        returns (PublishRecord[] memory) 
    {
        return _projectPublishHistory[projectId];
    }

    /**
     * @dev Get latest publish record for a project
     * @param projectId Project UUID
     * @return record Latest publish record
     */
    function getLatestPublish(string calldata projectId) 
        external 
        view 
        returns (PublishRecord memory) 
    {
        PublishRecord[] storage history = _projectPublishHistory[projectId];
        require(history.length > 0, "No publish history");
        return history[history.length - 1];
    }

    /**
     * @dev Get publish count for a project
     * @param projectId Project UUID
     * @return count Number of publishes
     */
    function getPublishCount(string calldata projectId) 
        external 
        view 
        returns (uint256) 
    {
        return _projectPublishHistory[projectId].length;
    }

    /**
     * @dev Get a specific publish record
     * @param projectId Project UUID
     * @param index Index in the history array
     * @return record Publish record at the index
     */
    function getPublishByIndex(string calldata projectId, uint256 index) 
        external 
        view 
        returns (PublishRecord memory) 
    {
        PublishRecord[] storage history = _projectPublishHistory[projectId];
        require(index < history.length, "Index out of bounds");
        return history[index];
    }

    /**
     * @dev Check if content hash exists for a project
     * @param projectId Project UUID
     * @param contentHash Hash to check
     * @return exists Whether the hash exists
     */
    function hashExists(string calldata projectId, bytes32 contentHash) 
        external 
        view 
        returns (bool) 
    {
        return _projectHashes[projectId].contains(contentHash);
    }

    /**
     * @dev Get all content hashes for a project
     * @param projectId Project UUID
     * @return hashes Array of content hashes
     */
    function getAllHashes(string calldata projectId) 
        external 
        view 
        returns (bytes32[] memory) 
    {
        EnumerableSet.Bytes32Set storage hashSet = _projectHashes[projectId];
        bytes32[] memory hashes = new bytes32[](hashSet.length());
        
        for (uint256 i = 0; i < hashSet.length(); i++) {
            hashes[i] = hashSet.at(i);
        }
        
        return hashes;
    }

    /**
     * @dev Compare two hashes and return if they match
     * @param projectId Project UUID
     * @param expectedHash Expected content hash
     * @param actualHash Actual content hash
     * @return matches Whether the hashes match
     * @return warning Warning message if mismatch
     */
    function compareHashes(
        string calldata projectId,
        bytes32 expectedHash,
        bytes32 actualHash
    ) external view returns (bool matches, string memory warning) {
        bool expectedExists = _projectHashes[projectId].contains(expectedHash);
        bool actualExists = _projectHashes[projectId].contains(actualHash);

        if (expectedHash == actualHash) {
            return (true, "");
        } else if (!expectedExists) {
            return (false, "Expected hash not found in history");
        } else if (!actualExists) {
            return (false, "Actual hash not found in history");
        } else {
            return (false, "Hash mismatch detected - content may have been modified");
        }
    }
}
