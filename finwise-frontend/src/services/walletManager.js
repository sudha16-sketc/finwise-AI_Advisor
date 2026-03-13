/**
 * walletManager.js
 *
 * Thin wrapper around the Stellar Wallets Kit (v2).
 * No per-wallet code — the kit handles Freighter, Albedo, xBull,
 * WalletConnect (mobile QR), Rabet, Hana, Lobstr, and more automatically.
 *
 * Install:
 *   npx jsr add @creit-tech/stellar-wallets-kit
 *
 * WalletConnect project ID:
 *   Free at https://cloud.walletconnect.com  →  add to .env as
 *   VITE_WALLETCONNECT_PROJECT_ID=xxxx
 */

import { StellarWalletsKit } from "@creit-tech/stellar-wallets-kit/sdk";
import { defaultModules } from "@creit-tech/stellar-wallets-kit/modules/utils";
import { WalletConnectModule } from "@creit-tech/stellar-wallets-kit/modules/wallet-connect";
import { KitEventType } from "@creit-tech/stellar-wallets-kit/types";
import { Networks } from "@stellar/stellar-sdk";

// ── Initialise once at module load time ───────────────────────────────────────

const WC_PROJECT_ID = import.meta.env.VITE_WALLETCONNECT_PROJECT_ID ?? "";
const APP_URL       = window.location.origin;

StellarWalletsKit.init({
  // defaultModules() bundles Freighter, Albedo, xBull, Rabet, Hana, Lobstr…
  modules: [
    ...defaultModules(),

    // WalletConnect adds mobile-wallet support via QR code.
    // Remove this block if you don't have a project ID yet.
    ...(WC_PROJECT_ID
      ? [
          new WalletConnectModule({
            projectId: WC_PROJECT_ID,
            metadata: {
              name:        "FinWise",
              description: "FinWise Stellar DApp",
              icons:       [`${APP_URL}/logo.png`],
              url:          APP_URL,
            },
          }),
        ]
      : []),
  ],

  // Hide wallets that can't work in the current environment.
  // On mobile browsers, extension wallets (Freighter, Rabet, xBull) are
  // never injected into window, so the kit correctly treats them as
  // unsupported and will hide them rather than showing an "Install" prompt.
  authModal: {
    hideUnsupportedWallets: true,
    showInstallLabel:       false,
  },
});

// ── Public API ────────────────────────────────────────────────────────────────

/**
 * Open the kit's built-in wallet-picker modal.
 * Resolves with the Stellar public key (G…) once the user connects.
 *
 * @returns {Promise<string>} publicKey
 */
export async function openWalletModal() {
  const { address } = await StellarWalletsKit.authModal();
  if (!address) throw new Error("No address returned from wallet");
  return address;
}

/**
 * Get the currently connected address without opening any UI.
 * Returns null if no wallet is connected.
 *
 * @returns {Promise<string|null>}
 */
export async function getConnectedAddress() {
  try {
    const { address } = await StellarWalletsKit.getAddress();
    return address ?? null;
  } catch {
    return null;
  }
}

/**
 * Sign a Stellar transaction XDR with the connected wallet.
 *
 * @param {string} xdr                  – base-64 transaction envelope XDR
 * @param {string} [address]            – signer address (defaults to connected)
 * @param {string} [networkPassphrase]  – defaults to Networks.PUBLIC
 * @returns {Promise<string>} signedTxXdr
 */
export async function signTransaction(
  xdr,
  address,
  networkPassphrase = Networks.PUBLIC
) {
  const signerAddress = address ?? (await getConnectedAddress());
  if (!signerAddress) throw new Error("No wallet connected");

  const { signedTxXdr } = await StellarWalletsKit.signTransaction(xdr, {
    networkPassphrase,
    address: signerAddress,
  });
  return signedTxXdr;
}

/**
 * Disconnect the active wallet and clear kit state.
 */
export function disconnect() {
  StellarWalletsKit.disconnect();
}

/**
 * Subscribe to kit state changes (address updates, disconnects, wallet swaps).
 *
 * @param {(event: KitEvent) => void} callback
 * @returns {() => void} unsubscribe function
 *
 * @example
 *   const unsub = onKitEvent(e => {
 *     if (e.eventType === KitEventType.DISCONNECT) setPublicKey(null);
 *     if (e.eventType === KitEventType.STATE_UPDATED) setPublicKey(e.payload.address);
 *   });
 *   // cleanup: unsub();
 */
export function onKitEvent(callback) {
  const unsubs = [
    StellarWalletsKit.on(KitEventType.STATE_UPDATED,   callback),
    StellarWalletsKit.on(KitEventType.WALLET_SELECTED, callback),
    StellarWalletsKit.on(KitEventType.DISCONNECT,      callback),
  ];
  return () => unsubs.forEach((fn) => fn());
}

// Re-export for callers that need low-level kit access (e.g. signAuthEntry).
export { StellarWalletsKit, KitEventType };