// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { expect } from "chai";
// import { PublicKey, Keypair } from "@solana/web3.js";
// import { TrustLock } from "../target/types/trust_lock";

// describe("sol-trust", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   //Fetch the Program now
//   const program = anchor.workspace.TrustLock as Program<TrustLock>;

//   const adminPrivateKey = Uint8Array.from([
//     236, 80, 202, 29, 108, 93, 60, 166, 32, 159, 20, 143, 219, 66, 78, 8, 88,
//     241, 128, 37, 95, 95, 159, 217, 68, 144, 80, 179, 7, 61, 21, 12, 230, 99,
//     143, 181, 243, 42, 75, 81, 145, 153, 83, 95, 32, 240, 19, 51, 193, 207, 79,
//     246, 230, 79, 18, 28, 73, 0, 241, 56, 158, 215, 86, 56,
//   ]);
//   let admin = Keypair.fromSecretKey(adminPrivateKey);

//   let index = 0;
//   let mint_whitelist = [
//     new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
//     new PublicKey("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
//   ];
//   let user = Keypair.generate();

//   let [initialize_trustlock_configuration] = PublicKey.findProgramAddressSync(
//     [Buffer.from("Config_Initialized"), Buffer.from([index])],
//     program.programId
//   );

//   let [create_trustlock_account] = PublicKey.findProgramAddressSync(
//     [Buffer.from("TrustLock_Account"), Buffer.from(user.publicKey.toBuffer())],
//     program.programId
//   );

//   it("Is Initialized By Admin in Practice!", async () => {
//     //Let's define the Transaction
//     let tx = await program.methods
//       .initializeTrustlockConfiguration(index, mint_whitelist)
//       .accounts({
//         admin: admin.publicKey,
//         trustlockConfigAccount: initialize_trustlock_configuration,
//       })
//       .signers([admin])
//       .rpc({ commitment: "confirmed" });

//     console.log("Transaction Signature : ", tx);

//     const configAccount = await program.account.trustLockConfig.fetch(
//       initialize_trustlock_configuration
//     );

//     console.log(configAccount);
//   });

//   it("Should Create Trust Lock Account : ", async () => {
//     const airdropSig = await provider.connection.requestAirdrop(
//       user.publicKey,
//       2 * anchor.web3.LAMPORTS_PER_SOL
//     );

//     const latestBlockHash = await provider.connection.getLatestBlockhash();
//     const confirmationConfig = {
//       blockhash: latestBlockHash.blockhash,
//       lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
//       signature: airdropSig,
//     };

//     await provider.connection.confirmTransaction(confirmationConfig);

//     let tx = await program.methods
//       .createTrustlockAccount()
//       .accounts({
//         signer: user.publicKey,
//         trustLockConfigAccount: initialize_trustlock_configuration,
//         createTrustlockAccount: create_trustlock_account,
//       })
//       .signers([user])
//       .rpc({ commitment: "confirmed" });

//     console.log(tx);

//     const TrustLockAccount =
//       await program.account.createTrustLockAccountState.fetch(
//         create_trustlock_account
//       );

//     console.log("Account No : ", TrustLockAccount.accountNo);

//     expect(TrustLockAccount.accountNo.toNumber()).to.equal(0);
//   });
// });
