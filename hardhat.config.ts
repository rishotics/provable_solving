import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: {
    version: "0.8.19",
    settings: {
      optimizer: {
        enabled: true,
        runs: 10000,
      },
      viaIR: true,
    },
      },
  defaultNetwork: "goerli",
    networks: {
      hardhat: {
        allowUnlimitedContractSize: true,
      },
    goerli: {
      url: `https://eth-goerli.g.alchemy.com/v2/BkTen6TcAuQh8m3WOTHWlOahn4Itn7_R`,
      accounts: ["af81468d7adf2bdf5a35fd2fab612fe3d1b22fb3302c0cafad1481ab6e79a5c6"]
    },
  }
};

export default config;
