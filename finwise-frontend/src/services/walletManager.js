

import { StellarWalletsKit, SwkAppDarkTheme } from "@creit-tech/stellar-wallets-kit";
import { defaultModules } from "@creit-tech/stellar-wallets-kit/modules/utils";
import { WalletConnectModule } from "@creit-tech/stellar-wallets-kit/modules/wallet-connect";
import { KitEventType } from "@creit-tech/stellar-wallets-kit/types";
import { Networks } from "@stellar/stellar-sdk";

// ── 1. Guard: only run in browser ─────────────────────────────────────────────
if (typeof window === "undefined") {
  throw new Error("walletManager must be imported in a browser environment only.");
}

// ── 2. WalletConnect config ───────────────────────────────────────────────────
const WC_PROJECT_ID = import.meta.env.VITE_WALLETCONNECT_PROJECT_ID ?? "";
const APP_URL = window.location.origin;

const wcModule = WC_PROJECT_ID
  ? new WalletConnectModule({
      projectId: WC_PROJECT_ID,
      metadata: {
        name: "FinWise",
        description: "FinWise Stellar DApp",
        icons: [`${APP_URL}/logo.png`],
        url: APP_URL,
      },
    })
  : null;

// ── 3. Initialise the kit (registers Web Components in the DOM) ───────────────
//
// IMPORTANT: StellarWalletsKit.init() must be called ONCE, at module load time.
// It registers the custom elements (<swk-app-modal> etc.) that the modal needs.
// Without this, authModal() opens nothing.
//
StellarWalletsKit.init({
  theme: SwkAppDarkTheme,          // provides the modal's styles — required
  modules: [
    ...defaultModules(),           // Freighter, Albedo, xBull, Rabet, Lobstr, Hana
    ...(wcModule ? [wcModule] : []),
  ],
  authModal: {
    hideUnsupportedWallets: true,  // hides extension wallets on mobile
    showInstallLabel: false,
  },
});

// ── 4. Public API ─────────────────────────────────────────────────────────────

/**
 * Open the kit's built-in wallet-picker modal (and WalletConnect QR if configured).
 * The modal is a Web Component injected into document.body by the kit.
 *
 * @returns {Promise<string>} Stellar public key (G…)
 */
export async function openWalletModal() {
  const { address } = await StellarWalletsKit.authModal();
  if (!address) throw new Error("No address returned from wallet");
  return address;
}

/**
 * Returns the currently connected address, or null if disconnected.
 * Safe to call on page load to restore a previous session.
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
 * Sign a Stellar transaction XDR with the active wallet.
 *
 * @param {string} xdr
 * @param {string} [address]           - defaults to currently connected address
 * @param {string} [networkPassphrase] - defaults to Networks.PUBLIC
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
 * Disconnect the active wallet session.
 */
export function disconnect() {
  StellarWalletsKit.disconnect();
}

/**
 * Subscribe to kit events: STATE_UPDATED, WALLET_SELECTED, DISCONNECT.
 * Returns an unsubscribe function — call it in useEffect cleanup.
 *
 * @param {(event: KitEvent) => void} callback
 * @returns {() => void}
 */
export function onKitEvent(callback) {
  const unsubs = [
    StellarWalletsKit.on(KitEventType.STATE_UPDATED, callback),
    StellarWalletsKit.on(KitEventType.WALLET_SELECTED, callback),
    StellarWalletsKit.on(KitEventType.DISCONNECT, callback),
  ];
  return () => unsubs.forEach((fn) => fn());
}

export { StellarWalletsKit, KitEventType };