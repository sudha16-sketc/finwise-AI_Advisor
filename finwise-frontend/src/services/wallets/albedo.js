export const albedoWallet = {
  id: "albedo",
  name: "Albedo",
  async isInstalled() {
    return true; // Web wallet
  },
  async connect() {
    const url = `https://albedo.link/public-key`;
    const popup = window.open(url, "_blank");
    return new Promise((resolve) => {
      window.addEventListener("message", (event) => {
        if (event.origin.includes("albedo")) {
          resolve(event.data.pubkey);
          popup.close();
        }
      });
    });
  },
};