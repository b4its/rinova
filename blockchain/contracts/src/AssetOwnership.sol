// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @title AssetOwnership
 * @dev ERC-721 contract for recording website/component ownership on blockchain
 * Requirements: 8.1, 8.2, 8.4, 8.5
 */
contract AssetOwnership is ERC721, ERC721URIStorage, AccessControl {
    using Counters for Counters.Counter;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");
    bytes32 public constant RECORDER_ROLE = keccak256("RECORDER_ROLE");

    Counters.Counter private _tokenIdCounter;

    // Mapping from token ID to ownership metadata
    mapping(uint256 => OwnershipMetadata) private _ownershipMetadata;

    // Mapping from asset ID (external) to token ID
    mapping(string => uint256) private _assetToToken;

    // Mapping from owner address to their asset count
    mapping(address => uint256) private _ownerAssetCount;

    struct OwnershipMetadata {
        string assetId;           // External asset identifier (UUID)
        string assetType;         // "website", "component", "template"
        string projectName;       // Name of the project
        uint256 createdAt;        // Timestamp of creation
        uint256 updatedAt;        // Timestamp of last update
        string ipfsHash;          // IPFS reference for asset data
        bool isTransferred;       // Whether ownership has been transferred
    }

    // Events for off-chain indexing
    event OwnershipRecorded(
        uint256 indexed tokenId,
        string assetId,
        address indexed owner,
        string assetType,
        string projectName,
        uint256 timestamp
    );

    event OwnershipTransferred(
        uint256 indexed tokenId,
        address indexed from,
        address indexed to,
        uint256 timestamp
    );

    event MetadataUpdated(
        uint256 indexed tokenId,
        string ipfsHash,
        uint256 timestamp
    );

    constructor(address defaultAdmin, address minter) ERC721("Rinova Asset Ownership", "RAO") {
        _grantRole(DEFAULT_ADMIN_ROLE, defaultAdmin);
        _grantRole(MINTER_ROLE, minter);
        _grantRole(RECORDER_ROLE, minter);
    }

    /**
     * @dev Record ownership of a new asset
     * @param to Address that will own the asset
     * @param assetId External asset identifier (UUID)
     * @param assetType Type of asset (website, component, template)
     * @param projectName Name of the project
     * @param ipfsHash IPFS reference for asset data
     * @return tokenId The ID of the minted token
     * Requirements: 8.1, 8.2
     */
    function recordOwnership(
        address to,
        string calldata assetId,
        string calldata assetType,
        string calldata projectName,
        string calldata ipfsHash
    ) external onlyRole(MINTER_ROLE) returns (uint256) {
        require(_assetToToken[assetId] == 0, "Asset already registered");
        require(bytes(assetId).length > 0, "Asset ID required");
        require(to != address(0), "Invalid owner address");

        uint256 tokenId = _tokenIdCounter.current();
        _tokenIdCounter.increment();

        _safeMint(to, tokenId);
        _setTokenURI(tokenId, ipfsHash);

        _ownershipMetadata[tokenId] = OwnershipMetadata({
            assetId: assetId,
            assetType: assetType,
            projectName: projectName,
            createdAt: block.timestamp,
            updatedAt: block.timestamp,
            ipfsHash: ipfsHash,
            isTransferred: false
        });

        _assetToToken[assetId] = tokenId;
        _ownerAssetCount[to]++;

        emit OwnershipRecorded(tokenId, assetId, to, assetType, projectName, block.timestamp);

        return tokenId;
    }

    /**
     * @dev Transfer ownership with signature verification
     * @param from Current owner address
     * @param to New owner address
     * @param tokenId Token ID to transfer
     * @param signature EIP-712 signature from current owner
     * Requirements: 8.4, 8.5
     */
    function transferOwnershipWithSignature(
        address from,
        address to,
        uint256 tokenId,
        bytes calldata signature
    ) external {
        require(ownerOf(tokenId) == from, "Not token owner");
        require(to != address(0), "Invalid recipient");
        require(!_ownershipMetadata[tokenId].isTransferred, "Already transferred");

        // Verify signature (simplified - in production use EIP-712)
        bytes32 message = keccak256(abi.encodePacked(from, to, tokenId, block.timestamp));
        address signer = _recoverSigner(message, signature);
        require(signer == from, "Invalid signature");

        _transfer(from, to, tokenId);
        _ownershipMetadata[tokenId].isTransferred = true;
        _ownershipMetadata[tokenId].updatedAt = block.timestamp;
        _ownerAssetCount[from]--;
        _ownerAssetCount[to]++;

        emit OwnershipTransferred(tokenId, from, to, block.timestamp);
    }

    /**
     * @dev Update metadata for an existing asset
     * @param tokenId Token ID to update
     * @param newIpfsHash New IPFS hash for updated content
     */
    function updateMetadata(uint256 tokenId, string calldata newIpfsHash) 
        external 
        onlyRole(RECORDER_ROLE) 
    {
        require(_exists(tokenId), "Token does not exist");
        
        _ownershipMetadata[tokenId].ipfsHash = newIpfsHash;
        _ownershipMetadata[tokenId].updatedAt = block.timestamp;
        _setTokenURI(tokenId, newIpfsHash);

        emit MetadataUpdated(tokenId, newIpfsHash, block.timestamp);
    }

    /**
     * @dev Get ownership metadata for a token
     * @param tokenId Token ID to query
     * @return assetId, assetType, projectName, createdAt, updatedAt, ipfsHash, isTransferred
     */
    function getOwnershipMetadata(uint256 tokenId) 
        external 
        view 
        returns (
            string memory assetId,
            string memory assetType,
            string memory projectName,
            uint256 createdAt,
            uint256 updatedAt,
            string memory ipfsHash,
            bool isTransferred
        ) 
    {
        require(_exists(tokenId), "Token does not exist");
        OwnershipMetadata memory metadata = _ownershipMetadata[tokenId];
        return (
            metadata.assetId,
            metadata.assetType,
            metadata.projectName,
            metadata.createdAt,
            metadata.updatedAt,
            metadata.ipfsHash,
            metadata.isTransferred
        );
    }

    /**
     * @dev Get token ID by external asset ID
     * @param assetId External asset identifier
     * @return tokenId Token ID (0 if not found)
     */
    function getTokenByAssetId(string calldata assetId) external view returns (uint256) {
        return _assetToToken[assetId];
    }

    /**
     * @dev Get asset count for an owner
     * @param owner Owner address
     * @return count Number of assets owned
     */
    function getAssetCountByOwner(address owner) external view returns (uint256) {
        return _ownerAssetCount[owner];
    }

    /**
     * @dev Verify ownership of an asset
     * @param assetId External asset identifier
     * @param claimedOwner Address claiming ownership
     * @return isOwner Whether the address owns the asset
     */
    function verifyOwnership(string calldata assetId, address claimedOwner) 
        external 
        view 
        returns (bool) 
    {
        uint256 tokenId = _assetToToken[assetId];
        if (tokenId == 0) return false;
        return ownerOf(tokenId) == claimedOwner;
    }

    /**
     * @dev Recover signer from signature
     */
    function _recoverSigner(bytes32 message, bytes calldata signature) 
        internal 
        pure 
        returns (address) 
    {
        bytes32 ethSignedMessage = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n32", message)
        );
        return _recoverAddress(ethSignedMessage, signature);
    }

    function _recoverAddress(bytes32 hash, bytes calldata signature) 
        internal 
        pure 
        returns (address) 
    {
        require(signature.length == 65, "Invalid signature length");
        
        bytes32 r;
        bytes32 s;
        uint8 v;
        
        assembly {
            r := calldataload(signature.offset)
            s := calldataload(add(signature.offset, 32))
            v := byte(0, calldataload(add(signature.offset, 64)))
        }
        
        return ecrecover(hash, v, r, s);
    }

    // Required overrides
    function tokenURI(uint256 tokenId) 
        public 
        view 
        override(ERC721, ERC721URIStorage) 
        returns (string memory) 
    {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId) 
        public 
        view 
        override(ERC721, ERC721URIStorage, AccessControl) 
        returns (bool) 
    {
        return super.supportsInterface(interfaceId);
    }
}
