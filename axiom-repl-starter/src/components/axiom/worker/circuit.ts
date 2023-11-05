//This file is generated by the AxiomREPL. DO NOT DIRECTLY EDIT THIS FILE!
//To make changes, go to https://repl.axiom.xyz/ and export a new circuit.
//
//                 _                 _____  ______ _____  _
//     /\         (_)               |  __ \|  ____|  __ \| |
//    /  \   __  ___  ___  _ __ ___ | |__) | |__  | |__) | |
//   / /\ \  \ \/ / |/ _ \| '_ ` _ \|  _  /|  __| |  ___/| |
//  / ____ \  >  <| | (_) | | | | | | | \ \| |____| |    | |____
// /_/    \_\/_/\_\_|\___/|_| |_| |_|_|  \_\______|_|    |______|
//
//

import {
  Halo2Lib,
  AxiomData,
  CircuitValue,
  CircuitValue256
} from "@axiom-crypto/experimental/halo2-js";
const defaultInputs = {
  winning_contract_address: ["0x3041CbD36888bECc7bbCBc0045E3B1f144466f5f"],
  token0: ["0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"],
  token1: ["0xdAC17F958D2ee523a2206206994597C13D831ec7"],
  token0Reserve: [321312312],
  token1Reserve: [321312312],
  block: [15002251],
  numSolutions: 1,

  published_block: 15002251,
  published_txIdx: 6,
  published_logIdx: 3,
  published_auction_id: 1,
  published_sell: "0x8f0A561075aA46833c5519001b7579E58EC825C9",
  published_amount: 1000,
  published_buy: "0x8f0A561075aA46833c5519001b7579E58EC825C9",
  published_task_signature: "",
  published_winning_value: 2131312312321321,
  published_winning_address: "0x8f0A561075aA46833c5519001b7579E58EC825C9",

  challenge_auction_id: 1,
  challenge_sell: "0x8f0A561075aA46833c5519001b7579E58EC825C9",
  challenge_amount: 1000,
  challenge_buy: "0x8f0A561075aA46833c5519001b7579E58EC825C9",
  challenge_task_signature: "",
  challenge_solution: "",
  challenge_value: 2131312312321322,
  challenge_signature: "",
  challenge_receipt: ""
};
type CircuitInputType = typeof defaultInputs;
export interface CircuitInputs extends CircuitInputType {}
export interface CircuitValueInputs {
  winning_contract_address: CircuitValue[];
  token0: CircuitValue[];
  token1: CircuitValue[];
  token0Reserve: CircuitValue[];
  token1Reserve: CircuitValue[];
  block: CircuitValue[];
  numSolutions: CircuitValue;
  published_block: CircuitValue;
  published_txIdx: CircuitValue;
  published_logIdx: CircuitValue;
  published_auction_id: CircuitValue;
  published_sell: CircuitValue;
  published_amount: CircuitValue;
  published_buy: CircuitValue;
  published_task_signature: CircuitValue;
  published_winning_value: CircuitValue;
  published_winning_address: CircuitValue;
  challenge_auction_id: CircuitValue;
  challenge_sell: CircuitValue;
  challenge_amount: CircuitValue;
  challenge_buy: CircuitValue;
  challenge_task_signature: CircuitValue;
  challenge_solution: CircuitValue;
  challenge_value: CircuitValue;
  challenge_signature: CircuitValue;
  challenge_receipt: CircuitValue;
}
const circuitFn = async (
  halo2Lib: Halo2Lib,
  axiomData: AxiomData,
  {
    winning_contract_address,
    token0,
    token1,
    token0Reserve,
    token1Reserve,
    block,
    numSolutions,
    published_block,
    published_txIdx,
    published_logIdx,
    published_auction_id,
    published_sell,
    published_amount,
    published_buy,
    published_task_signature,
    published_winning_value,
    published_winning_address,
    challenge_auction_id,
    challenge_sell,
    challenge_amount,
    challenge_buy,
    challenge_task_signature,
    challenge_solution,
    challenge_value,
    challenge_signature,
    challenge_receipt
  }: CircuitValueInputs
) => {
  const {
    witness,
    constant,
    add,
    sub,
    mul,
    and,
    or,
    not,
    isEqual,
    checkEqual,
    isLessThan,
    div,
    value,
    log
  } = halo2Lib;
  const { getAccount, getReceipt, getStorage } = axiomData;
  /*
    Step 1: Prove that the winning solution auctioneer submitted is correct.
    Step 2: Verify the reciept is the one aunctioneer sent to the solver
    Step 3: Compare winning solution and solver's solution
*/

  //This file is generated by the AxiomREPL. DO NOT DIRECTLY EDIT THIS FILE!
  //To make changes, go to https://repl.axiom.xyz/ and export a new circuit.
  //
  //                 _                 _____  ______ _____  _
  //     /\         (_)               |  __ \|  ____|  __ \| |
  //    /  \   __  ___  ___  _ __ ___ | |__) | |__  | |__) | |
  //   / /\ \  \ \/ / |/ _ \| '_ ` _ \|  _  /|  __| |  ___/| |
  //  / ____ \  >  <| | (_) | | | | | | | \ \| |____| |    | |____
  // /_/    \_\/_/\_\_|\___/|_| |_| |_|_|  \_\______|_|    |______|
  //
  //

  // Some constants

  const zero = BigInt(0);
  const one = BigInt(1);

  // ---------------------------Clarification of inputs-------------------------------------

  // Below input should be read inside the circuit:
  // @rishabh: Fi what we can do is read the event from publishing and then check values from event and provided by the challenger
  /*
"published_auction_id": 1
--auction id the challenger is challenging, if we use challenge_auction_id that challenger provides,
--then this paramater is redundant.
--(not sure if using this id to find corresponding storage is convinient, of course for the presentation,
--we can choose whatever way that is convinient)

"published_sell": "0x8f0A561075aA46833c5519001b7579E58EC825C9",
--In this auction, the currency to sell, which should be same as token0 based on today's conversation

"published_amount": 1000,
--In this auction, the amount of currency (published_sell) want to sell

"published_buy": "0x8f0A561075aA46833c5519001b7579E58EC825C9",
--In this auction, the currency to buy, which should be same as token1 based on today's conversation

"published_task_signature": "",
--In this auction, the signature of the task,
--i.e. published_task_signature = SIG_{auctioneer}(published_auction_id, published_sell, published_amount, published_buy)
--Again, if we cannot find easy way to verify signature using current Axiom, we can check it in smart contract (EVM).
//@rishabh : why do we need to check the signature, here?

"published_winning_value": 2131312312321321,
--The published winning value (amount of currency bought in the winning solution, given the uni-swap pools' states at the time that auction happened)
*/

  // Below input are real input to the circuit:
  /*
"challenge_auction_id": 1,
"challenge_sell": "0x8f0A561075aA46833c5519001b7579E58EC825C9",
"challenge_amount": 1000,
"challenge_buy": "0x8f0A561075aA46833c5519001b7579E58EC825C9",
"challenge_task_signature": "",
"challenge_value": 2131312312321322,
--above 6 terms are similar as above, only difference is these are provided by the challenger, 
--they are inputs of the circuit instead of read inside the circuit
--and be aware that challenge_task_signature is also signed by auctioneer at the time he publish task
--to solvers, we need this here to show that the challenger doesn't make up fake task

"challenge_solution": "",
--this is complete chllenger's solution. Based on today's meeting, we can simplify the solution
--involves only 1 exchange in 1 uniswap pool. Then this can just be 1 contract_address.
--However, it is ok to leave it here since we are not going to check challenger's complete solution in the
--challenge() method. The verification is done by the auctioneer before he sent solver the receipt,
--in challenge() method, we verify chllenger's soluton only by verify the signature signed by auctioneer
--in the receipt.

"challenge_signature": "",
--This is SIG_{solver/chllenger}(all above 7 paramaters, i.e. challenge_auction_id, ..., challenge_solution)
--This is signed by challenger
--again, we can check this outside the circuit but in smart contract/ EVM

"challenge_receipt": ""
--This is SIG_{auctioneer}(all above 8 paramaters, i.e. challenge_auction_id, ..., challenge_signature)
--This is signed by auctioneer
--again, we can check this outside the circuit but in smart contract/ EVM
*/

  // parameters published_sell, published_buy and token0, token1 are same

  // Changed some codes in line 152-170, for this part, I think it is better to just check if the auctioneer's
  // published winning value is valid or not. And comparison of winning value and challenge value is
  // is in the last step (line 199 - 204). Since if auctioneer published fake values, we may want to splash auctioneer directly.

  // Checked Uni-swap V2 amount out method, function checkUniswapV2Exec() looks good to me. I only changed
  // inputs and output (line 167)

  /*
---------------------part we need to do-------------------------
1. read parameters (line 2-14 in the input) from smart contract given challenge_auction_id
    (if using id to loop up is convinient, if not we may want challenger provides block number and we can 
    use block number to read?)
2. check following signatures in the smart contract:
    a. signature published_task_signature, if this is wrong, challenge succeeds
    b. challenge_task_signature, if this is wrong, challenge fail
    c. challenge_receipt, if wrong, challenge fail
    d. challenge_signature, if challenge_receipt is correct, then if this is wrong means auctioneer
        cheated, so if challenge_receipt is correcnt, and challenge_signature is wrong, then challenge succeeds
3. check age in smart contract
4. output of the circuit when challenge success / challenge fail(call back functions?).
*/

  //--------------------------------------------------------------------------------------------------

  // Step 1: count block age, if > 3 days, challenge fail

  // Q1 : Do it in EVM?

  //--------------------------------------------------------------------------------------------------

  // Step 2: verify auctioneer published legal winning solution

  // substep 2.0: read the winning solution [Ex1,Ex2, …], make sure they are in valid form

  // Q2 : want to read from smart contract's storage instead of inputs to the circuit?

  checkEqual(witness(winning_contract_address.length), numSolutions);
  checkEqual(witness(token0.length), numSolutions);
  checkEqual(witness(token1.length), numSolutions);
  checkEqual(witness(block.length), numSolutions);

  //substep 2.1: check if provided values for published values are correct
  // PublishWinner(auctionId, sellingAmount, buyingAmount, winningAddress, block.number)
  const eventSchema =
    "0x573f2df6684fe332379c83fff9268aeb6523c26466925f7f5a930702aa9e6482";
  // let receipt = getReceipt(published_block, published_txIdx);
  // let receiptLog = receipt.log(published_logIdx);
  // let publishSchema = receiptLog.topic(0, eventSchema);
  // let auctionId = receiptLog.topic(0, eventSchema).toCircuitValue();
  // let sellingAmount = receiptLog.topic(1, eventSchema).toCircuitValue();
  // let buyingAmount = receiptLog.topic(2, eventSchema).toCircuitValue();
  // let winningAddress = receiptLog.topic(3, eventSchema).toCircuitValue();

  // checkEqual(auctionId, published_auction_id);
  // checkEqual(sellingAmount, published_sell);
  // checkEqual(buyingAmount, published_buy);
  // checkEqual(winningAddress, published_winning_address);

  for (var i = 0; i < numSolutions.value(); i++) {
    //   let pool = getAccount(winning_contract_address[i], block[i]);
    //  extract value of tokens prices from the pool of uinswap v2
    const storage = getStorage(block[i], winning_contract_address[i]);
    //   log(storage);

    // gets the value of the specified slot in the contract's storage
    //   const slotValueToken0Reserve = storage.slot(constant(5));
    //   const slotValueToken1Reserve = storage.slot(constant(6));
    //   checkEqual(token0Reserve[i], slotValueToken0Reserve.toCircuitValue());
    //   checkEqual(token1Reserve[i], slotValueToken1Reserve.toCircuitValue());

    checkUniswapV2Exec(
      published_winning_value,
      token0Reserve[i],
      token1Reserve[i],
      published_amount
    );
  }

  function checkUniswapV2Exec(
    publishedAmountOut,
    reserveIn,
    reserveOut,
    token0in
  ) {
    //code from uniV2 https://github.com/Uniswap/v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol#L43
    const amountInWithFee = mul(token0in, constant(997));
    log(amountInWithFee);
    const numerator = mul(amountInWithFee, reserveOut);
    const denominator = add(mul(constant(1000), reserveIn), amountInWithFee);
    const amountOut = div(numerator, denominator);
    // checkEqual(amountOut, challengeAmountOut);
    if (isLessThan(publishedAmountOut, amountOut).value() === one) {
      // challenge success
    }
  }

  // substep 2.2: follow Ex1, Ex2, …, verify they are legal and also update amounts of currencies in pools

  // substep 2.3: the final amount of C1 is no smaller than the published winning value Vw.

  //--------------------------------------------------------------------------------------------------

  // Step 3: verify the challenged task by comparing to the published one
  //         verify challenger's solution by verifying the receipt he provides
  //         signature verification part is currently in EVM, waiting for Axiom's ecrecover of signature verification

  let isSameId = isEqual(published_auction_id, challenge_auction_id).value();
  let isSameSell = isEqual(published_sell, challenge_sell).value();
  let isSameBuy = isEqual(published_buy, challenge_buy).value();
  let isSameAmount = isEqual(published_amount, challenge_amount).value();
  let isSameSignature = isEqual(
    published_task_signature,
    challenge_task_signature
  ).value();

  if (
    isSameId == zero ||
    isSameSell == zero ||
    isSameBuy == zero ||
    isSameAmount == zero ||
    isSameSignature == zero
  ) {
    // challenge fail
  }

  // Step 4: compare winning value and challenger's value

  let challenge_success = isLessThan(
    published_winning_value,
    challenge_value
  ).value();
  if (challenge_success == one) {
    // challenge success, splash auctioneer
  } else {
    // challenge fail
  }
};
const config = {
  k: 13,
  numAdvice: 4,
  numLookupAdvice: 1,
  numInstance: 1,
  numLookupBits: 12,
  numVirtualInstance: 2
};
const vk = [
  2,
  13,
  0,
  0,
  0,
  0,
  6,
  0,
  0,
  0,
  22,
  53,
  175,
  191,
  189,
  44,
  47,
  125,
  102,
  223,
  68,
  183,
  53,
  24,
  221,
  245,
  11,
  40,
  210,
  84,
  147,
  34,
  241,
  111,
  251,
  44,
  176,
  97,
  40,
  23,
  111,
  5,
  236,
  172,
  54,
  30,
  205,
  68,
  139,
  37,
  34,
  255,
  110,
  222,
  63,
  213,
  167,
  105,
  46,
  125,
  148,
  2,
  105,
  228,
  6,
  175,
  114,
  9,
  31,
  238,
  182,
  133,
  168,
  45,
  239,
  2,
  5,
  56,
  102,
  139,
  230,
  228,
  0,
  108,
  94,
  240,
  97,
  214,
  250,
  140,
  47,
  136,
  25,
  45,
  25,
  14,
  17,
  131,
  44,
  222,
  11,
  222,
  35,
  145,
  185,
  22,
  2,
  13,
  71,
  122,
  25,
  76,
  214,
  203,
  241,
  133,
  100,
  193,
  18,
  255,
  150,
  67,
  235,
  216,
  214,
  31,
  121,
  27,
  191,
  226,
  31,
  20,
  57,
  122,
  47,
  200,
  88,
  42,
  132,
  44,
  82,
  29,
  23,
  154,
  210,
  47,
  50,
  153,
  92,
  2,
  250,
  47,
  221,
  123,
  135,
  104,
  188,
  35,
  136,
  122,
  55,
  105,
  178,
  243,
  227,
  131,
  196,
  115,
  0,
  6,
  9,
  206,
  94,
  26,
  185,
  164,
  51,
  24,
  136,
  117,
  255,
  172,
  78,
  248,
  0,
  82,
  150,
  158,
  249,
  197,
  232,
  10,
  135,
  16,
  9,
  164,
  55,
  183,
  168,
  73,
  44,
  36,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  0,
  54,
  253,
  106,
  203,
  70,
  30,
  38,
  52,
  82,
  37,
  107,
  238,
  95,
  141,
  103,
  222,
  69,
  237,
  24,
  97,
  126,
  45,
  96,
  80,
  148,
  125,
  7,
  96,
  229,
  236,
  87,
  36,
  95,
  86,
  173,
  44,
  25,
  36,
  105,
  148,
  235,
  188,
  174,
  41,
  4,
  165,
  77,
  149,
  149,
  23,
  97,
  216,
  3,
  24,
  70,
  73,
  70,
  198,
  80,
  141,
  113,
  195,
  117,
  29,
  146,
  227,
  194,
  31,
  189,
  45,
  218,
  70,
  1,
  188,
  167,
  155,
  44,
  45,
  215,
  85,
  61,
  145,
  104,
  239,
  200,
  88,
  242,
  241,
  168,
  17,
  161,
  67,
  102,
  67,
  219,
  32,
  195,
  16,
  98,
  181,
  99,
  48,
  111,
  138,
  213,
  86,
  222,
  160,
  33,
  17,
  19,
  118,
  218,
  128,
  56,
  71,
  137,
  144,
  165,
  107,
  72,
  192,
  243,
  186,
  83,
  219,
  134,
  19,
  129,
  32,
  221,
  160,
  2,
  15,
  131,
  249,
  95,
  54,
  190,
  51,
  37,
  210,
  75,
  10,
  123,
  164,
  170,
  220,
  46,
  2,
  32,
  0,
  126,
  162,
  161,
  23,
  118,
  254,
  8,
  8,
  145,
  202,
  133,
  199,
  119,
  206,
  57,
  43,
  71,
  250,
  177,
  202,
  247,
  247,
  49,
  208,
  24,
  55,
  134,
  206,
  167,
  14,
  195,
  5,
  67,
  75,
  229,
  119,
  93,
  216,
  75,
  48,
  129,
  127,
  109,
  132,
  109,
  219,
  168,
  23,
  159,
  8,
  162,
  147,
  15,
  247,
  240,
  86,
  108,
  80,
  248,
  240,
  65,
  159,
  237,
  247,
  215,
  190,
  191,
  70,
  240,
  218,
  95,
  15,
  139,
  84,
  196,
  177,
  252,
  158,
  196,
  233,
  173,
  21,
  59,
  139,
  120,
  126,
  241,
  79,
  176,
  156,
  21,
  225,
  98,
  163,
  218,
  200,
  210,
  106,
  88,
  71,
  32,
  119,
  134,
  30,
  248,
  17,
  160,
  55,
  121,
  168,
  124,
  85,
  5,
  232,
  156,
  11,
  224,
  89,
  116,
  78,
  181,
  45,
  120,
  198,
  223,
  203,
  156,
  189,
  160,
  140,
  117,
  105,
  10,
  53,
  212,
  37,
  140,
  202,
  224,
  95,
  204,
  114,
  5,
  234,
  227,
  19,
  84,
  3,
  218,
  83,
  80,
  10,
  207,
  66,
  72,
  41,
  104,
  80,
  210,
  173,
  6,
  147,
  3,
  3,
  204,
  9,
  218,
  43,
  212,
  46,
  116,
  126,
  132,
  110,
  248,
  72,
  239,
  200,
  62,
  156,
  203,
  162,
  106,
  151,
  100,
  189,
  17,
  211,
  193,
  166,
  128,
  123,
  135,
  86,
  226,
  226,
  113,
  56,
  182,
  47,
  161,
  27,
  89,
  176,
  200,
  131,
  125,
  116,
  140,
  76,
  189,
  46,
  160,
  102,
  134,
  4,
  164,
  25,
  124,
  8,
  224,
  4,
  84,
  48,
  84,
  253,
  240,
  129,
  201,
  241,
  141,
  34,
  113,
  128,
  248,
  224,
  219,
  148,
  168,
  132,
  230,
  181,
  47,
  187,
  74,
  43,
  91,
  35,
  15,
  215,
  255,
  164,
  72,
  89,
  155,
  34,
  197,
  8,
  57,
  55,
  76,
  216,
  151,
  5,
  110,
  207,
  108,
  57,
  65,
  216,
  137,
  53,
  80,
  238,
  237,
  219,
  221,
  157,
  219,
  150,
  99,
  25,
  41,
  178,
  150,
  25,
  11,
  59,
  71,
  16,
  243,
  18,
  55,
  209,
  151,
  19
];
export const circuit = Object.freeze({
  circuit: circuitFn,
  config,
  defaultInputs,
  vk
});
