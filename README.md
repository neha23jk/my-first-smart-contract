# 🏆 Achievement Vault: A Collaborative Commitment Device

**Achievement Vault** is a Soroban-based smart contract designed to help users reach their financial goals through a "lock-and-key" savings mechanism. It serves as a decentralized commitment device: you set a goal, your community helps you reach it, and the blockchain ensures you don't spend it until the time is right.

---

## 🛠 What the Contract Does
The contract acts as a secure, on-chain vault for saving assets (like XLM or USDC).

* **State Initialization:** Uses a custom `Vault` struct and `instance` storage to lock in the saver's address, the target asset, the goal amount, and the unlock time.
* **Access Control:** Implements `saver.require_auth()` to ensure that **only** the designated saver can trigger the final withdrawal.
* **Secure Deposits:** Uses the `token::Client` to handle cross-contract calls, allowing anyone to contribute to the vault.
* **Custom Error Handling:** Includes a robust `Error` enum to provide clear feedback (e.g., `TooEarly` or `GoalNotReached`).
* **Safety First:** Logic ensures that funds can never be "trapped" if the goal is met and time has passed.

---

## 🎨 Why the Design was Chosen
* **Friction as a Feature:** Most DeFi apps focus on speed; this contract focuses on **discipline**.
* **Simplicity:** By using a "Dual-Lock" (Time + Amount), it provides a clear real-world analogy of a locked piggy bank.
* **Creativity:** It transforms a simple wallet into a psychological commitment tool.

---

## 🔄 How State Changes Work
1. **Initialize:** The contract state is set to **Active** with a specific goal and timestamp.
2. **Deposit:** The balance state increases as the `token::Client` transfers funds into the vault.
3. **Validate:** Upon withdrawal, the contract checks if `current_time > unlock_time` **AND** `current_balance > goal`.
4. **Complete:** Funds are transferred and the vault state is cleared.

---

## 🛡 Security Checks Implemented
* **Auth Check:** Strict `require_auth` ensures only the owner can withdraw.
* **Re-init Guard:** Prevents changing the goal once the vault is started.
* **Ledger Truth:** Uses `env.ledger().timestamp()` so users cannot "fake" the time.

---

## 🚀 Deployed Link
* **Network:** Stellar Testnet
* **Contract ID:** `CBEYHXY22YKX6KPNGC3XU7CH2JG6V7XXWGS2ALZFBRDWQUFTLKYBIVMK`
* **Wasm Hash:** `f6c89853ddfb854c8b19821fb16ecac8a4ae50852bb5cc900fb4828eb73b84cf`
* **Stellar Expert:** [View Contract on Stellar Expert](https://stellar.expert/explorer/testnet/tx/965e9ff239c8bb99542e4d2609be975b30b5f3958ab0575cf1f6a63f0afb353e)
* **Stellar Lab:** [View Contract on Stellar Lab](https://lab.stellar.org/r/testnet/contract/CBEYHXY22YKX6KPNGC3XU7CH2JG6V7XXWGS2ALZFBRDWQUFTLKYBIVMK).   

---

## 🧪 Security Verification (Tests)
* ✅ **Success:** Saver withdraws after time expires and goal is met.
* ❌ **Failure:** Stranger tries to withdraw (Access Control check).
* ❌ **Failure:** Saver tries to withdraw before the timestamp (Time Lock check).
* ❌ **Failure:** Saver tries to withdraw before the goal is met (Goal Threshold check).

---

## 🗺 Roadmap
* **Multi-Sig Support:** Require a "Buddy" to approve withdrawals.
* **Partial Goal Exit:** Allow a percentage withdrawal if time has passed but the goal was missed.