
import { albedoWallet } from "./wallets/albedo";
import { xbullWallet } from "./wallets/xbull";

export const wallets = [

  albedoWallet,
  xbullWallet,
];

export async function connectWithWallet(walletId) {
  const wallet = wallets.find((w) => w.id === walletId);
  if (!wallet) throw new Error("Wallet not found");

  const installed = await wallet.isInstalled();
  if (!installed) throw new Error(`${wallet.name} not installed`);

  return wallet.connect();
}