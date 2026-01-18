### 🛠️ How it Works: The Achievement Vault

While many vaults are built for simple storage, the Achievement Vault is a creative application of 'Decentralized Discipline.' It uses the unchangeable nature of the Stellar Ledger to help users overcome impulsive spending by locking funds behind both a financial milestone and a time-based deadline.

Think of this contract as a digital piggy bank with a brain. Here is the technical breakdown for anyone joining the project:

* **The Big Idea:** We aren't just storing money; we are enforcing a goal. The contract is designed to stay "locked" until you hit a specific date AND a specific savings target.
* **The "Brain" (Storage):** When you start a vault, we save your goal (how much) and your deadline (when) directly onto the blockchain. Once these are set, they are the "law" for that vault.
* **The Double Lock:** In the code, we use a simple `if` statement. If the clock says it's too early, OR the balance is too low, the `withdraw` button simply won't work. The transaction will fail and tell you exactly why.
* **ID Check (Security):** We use `require_auth`. This is like a bouncer at a club checking your ID. Even if the vault is full and the time is up, only the "Saver" who started the vault can take the money out.
* **Talking to Tokens:** The contract doesn't "own" the money itself. It talks to the Stellar Token system. It tells the token: "Hey, move 100 XLM from this person to the vault." It’s safe because it uses the official Stellar standards.
* **Trusting the Clock:** We don't trust the user's phone or computer clock (which can be changed). We use the "Ledger Time," which is the official time synced across the entire global Stellar network.
* **Helping Hands:** The `deposit` function is public. This means your friends or family can send money into your vault to help you hit your goal faster, but our security ensures they can't take it back out!
* **Clear Feedback:** If something goes wrong, the contract sends back a clear error message like `TooEarly` or `GoalNotReached` instead of a confusing computer error code.
