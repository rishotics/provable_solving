"use client";

import { useEffect, useState } from "react";
import { AxiomV2Callback } from "@axiom-crypto/experimental";
import { ConnectKitButton } from "connectkit";
import { writeContract } from "wagmi/actions";
import { useAccount } from "wagmi";
import { useAxiomCircuit } from "@/components/axiom/AxiomCircuitProvider";
import { CircuitInputs, circuit } from "@/components/axiom/worker/circuit";

export default function Home() {
  const mock = (process.env.NEXT_PUBLIC_MOCK ?? "true").toLowerCase() === "true";
  const defaultTarget = mock ? "0x8FB73CE80FDB8F15877B161e4Fe08b2a0A9979A9" : "0x41a7a901ef58d383801272d2408276d96973550d";

  const [input, setInput] = useState<string>(JSON.stringify(circuit.defaultInputs));
  const [target, setTarget] = useState<string>(defaultTarget);
  const [extraData, setExtraData] = useState<string>("0x0");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isBuilt, setIsBuilt] = useState<boolean>(false);
  const { address } = useAccount();
  const { axiom, setParams, build, reset } = useAxiomCircuit();

  useEffect(() => {
    const inputs: CircuitInputs = JSON.parse(input);
    const callback: AxiomV2Callback = {
      target,
      extraData,
    };
    setParams(inputs, callback);
  }, [input, target, extraData, setParams]);

  const generateAndSendQuery = async () => {
    setIsLoading(true);
    const buildRes = await build();
    if (buildRes === null) {
      setIsLoading(false);
      console.error("Unable to build Query");
      return;
    }
    const { builtQuery, payment } = buildRes;
    setIsBuilt(true);
    setIsLoading(false);

    const queryArgs = [
      builtQuery.sourceChainId,
      builtQuery.dataQueryHash,
      builtQuery.computeQuery,
      builtQuery.callback,
      builtQuery.userSalt,
      builtQuery.maxFeePerGas,
      builtQuery.callbackGasLimit,
      address,
      builtQuery.dataQuery,
    ];
    const preparedContract = {
      address: axiom.getAxiomQueryAddress() as `0x${string}`,
      abi: axiom.getAxiomQueryAbi(),
      functionName: "sendQuery",
      value: BigInt(payment),
      args: queryArgs,
    };
    const { hash } = await writeContract(preparedContract);

    console.log(`Sent query: https://goerli.etherscan.io/tx/${hash}`);
  };

  const resetBuiltQuery = () => {
    reset();
    setIsBuilt(false);
    setIsLoading(false);
  }

  const builtOrLoading = isBuilt || isLoading;

  return (
    <div className="flex flex-col p-4 gap-2">
      <ConnectKitButton />
      <div className="flex flex-row gap-2">
        <button
          onClick={generateAndSendQuery}
          disabled={builtOrLoading}
          className={`${builtOrLoading ? "bg-gray-200 hover:border-bg-gray-200" : "" } border py-2 px-4 hover:border-black`}
        >
          { isBuilt ? "Query already built" : (isLoading ? "Loading..." : "Build and Send Query (on Goerli)") }
        </button>
        <button
          onClick={() => resetBuiltQuery()}
          className={`${ builtOrLoading ? "" : "hidden" } border py-2 px-4 hover:border-black`}
        >
          {"Reset"}
        </button>
      </div>
      <div className="flex flex-col">
        <label htmlFor="input" className="mr-2">
          Circuit inputs:
        </label>
        <textarea
          id="input"
          value={input}
          onChange={e => setInput(e.target.value)}
          className="px-1 border rounded"
        />
      </div>

      <div className="flex flex-col gap-2">
        <div className="flex flex-col">
          <label htmlFor="target" className="mr-2">
            Callback Address:
          </label>
          <textarea
            id="target"
            value={target}
            onChange={e => setTarget(e.target.value)}
            className="px-1 border rounded"
          />
        </div>
        <div className="flex flex-col">
          <label htmlFor="extraData" className="mr-2">
            Callback Extra Data:
          </label>
          <textarea
            id="extraData"
            value={extraData}
            onChange={e => setExtraData(e.target.value)}
            className="px-1 border rounded"
          />
        </div>
      </div>

      <div className="semi-bold">Open Developer Console to see logs and outputs!</div>
    </div>
  );
}
