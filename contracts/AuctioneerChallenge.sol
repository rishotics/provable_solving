// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

import { AxiomV2Client } from './AxiomV2Client.sol';

import "prb-math/contracts/PRBMathUD60x18.sol";
import "./ECDSA.sol";


contract AuctioneerChallenge is AxiomV2Client {
    uint256 public constant EXPONENT_CONSTANT = 1;
    using PRBMathUD60x18 for uint256;
    using ECDSA for bytes32;

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
        address winningAddress,
        uint256 blockNumber
    );

    event AuctioneerPunished(
        address indexed challenger,
        uint256 auctionId,
        uint256 punishmentAmount,
        bytes32[] axiomResults
    );
    event AxiomCallbackQuerySchemaUpdated(bytes32 axiomCallbackQuerySchema);

    
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


    constructor(
        address _axiomV2QueryAddress, //contarct address for Axioms V2 Query contract
        uint64 _callbackSourceChainId, //chain id for the chain that will be calling the callback
        address _auctioneer //
    ) AxiomV2Client(_axiomV2QueryAddress) {
        callbackSourceChainId = _callbackSourceChainId;
        
        auctioneer = _auctioneer;
    }

    function _publishWinner(
        uint256 auctionId,
        uint256 sellingAmount,
        uint256 buyingAmount,
        bytes memory signature,
        address winningAddress
    ) internal {
        require(winningAddress != address(0), "AuctioneerChallenge: Winning address cannot be 0x0");
        bytes32 hashed = keccak256(abi.encodePacked(sellingAmount, buyingAmount, winningAddress));
        require(hashed.recover(signature) == auctioneer, "AuctioneerChallenge: Invalid signature");
        
        auctionIdToWinnerData[auctionId] = WinnerData({
            auctionId: auctionId,
            sellingAmount: sellingAmount,
            buyingAmount: buyingAmount,
            winningAddress: winningAddress
        
        });

        emit PublishWinner(auctionId, sellingAmount, buyingAmount, winningAddress, block.number);
    }

    function publishWinner(
        uint256 auctionId,
        uint256 sellingAmount,
        uint256 buyingAmount,
        bytes memory signature,
        address winningAddress
    ) public {
        require(msg.sender == auctioneer, "AuctioneerChallenge: Only auctioneer can publish winner");
        _publishWinner(auctionId, sellingAmount, buyingAmount, signature, winningAddress);
    }


    // function updateCallbackQuerySchema(
    //     bytes32 _axiomCallbackQuerySchema
    // ) public  {
    //     axiomCallbackQuerySchema = _axiomCallbackQuerySchema;
    //     emit AxiomCallbackQuerySchemaUpdated(_axiomCallbackQuerySchema);
    // }

    function updateAuctioneer(address _auctioneer) public  {
        address oldauctioneer = auctioneer;
        auctioneer = _auctioneer;
        emit ActioneerUpdated(oldauctioneer, auctioneer);
    }

    function _axiomV2Callback(
        uint64 sourceChainId,
        address callerAddr,
        bytes32 querySchema,
        bytes32 queryHash,
        bytes32[] calldata axiomResults,
        bytes calldata callbackExtraData
    ) internal virtual override  {
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
        
        uint256 ageExp = (((blockNumber - block.number)*36900)/100000).exp();
        uint256 const = EXPONENT_CONSTANT.exp();
        uint256 punishmentAmount = ((ageExp.mul(const)).mul(367879)).div(1000000);
        
        (bool sent, bytes memory data) = challenger.call{value: punishmentAmount}("");
        require(sent, "Failed to send Ether");

        emit AuctioneerPunished(
            challenger,
            auctionId,
            punishmentAmount,
            axiomResults
        );
    }


    

}