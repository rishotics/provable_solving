// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import { AxiomV2Client } from './AxiomV2Client.sol';
import { IERC20 } from '@openzeppelin-contracts/token/ERC20/IERC20.sol';
import { Ownable } from '@openzeppelin-contracts/access/Ownable.sol';
import "prb-math/contracts/PRBMathSD59x18.sol";



contract AuctioneerChallenge is AxiomV2Client, Ownable {
    uint256 public constant EXPONENT_CONSTANT = 1;
    using PRBMathSD59x18 for uint256;

    struct WinnerData{
        uint256 auctionId;
        uint256 sellingAmount;
        uint256 buyingAmount;
        address winningAddress;
    }
    event PublishWinner(
        uint256 auctionId,
        uint256 sellingAmount,
        uint256 buyingAmount,
        address winningAddress
    );

    
    // event ClaimAirdrop(
    //     address indexed user,
    //     uint256 indexed queryId,
    //     uint256 numTokens,
    //     bytes32[] axiomResults
    // );
    // event ClaimAirdropError(
    //     address indexed user,
    //     string error
    // );
    // event AxiomCallbackQuerySchemaUpdated(bytes32 axiomCallbackQuerySchema);
    event ActioneerUpdated(address OldAuctioneer, address NewAuctioneer);

    // bytes32 public constant SWAP_EVENT_SCHEMA = 0xc42079f94a6350d7e6235f29174924f928cc2ac818eb64fed8004e115fbcca67;
    // address public constant UNI_UNIV_ROUTER_ADDR = 0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD;

    uint64 public callbackSourceChainId;
    bytes32 public axiomCallbackQuerySchema;
    address public auctioneer;
    mapping(address => bool) public querySubmitted;
    mapping(address => bool) public hasClaimed;

    mapping(uint256 => WinnerData) public auctionIdToWinnerData;

    IERC20 public token;

    constructor(
        address _axiomV2QueryAddress, //contarct address for Axioms V2 Query contract
        uint64 _callbackSourceChainId, //chain id for the chain that will be calling the callback
        bytes32 _axiomCallbackQuerySchema,
        address _auctioneer //
    ) AxiomV2Client(_axiomV2QueryAddress) {
        callbackSourceChainId = _callbackSourceChainId;
        axiomCallbackQuerySchema = _axiomCallbackQuerySchema;
        auctioneer = _auctioneer;
    }

    function _publishWinner(
        uint256 auctionId,
        uint256 sellingAmount,
        uint256 buyingAmount,
        bytes32 signature
        address winningAddress
    ) internal {
        require(winningAddress != address(0), "AuctioneerChallenge: Winning address cannot be 0x0");
        bytes32 hashed = keccak256(abi.encodePacked(sellingAmount, buyingAmount, winningAddress));
        bytes32 r = signature[0:32];
        bytes32 s = signature[32:64];
        uint8 v = uint8(signature[64:65]) + 27;
        require(ecrecover(keccak256(hashed,v,r,s)), "AuctioneerChallenge: Invalid signature");
        
        auctionIdToWinningAddress[auctionId] = WinnerData({
            auctionId: auctionId,
            sellingAmount: sellingAmount,
            buyingAmount: buyingAmount,
            winningAddress: winningAddress
        
        });

        emit PublishWinner(auctionId, sellingAmount, buyingAmount, winningAddress);
    }

    function publishWinner(
        uint256 auctionId,
        uint256 sellingAmount,
        uint256 buyingAmount,
        bytes32 signature,
        address winningAddress
    ) public {
        require(msg.sender == auctioneer, "AuctioneerChallenge: Only auctioneer can publish winner");
        _publishWinner(auctionId, sellingAmount, buyingAmount, signature, winningAddress);
    }


    function updateCallbackQuerySchema(
        bytes32 _axiomCallbackQuerySchema
    ) public onlyOwner {
        axiomCallbackQuerySchema = _axiomCallbackQuerySchema;
        emit AxiomCallbackQuerySchemaUpdated(_axiomCallbackQuerySchema);
    }

    function updateAuctioneer(address _auctioneer) public onlyOwner {
        address oldauctioneer = auctioneer;
        auctioneer = _auctioneer;
        emit ActioneerUpdated(oldauctioneer, auctioneer);
    }

    function _axiomV2Callback(
        uint64 sourceChainId,
        address callerAddr,
        bytes32 querySchema,
        uint256 queryId,
        bytes32[] calldata axiomResults,
        bytes calldata extraData
    ) internal virtual override {
        // require(!hasClaimed[callerAddr], "Autonomous Airdrop: User has already claimed this airdrop");

        // Parse results
        address userEventAddress = address(uint160(uint256(axiomResults[0])));
        uint32 blockNumber = uint32(uint256(axiomResults[1]));
        uint256 auctionId = uint256(axiomResults[2]);
        uint256 challengerSellingAmount = uint256(axiomResults[3]);
        uint256 challengerBuyingAmount = uint256(axiomResults[4]);
        address challenger = address(uint160(uint256(axiomResults[5])));
        

        // Validate the results
        
        require(userEventAddress == callerAddr, "AuctioneerChallenge: Invalid user address for event");
        require(blockNumber - block.number <= 18515, "AuctioneerChallenge: Block number for transaction receipt must be less than 3 days");
        require(challengerSellingAmount == auctionIdToWinnerData[auctionId].sellingAmount, "AuctioneerChallenge: Selling amount does not match");
        require(challengerBuyingAmount > auctionIdToWinnerData[auctionId].buyingAmount, "AuctioneerChallenge: Buying amount does not match");
        
        uint256 punishmentAmount = (exponentialFunction(blockNumber - block.number)*exponentialFunction(EXPONENT_CONSTANT)*36900)/100000;

        (bool sent, bytes memory data) = challenger.call{value: punishmentAmount}("");
        require(sent, "Failed to send Ether");

        emit AuctioneerPunished(
            challenger,
            auctionId,
            punishmentAmount,
            axiomResults
        );
    }


    function exponentialFunction(int256 x) public view returns (int256) {
        int256 z = 90000000000000000;      // 0.09
        int256 a = 200000000000000000;     // 0.2
        int256 b = 1080000000000000000;    // 1.08
        int256 c = -10000000000000000000;  // -10
        int256 d = 100000000000000000;     // 0.1
        int256 _x = x * 1000000000000000000;
        int256 outcome = PRBMathSD59x18.mul(a, b.pow(PRBMathSD59x18.mul(z, _x) + c)) + d;
        return outcome;
    }

    function _validateAxiomV2Call(
        uint64 sourceChainId,
        address callerAddr,
        bytes32 querySchema
    ) internal virtual override {
        require(sourceChainId == callbackSourceChainId, "AxiomV2: caller sourceChainId mismatch");
        require(querySchema == axiomCallbackQuerySchema, "AxiomV2: query schema mismatch");
    }
}