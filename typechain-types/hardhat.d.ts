/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { ethers } from "ethers";
import {
  DeployContractOptions,
  FactoryOptions,
  HardhatEthersHelpers as HardhatEthersHelpersBase,
} from "@nomicfoundation/hardhat-ethers/types";

import * as Contracts from ".";

declare module "hardhat/types/runtime" {
  interface HardhatEthersHelpers extends HardhatEthersHelpersBase {
    getContractFactory(
      name: "AuctioneerChallenge",
      signerOrOptions?: ethers.Signer | FactoryOptions
    ): Promise<Contracts.AuctioneerChallenge__factory>;
    getContractFactory(
      name: "AxiomV2Client",
      signerOrOptions?: ethers.Signer | FactoryOptions
    ): Promise<Contracts.AxiomV2Client__factory>;
    getContractFactory(
      name: "ECDSA",
      signerOrOptions?: ethers.Signer | FactoryOptions
    ): Promise<Contracts.ECDSA__factory>;
    getContractFactory(
      name: "IAxiomV2Client",
      signerOrOptions?: ethers.Signer | FactoryOptions
    ): Promise<Contracts.IAxiomV2Client__factory>;
    getContractFactory(
      name: "Lock",
      signerOrOptions?: ethers.Signer | FactoryOptions
    ): Promise<Contracts.Lock__factory>;

    getContractAt(
      name: "AuctioneerChallenge",
      address: string | ethers.Addressable,
      signer?: ethers.Signer
    ): Promise<Contracts.AuctioneerChallenge>;
    getContractAt(
      name: "AxiomV2Client",
      address: string | ethers.Addressable,
      signer?: ethers.Signer
    ): Promise<Contracts.AxiomV2Client>;
    getContractAt(
      name: "ECDSA",
      address: string | ethers.Addressable,
      signer?: ethers.Signer
    ): Promise<Contracts.ECDSA>;
    getContractAt(
      name: "IAxiomV2Client",
      address: string | ethers.Addressable,
      signer?: ethers.Signer
    ): Promise<Contracts.IAxiomV2Client>;
    getContractAt(
      name: "Lock",
      address: string | ethers.Addressable,
      signer?: ethers.Signer
    ): Promise<Contracts.Lock>;

    deployContract(
      name: "AuctioneerChallenge",
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.AuctioneerChallenge>;
    deployContract(
      name: "AxiomV2Client",
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.AxiomV2Client>;
    deployContract(
      name: "ECDSA",
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.ECDSA>;
    deployContract(
      name: "IAxiomV2Client",
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.IAxiomV2Client>;
    deployContract(
      name: "Lock",
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.Lock>;

    deployContract(
      name: "AuctioneerChallenge",
      args: any[],
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.AuctioneerChallenge>;
    deployContract(
      name: "AxiomV2Client",
      args: any[],
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.AxiomV2Client>;
    deployContract(
      name: "ECDSA",
      args: any[],
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.ECDSA>;
    deployContract(
      name: "IAxiomV2Client",
      args: any[],
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.IAxiomV2Client>;
    deployContract(
      name: "Lock",
      args: any[],
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<Contracts.Lock>;

    // default types
    getContractFactory(
      name: string,
      signerOrOptions?: ethers.Signer | FactoryOptions
    ): Promise<ethers.ContractFactory>;
    getContractFactory(
      abi: any[],
      bytecode: ethers.BytesLike,
      signer?: ethers.Signer
    ): Promise<ethers.ContractFactory>;
    getContractAt(
      nameOrAbi: string | any[],
      address: string | ethers.Addressable,
      signer?: ethers.Signer
    ): Promise<ethers.Contract>;
    deployContract(
      name: string,
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<ethers.Contract>;
    deployContract(
      name: string,
      args: any[],
      signerOrOptions?: ethers.Signer | DeployContractOptions
    ): Promise<ethers.Contract>;
  }
}
