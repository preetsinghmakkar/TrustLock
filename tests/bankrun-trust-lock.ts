// import * as anchor from "@coral-xyz/anchor";
// import { BankrunProvider } from "anchor-bankrun";
// import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
// import { BN, Program } from "@coral-xyz/anchor";

// import {
//   startAnchor,
//   Clock,
//   BanksClient,
//   ProgramTestContext,
// } from "solana-bankrun";

// import { createMint, mintTo } from "spl-token-bankrun";
// import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
// import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

// import IDL from "../target/idl/trust_lock.json";
// import { TrustLock } from "../target/types/trust_lock";
// import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
// import { expect } from "chai";

// describe("Trust Lock Tests", () => {
//   let context: ProgramTestContext;
//   let provider: BankrunProvider;
//   let initialize_trustlock_configuration: PublicKey;
//   let create_trustlock_account: PublicKey;
//   let program: Program<TrustLock>;
//   let admin: Keypair;
//   let user: Keypair;
//   let mint_whitelist = [
//     new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
//     new PublicKey("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
//   ];
//   let updated_mint_whitelist = [
//     new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
//     new PublicKey("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
//     new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
//   ];
//   let index = 0;

//   beforeEach(async () => {
//     const adminPrivateKey = Uint8Array.from([
//       236, 80, 202, 29, 108, 93, 60, 166, 32, 159, 20, 143, 219, 66, 78, 8, 88,
//       241, 128, 37, 95, 95, 159, 217, 68, 144, 80, 179, 7, 61, 21, 12, 230, 99,
//       143, 181, 243, 42, 75, 81, 145, 153, 83, 95, 32, 240, 19, 51, 193, 207,
//       79, 246, 230, 79, 18, 28, 73, 0, 241, 56, 158, 215, 86, 56,
//     ]);
//     admin = Keypair.fromSecretKey(adminPrivateKey); // Create Keypair from known private key

//     user = Keypair.generate();

//     console.log("User : ", user.publicKey.toBase58());

//     program = new Program<TrustLock>(IDL as TrustLock, provider);

//     context = await startAnchor(
//       "",
//       [{ name: "trust_lock", programId: new PublicKey(IDL.address) }],
//       [
//         {
//           address: admin.publicKey,
//           info: {
//             lamports: 4_000_000_000,
//             data: Buffer.alloc(0),
//             owner: SYSTEM_PROGRAM_ID,
//             executable: false,
//           },
//         },
//         {
//           address: user.publicKey,
//           info: {
//             lamports: 4_000_000_000,
//             data: Buffer.alloc(0),
//             owner: SYSTEM_PROGRAM_ID,
//             executable: false,
//           },
//         },
//       ]
//     );

//     provider = new BankrunProvider(context);

//     // Set the provider's wallet to the admin's wallet
//     anchor.setProvider(
//       new anchor.AnchorProvider(provider.connection, new NodeWallet(admin), {
//         commitment: "confirmed",
//       })
//     );

//     [initialize_trustlock_configuration] = PublicKey.findProgramAddressSync(
//       [Buffer.from("Config_Initialized"), Buffer.from([index])],
//       program.programId
//     );

//     [create_trustlock_account] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("TrustLock_Account"),
//         Buffer.from(user.publicKey.toBuffer()),
//       ],
//       program.programId
//     );

//     // Initialize Config Account
//     let tx = await program.methods
//       .initializeTrustlockConfiguration(index, mint_whitelist)
//       .accounts({
//         trustlockConfigAccount: initialize_trustlock_configuration,
//       })
//       .signers([admin])
//       .rpc({ commitment: "confirmed" });

//     console.log(tx);
//   });

//   it("Admin Should Initialize Config Account", async () => {
//     const trust_lock_config_data = await program.account.trustLockConfig.fetch(
//       initialize_trustlock_configuration,
//       "confirmed"
//     );
//     console.log(
//       "Vesting Account Data:",
//       JSON.stringify(trust_lock_config_data)
//     );

//     expect(trust_lock_config_data.configIndex).to.equal(0);
//     expect(trust_lock_config_data.orderId.toNumber()).to.equal(0);
//     expect(trust_lock_config_data.mintWhitelist[0].toBase58()).to.equal(
//       "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"
//     );
//   });

//   it("Should Update the mint_whitelist", async () => {
//     let tx = await program.methods
//       .updateWhitelist(updated_mint_whitelist)
//       .accounts({
//         trustlockConfigAccount: initialize_trustlock_configuration,
//       })
//       .signers([admin])
//       .rpc({ commitment: "confirmed" });

//     const trust_lock_config_data = await program.account.trustLockConfig.fetch(
//       initialize_trustlock_configuration,
//       "confirmed"
//     );

//     console.log("Account Data:", JSON.stringify(trust_lock_config_data));

//     console.log("Initialized Configuration Account : ", tx);

//     expect(trust_lock_config_data.mintWhitelist[2].toBase58()).to.equal(
//       "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
//     );
//   });

//   it("Should Create TrustLock Account", async () => {
//     // Use banksClient to get the user's balance
//     const userBalanceBefore = await context.banksClient.getBalance(
//       user.publicKey
//     );
//     console.log("User Balance Before : ", userBalanceBefore);

//     console.log("Signers for transaction:", user.publicKey.toBase58());

//     const userBalance = await context.banksClient.getBalance(user.publicKey);

//     console.log("User Balance After : ", userBalance);

//     let tx = await program.methods
//       .createTrustlockAccount()
//       .accounts({
//         signer: user.publicKey,
//         trustLockConfigAccount: initialize_trustlock_configuration,
//         createTrustlockAccount: create_trustlock_account,
//       })
//       .signers([user])
//       .rpc({ commitment: "confirmed" });

//     const trust_lock_account_data =
//       await program.account.createTrustLockAccountState.fetch(
//         create_trustlock_account,
//         "confirmed"
//       );
//     console.log("Account Data:", JSON.stringify(trust_lock_account_data));
//   });
// });
