/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumberish,
  BytesLike,
  FunctionFragment,
  Result,
  Interface,
  EventFragment,
  AddressLike,
  ContractRunner,
  ContractMethod,
  Listener,
} from "ethers";
import type {
  TypedContractEvent,
  TypedDeferredTopicFilter,
  TypedEventLog,
  TypedLogDescription,
  TypedListener,
  TypedContractMethod,
} from "./common";

export interface AxiomV2ClientInterface extends Interface {
  getFunction(
    nameOrSignature: "axiomV2Callback" | "axiomV2QueryAddress"
  ): FunctionFragment;

  getEvent(nameOrSignatureOrTopic: "AxiomV2Call"): EventFragment;

  encodeFunctionData(
    functionFragment: "axiomV2Callback",
    values: [
      BigNumberish,
      AddressLike,
      BytesLike,
      BytesLike,
      BytesLike[],
      BytesLike
    ]
  ): string;
  encodeFunctionData(
    functionFragment: "axiomV2QueryAddress",
    values?: undefined
  ): string;

  decodeFunctionResult(
    functionFragment: "axiomV2Callback",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "axiomV2QueryAddress",
    data: BytesLike
  ): Result;
}

export namespace AxiomV2CallEvent {
  export type InputTuple = [
    sourceChainId: BigNumberish,
    callerAddr: AddressLike,
    querySchema: BytesLike,
    queryHash: BytesLike
  ];
  export type OutputTuple = [
    sourceChainId: bigint,
    callerAddr: string,
    querySchema: string,
    queryHash: string
  ];
  export interface OutputObject {
    sourceChainId: bigint;
    callerAddr: string;
    querySchema: string;
    queryHash: string;
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>;
  export type Filter = TypedDeferredTopicFilter<Event>;
  export type Log = TypedEventLog<Event>;
  export type LogDescription = TypedLogDescription<Event>;
}

export interface AxiomV2Client extends BaseContract {
  connect(runner?: ContractRunner | null): AxiomV2Client;
  waitForDeployment(): Promise<this>;

  interface: AxiomV2ClientInterface;

  queryFilter<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;
  queryFilter<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;

  on<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  on<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  once<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  once<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  listeners<TCEvent extends TypedContractEvent>(
    event: TCEvent
  ): Promise<Array<TypedListener<TCEvent>>>;
  listeners(eventName?: string): Promise<Array<Listener>>;
  removeAllListeners<TCEvent extends TypedContractEvent>(
    event?: TCEvent
  ): Promise<this>;

  axiomV2Callback: TypedContractMethod<
    [
      sourceChainId: BigNumberish,
      callerAddr: AddressLike,
      querySchema: BytesLike,
      queryHash: BytesLike,
      axiomResults: BytesLike[],
      callbackExtraData: BytesLike
    ],
    [void],
    "nonpayable"
  >;

  axiomV2QueryAddress: TypedContractMethod<[], [string], "view">;

  getFunction<T extends ContractMethod = ContractMethod>(
    key: string | FunctionFragment
  ): T;

  getFunction(
    nameOrSignature: "axiomV2Callback"
  ): TypedContractMethod<
    [
      sourceChainId: BigNumberish,
      callerAddr: AddressLike,
      querySchema: BytesLike,
      queryHash: BytesLike,
      axiomResults: BytesLike[],
      callbackExtraData: BytesLike
    ],
    [void],
    "nonpayable"
  >;
  getFunction(
    nameOrSignature: "axiomV2QueryAddress"
  ): TypedContractMethod<[], [string], "view">;

  getEvent(
    key: "AxiomV2Call"
  ): TypedContractEvent<
    AxiomV2CallEvent.InputTuple,
    AxiomV2CallEvent.OutputTuple,
    AxiomV2CallEvent.OutputObject
  >;

  filters: {
    "AxiomV2Call(uint64,address,bytes32,bytes32)": TypedContractEvent<
      AxiomV2CallEvent.InputTuple,
      AxiomV2CallEvent.OutputTuple,
      AxiomV2CallEvent.OutputObject
    >;
    AxiomV2Call: TypedContractEvent<
      AxiomV2CallEvent.InputTuple,
      AxiomV2CallEvent.OutputTuple,
      AxiomV2CallEvent.OutputObject
    >;
  };
}
