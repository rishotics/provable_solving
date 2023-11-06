import { ethers } from "hardhat";

async function main() {
  
  const lock = await ethers.deployContract("AuctioneerChallenge", ["0x28CeE427fCD58e5EF1cE4C93F877b621E2Db66df", "5", "0x8f0A561075aA46833c5519001b7579E58EC825C9"], {
    value: 0,
  });

  await lock.waitForDeployment();

  console.log(
    `AuctioneerChallenge deployed to: ${lock.target}`
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
