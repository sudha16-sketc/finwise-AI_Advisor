export const xbullWallet = {
  id: "xbull",
  name: "xBull",
  async isInstalled() {
    return window.xBullSDK !== undefined;
  },
  async connect() {
    const res = await window.xBullSDK.connect();
    return res.publicKey;
  },
};