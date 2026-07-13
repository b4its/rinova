// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

/**
 * @title AssetOwnershipBatch
 * @dev ERC-1155 contract for batch ownership recording (gas optimized)
 * Requirements: 8.7
 */
contract AssetOwnershipBatch is ERC1155, AccessControl {
    using Counters for Counters.Counter;

    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    // Token IDs for different asset types
    uint256 public constant WEBSITE_TOKEN = 1;
    uint256 public constant COMPONENT_TOKEN = 2;
    uint256 public constant TEMPLATE_TOKEN = 3;

    Counters.Counter private _batchIdCounter;

    // Mapping from token ID to asset metadata
    mapping(uint256 => AssetMetadata) private _assetMetadata;

    // Mapping from owner to their asset IDs
    mapping(address => uint256[]) private _ownerAssets;

    // Mapping from external asset ID to internal token ID
    mapping(string => uint256) private _externalToTokenId;

    struct AssetMetadata {
        string assetId;           // External UUID
        uint256 assetType;        // Token type constant
        string name;              // Asset name
        string ipfsHash;          // IPFS reference
        uint256 createdAt;        // Creation timestamp
        bool exists;              // Whether metadata exists
    }

    struct BatchOwnershipRecord {
        string[] assetIds;
        address[] owners;
        uint256[] assetTypes;
        string ipfsHash;
        uint256 timestamp;
    }

    // Events
    event BatchOwnershipRecorded(
        uint256 indexed batchId,
        uint256 count,
        string ipfsHash,
        uint256 timestamp
    );

    event SingleOwnershipRecorded(
        uint256 indexed tokenId,
        string assetId,
        address indexed owner,
        uint256 assetType,
        uint256 timestamp
    );

    constructor(address admin, address minter) ERC1155("") {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(MINTER_ROLE, minter);
    }

    /**
     * @dev Record ownership for multiple assets in a single transaction
     * @param assetIds Array of external asset IDs
     * @param owners Array of owner addresses
     * @param assetTypes Array of asset types
     * @param amounts Array of amounts (usually 1 for NFT-like)
     * @param ipfsHash IPFS reference for batch metadata
     * Requirements: 8.7 - Optimize gas for 2+ assets per transaction
     */
    function recordBatchOwnership(
        string[] calldata assetIds,
        address[] calldata owners,
        uint256[] calldata assetTypes,
        uint256[] calldata amounts,
        string calldata ipfsHash
    ) external onlyRole(MINTER_ROLE) returns (uint256) {
        require(assetIds.length == owners.length, "Array length mismatch");
        require(assetIds.length == assetTypes.length, "Array length mismatch");
        require(assetIds.length == amounts.length, "Array length mismatch");
        require(assetIds.length >= 2, "Minimum 2 assets for batch operation");

        uint256 batchId = _batchIdCounter.current();
        _batchIdCounter.increment();

        uint256[] memory tokenIds = new uint256[](assetIds.length);

        for (uint256 i = 0; i < assetIds.length; i++) {
            require(bytes(assetIds[i]).length > 0, "Asset ID required");
            require(owners[i] != address(0), "Invalid owner");
            require(_externalToTokenId[assetIds[i]] == 0, "Asset already registered");

            // Generate unique token ID
            uint256 tokenId = uint256(keccak256(abi.encodePacked(assetIds[i], block.timestamp, i)));
            tokenIds[i] = tokenId;

            // Store metadata
            _assetMetadata[tokenId] = AssetMetadata({
                assetId: assetIds[i],
                assetType: assetTypes[i],
                name: "",
                ipfsHash: ipfsHash,
                createdAt: block.timestamp,
                exists: true
            });

            _externalToTokenId[assetIds[i]] = tokenId;
            _ownerAssets[owners[i]].push(tokenId);

            // Mint tokens
            _mint(owners[i], tokenId, amounts[i], "");

            emit SingleOwnershipRecorded(tokenId, assetIds[i], owners[i], assetTypes[i], block.timestamp);
        }

        emit BatchOwnershipRecorded(batchId, assetIds.length, ipfsHash, block.timestamp);

        return batchId;
    }

    /**
     * @dev Record single asset ownership
     * @param assetId External asset ID
     * @param owner Owner address
     * @param assetType Asset type
     * @param amount Amount to mint
     * @param ipfsHash IPFS reference
     */
    function recordSingleOwnership(
        string calldata assetId,
        address owner,
        uint256 assetType,
        uint256 amount,
        string calldata ipfsHash
    ) external onlyRole(MINTER_ROLE) returns (uint256) {
        require(bytes(assetId).length > 0, "Asset ID required");
        require(owner != address(0), "Invalid owner");
        require(_externalToTokenId[assetId] == 0, "Asset already registered");

        uint256 tokenId = uint256(keccak256(abi.encodePacked(assetId, block.timestamp)));

        _assetMetadata[tokenId] = AssetMetadata({
            assetId: assetId,
            assetType: assetType,
            name: "",
            ipfsHash: ipfsHash,
            createdAt: block.timestamp,
            exists: true
        });

        _externalToTokenId[assetId] = tokenId;
        _ownerAssets[owner].push(tokenId);

        _mint(owner, tokenId, amount, "");

        emit SingleOwnershipRecorded(tokenId, assetId, owner, assetType, block.timestamp);

        return tokenId;
    }

    /**
     * @dev Get token ID by external asset ID
     * @param assetId External asset ID
     * @return tokenId Internal token ID
     */
    function getTokenId(string calldata assetId) external view returns (uint256) {
        return _externalToTokenId[assetId];
    }

    /**
     * @dev Get asset metadata
     * @param tokenId Token ID
     * @return metadata Asset metadata
     */
    function getAssetMetadata(uint256 tokenId) 
        external 
        view 
        returns (AssetMetadata memory) 
    {
        require(_assetMetadata[tokenId].exists, "Asset not found");
        return _assetMetadata[tokenId];
    }

    /**
     * @dev Get assets owned by an address
     * @param owner Owner address
     * @return tokenIds Array of token IDs
     */
    function getAssetsByOwner(address owner) external view returns (uint256[] memory) {
        return _ownerAssets[owner];
    }

    /**
     * @dev Verify ownership of an asset
     * @param assetId External asset ID
     * @param owner Claimed owner
     * @return isOwner Whether the address owns the asset
     */
    function verifyOwnership(string calldata assetId, address owner) 
        external 
        view 
        returns (bool) 
    {
        uint256 tokenId = _externalToTokenId[assetId];
        if (tokenId == 0) return false;
        return balanceOf(owner, tokenId) > 0;
    }

    /**
     * @dev Transfer ownership of a single asset
     * @param from Current owner
     * @param to New owner
     * @param tokenId Token ID
     * @param amount Amount to transfer
     */
    function transferOwnership(
        address from,
        address to,
        uint256 tokenId,
        uint256 amount
    ) external {
        require(balanceOf(from, tokenId) >= amount, "Insufficient balance");
        
        _safeTransferFrom(from, to, tokenId, amount, "");

        // Update owner assets tracking
        _removeFromOwnerAssets(from, tokenId);
        _ownerAssets[to].push(tokenId);
    }

    /**
     * @dev Batch transfer ownership
     * @param from Current owner
     * @param to New owner
     * @param tokenIds Token IDs
     * @param amounts Amounts to transfer
     */
    function batchTransferOwnership(
        address from,
        address to,
        uint256[] calldata tokenIds,
        uint256[] calldata amounts
    ) external {
        _safeBatchTransferFrom(from, to, tokenIds, amounts, "");

        for (uint256 i = 0; i < tokenIds.length; i++) {
            _removeFromOwnerAssets(from, tokenIds[i]);
            _ownerAssets[to].push(tokenIds[i]);
        }
    }

    /**
     * @dev Remove token from owner's asset list
     */
    function _removeFromOwnerAssets(address owner, uint256 tokenId) internal {
        uint256[] storage assets = _ownerAssets[owner];
        for (uint256 i = 0; i < assets.length; i++) {
            if (assets[i] == tokenId) {
                assets[i] = assets[assets.length - 1];
                assets.pop();
                break;
            }
        }
    }

    function supportsInterface(bytes4 interfaceId) 
        public 
        view 
        override(ERC1155, AccessControl) 
        returns (bool) 
    {
        return super.supportsInterface(interfaceId);
    }

    function uri(uint256 tokenId) public view override returns (string memory) {
        require(_assetMetadata[tokenId].exists, "Asset not found");
        // Return IPFS URI for metadata
        return string(abi.encodePacked("ipfs://", _assetMetadata[tokenId].ipfsHash));
    }
}
