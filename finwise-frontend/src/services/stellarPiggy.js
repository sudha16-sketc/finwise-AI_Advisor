import {
  Address,
  Contract,
  rpc,
  TransactionBuilder,
  Networks,
  BASE_FEE,
  xdr,
} from "@stellar/stellar-sdk";
import { nativeToScVal,scValToNative  } from "@stellar/stellar-sdk";
import * as freighter from "@stellar/freighter-api";


const server = new rpc.Server("https://soroban-testnet.stellar.org");

const CONTRACT_ID = "CDPEPG5VFKOAVKLPF47HWEQIC2LJ353RECGOPW4OKQWDU2T62MDVGGCT";
// Convert number → i128

const DECIMALS = 7;

function toI128(value) {
  const scaled = BigInt(Math.floor(value * 10 ** DECIMALS));
  return nativeToScVal(scaled, { type: "i128" });
}

export async function deposit(amount) {
  const addr = await freighter.getAddress();
  const publicKey = typeof addr === "string" ? addr : addr.address;

  if (!publicKey) {
    throw new Error("Freighter wallet not connected");
  }

  const account = await server.getAccount(publicKey);
  const contract = new Contract(CONTRACT_ID);

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      contract.call(
        "deposit",
        Address.fromString(publicKey).toScVal(),
        toI128(amount),
      ),
    )
    .setTimeout(30)
    .build();

  const prepared = await server.prepareTransaction(tx);

  const signed = await freighter.signTransaction(prepared.toXDR(), {
    networkPassphrase: Networks.TESTNET,
  });

  const signedTx = TransactionBuilder.fromXDR(
    signed.signedTxXdr,
    Networks.TESTNET,
  );

  const response = await server.sendTransaction(signedTx);

  if (response.status === "PENDING") {
    let txResponse;
    do {
      txResponse = await server.getTransaction(response.hash);
    } while (txResponse.status === "NOT_FOUND");

    return txResponse;
  }

  return response;
}

export async function getStats() {
  const addr = await freighter.getAddress();
  const publicKey = typeof addr === "string" ? addr : addr.address;

  if (!publicKey) {
    throw new Error("Freighter wallet not connected");
  }

  const contract = new Contract(CONTRACT_ID);
  const account = await server.getAccount(publicKey);

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(
      contract.call(
        "get_user_stats",
        Address.fromString(publicKey).toScVal()
      )
    )
    .setTimeout(30)
    .build();

  const sim = await server.simulateTransaction(tx);

  if (!sim.result?.retval) return null;

  // 🔥 THIS IS THE IMPORTANT PART
  return scValToNative(sim.result.retval);
}