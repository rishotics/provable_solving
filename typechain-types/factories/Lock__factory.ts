/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import {
  Contract,
  ContractFactory,
  ContractTransactionResponse,
  Interface,
} from "ethers";
import type {
  Signer,
  BigNumberish,
  ContractDeployTransaction,
  ContractRunner,
} from "ethers";
import type { PayableOverrides } from "../common";
import type { Lock, LockInterface } from "../Lock";

const _abi = [
  {
    inputs: [
      {
        internalType: "uint256",
        name: "_unlockTime",
        type: "uint256",
      },
    ],
    stateMutability: "payable",
    type: "constructor",
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: "uint256",
        name: "amount",
        type: "uint256",
      },
      {
        indexed: false,
        internalType: "uint256",
        name: "when",
        type: "uint256",
      },
    ],
    name: "Withdrawal",
    type: "event",
  },
  {
    inputs: [],
    name: "owner",
    outputs: [
      {
        internalType: "address payable",
        name: "",
        type: "address",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "unlockTime",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [],
    name: "withdraw",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
] as const;

const _bytecode =
  "0x6080601f61034538819003918201601f19168301916001600160401b038311848410176100bc578084926020946040528339810103126100b757518042101561006657600055600180546001600160a01b0319163317905560405161027290816100d38239f35b60405162461bcd60e51b815260206004820152602360248201527f556e6c6f636b2074696d652073686f756c6420626520696e207468652066757460448201526275726560e81b6064820152608490fd5b600080fd5b634e487b7160e01b600052604160045260246000fdfe60806040818152600436101561001457600080fd5b600091823560e01c908163251c1aa314610204575080633ccfd60b1461009857638da5cb5b1461004357600080fd5b3461009457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc3601126100945760209073ffffffffffffffffffffffffffffffffffffffff600154169051908152f35b5080fd5b503461009457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261009457815442106101a75773ffffffffffffffffffffffffffffffffffffffff6001541680330361014a5782808080937fbf2ed60bd5b5965d685680c01195c9514e4382e28e3a5a2d2d5244bf59411b9386478151908152426020820152a14790828215610141575bf115610137575080f35b51903d90823e3d90fd5b506108fc61012d565b606482517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f596f75206172656e277420746865206f776e65720000000000000000000000006044820152fd5b606490517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601660248201527f596f752063616e277420776974686472617720796574000000000000000000006044820152fd5b83903461009457817ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc36011261009457602091548152f3fea264697066735822122064fa4966bc23a9e39c43f0098a7a4f5c55173ed1dc5aafa6ba7f016965e2c41664736f6c63430008130033";

type LockConstructorParams =
  | [signer?: Signer]
  | ConstructorParameters<typeof ContractFactory>;

const isSuperArgs = (
  xs: LockConstructorParams
): xs is ConstructorParameters<typeof ContractFactory> => xs.length > 1;

export class Lock__factory extends ContractFactory {
  constructor(...args: LockConstructorParams) {
    if (isSuperArgs(args)) {
      super(...args);
    } else {
      super(_abi, _bytecode, args[0]);
    }
  }

  override getDeployTransaction(
    _unlockTime: BigNumberish,
    overrides?: PayableOverrides & { from?: string }
  ): Promise<ContractDeployTransaction> {
    return super.getDeployTransaction(_unlockTime, overrides || {});
  }
  override deploy(
    _unlockTime: BigNumberish,
    overrides?: PayableOverrides & { from?: string }
  ) {
    return super.deploy(_unlockTime, overrides || {}) as Promise<
      Lock & {
        deploymentTransaction(): ContractTransactionResponse;
      }
    >;
  }
  override connect(runner: ContractRunner | null): Lock__factory {
    return super.connect(runner) as Lock__factory;
  }

  static readonly bytecode = _bytecode;
  static readonly abi = _abi;
  static createInterface(): LockInterface {
    return new Interface(_abi) as LockInterface;
  }
  static connect(address: string, runner?: ContractRunner | null): Lock {
    return new Contract(address, _abi, runner) as unknown as Lock;
  }
}
