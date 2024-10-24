import * as anchor from "@coral-xyz/anchor";
import { BankrunProvider } from "anchor-bankrun";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN, Program } from "@coral-xyz/anchor";

import {
  startAnchor,
  Clock,
  BanksClient,
  ProgramTestContext,
} from "solana-bankrun";

import {
  createAssociatedTokenAccount,
  createMint,
  mintTo,
} from "spl-token-bankrun";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

import IDL from "../target/idl/trust_lock.json";
import { TrustLock } from "../target/types/trust_lock";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { assert, expect } from "chai";

describe("Trust Lock Tests", () => {
  let context: ProgramTestContext;
  let userProvider: BankrunProvider;
  let provider: BankrunProvider;
  let initialize_trustlock_configuration: PublicKey;
  let create_trustlock_account: PublicKey;
  let create_vault_state: PublicKey;
  let token_vault: PublicKey;
  let create_order_account: PublicKey;
  let program: Program<TrustLock>;
  let admin: Keypair;
  let user: Keypair;
  let mint: PublicKey;
  let banksClient: BanksClient;
  let mint_whitelist = [
    new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
    new PublicKey("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
  ];
  let updated_mint_whitelist = [
    new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
    new PublicKey("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
    new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
  ];
  let index = 0;

  beforeEach(async () => {
    const adminPrivateKey = Uint8Array.from([
      236, 80, 202, 29, 108, 93, 60, 166, 32, 159, 20, 143, 219, 66, 78, 8, 88,
      241, 128, 37, 95, 95, 159, 217, 68, 144, 80, 179, 7, 61, 21, 12, 230, 99,
      143, 181, 243, 42, 75, 81, 145, 153, 83, 95, 32, 240, 19, 51, 193, 207,
      79, 246, 230, 79, 18, 28, 73, 0, 241, 56, 158, 215, 86, 56,
    ]);
    admin = Keypair.fromSecretKey(adminPrivateKey);
    user = new anchor.web3.Keypair();

    // Initialize the context first
    context = await startAnchor(
      "",
      [{ name: "trust_lock", programId: new PublicKey(IDL.address) }],
      [
        {
          address: user.publicKey,
          info: {
            lamports: 4_000_000_000,
            data: Buffer.alloc(0),
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
          },
        },
        {
          address: admin.publicKey,
          info: {
            lamports: 4_000_000_000,
            data: Buffer.alloc(0),
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
          },
        },
      ]
    );

    banksClient = context.banksClient;
    mint = await createMint(banksClient, admin, admin.publicKey, null, 9);

    console.log("Mint : ", mint.toBase58());

    // Comment this if you want to check unsupported tokens
    mint_whitelist = [
      new PublicKey("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
      new PublicKey("EKpQGSJtjMFqKZ9KQanSqYXRcF8fBopzLHYxdM65zcjm"),
      new PublicKey(mint.toBase58()),
    ];

    // Initialize the provider after context
    provider = new BankrunProvider(context);

    // Initialize the program after the provider
    program = new Program<TrustLock>(IDL as TrustLock, provider); // Ensure program is initialized here

    // Set the provider's wallet to the admin's wallet
    anchor.setProvider(
      new anchor.AnchorProvider(provider.connection, new NodeWallet(admin), {
        commitment: "confirmed",
      })
    );

    userProvider = new BankrunProvider(context);
    userProvider.wallet = new NodeWallet(user);

    console.log("User After : ", user.publicKey.toBase58());

    [initialize_trustlock_configuration] = PublicKey.findProgramAddressSync(
      [Buffer.from("Config_Initialized"), Buffer.from([index])],
      program.programId
    );

    [create_trustlock_account] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("TrustLock_Account"),
        Buffer.from(user.publicKey.toBuffer()),
      ],
      program.programId
    );

    [create_vault_state] = PublicKey.findProgramAddressSync(
      [Buffer.from("Create_Vault"), Buffer.from(admin.publicKey.toBuffer())],
      program.programId
    );

    [token_vault] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("Create_Vault"),
        Buffer.from(admin.publicKey.toBuffer()),
        Buffer.from(mint.toBuffer()),
      ],
      program.programId
    );

    // Initialize Config Account
    let tx = await program.methods
      .initializeTrustlockConfiguration(index, mint_whitelist)
      .accounts({
        trustlockConfigAccount: initialize_trustlock_configuration,
      })
      .signers([admin])
      .rpc({ commitment: "confirmed" });

    console.log(tx);

    const trust_lock_config_data = await program.account.trustLockConfig.fetch(
      initialize_trustlock_configuration,
      "confirmed"
    );

    const orderIdBuffer = Buffer.alloc(8);
    orderIdBuffer.writeBigUInt64LE(
      BigInt(trust_lock_config_data.orderId.toNumber())
    );

    [create_order_account] = PublicKey.findProgramAddressSync(
      [Buffer.from("Create_Order"), Buffer.from(orderIdBuffer)],
      program.programId
    );
  });

  it("Admin Should Initialize Config Account", async () => {
    const trust_lock_config_data = await program.account.trustLockConfig.fetch(
      initialize_trustlock_configuration,
      "confirmed"
    );
    console.log(
      "Vesting Account Data:",
      JSON.stringify(trust_lock_config_data)
    );

    expect(trust_lock_config_data.configIndex).to.equal(0);
    expect(trust_lock_config_data.orderId.toNumber()).to.equal(0);
    expect(trust_lock_config_data.mintWhitelist[0].toBase58()).to.equal(
      "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"
    );
  });

  it("Should Update the mint_whitelist", async () => {
    let tx = await program.methods
      .updateWhitelist(updated_mint_whitelist)
      .accounts({
        trustlockConfigAccount: initialize_trustlock_configuration,
      })
      .signers([admin])
      .rpc({ commitment: "confirmed" });

    const trust_lock_config_data = await program.account.trustLockConfig.fetch(
      initialize_trustlock_configuration,
      "confirmed"
    );

    console.log("Account Data:", JSON.stringify(trust_lock_config_data));

    console.log("Initialized Configuration Account : ", tx);

    expect(trust_lock_config_data.mintWhitelist[2].toBase58()).to.equal(
      "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
    );
  });

  it("Should Create TrustLock Account", async () => {
    // Use banksClient to get the user's balance
    const userBalanceBefore = await context.banksClient.getBalance(
      user.publicKey
    );
    console.log("User Balance Before : ", userBalanceBefore);

    console.log("Signers for transaction:", user.publicKey.toBase58());

    const userBalance = await context.banksClient.getBalance(user.publicKey);

    console.log("User Balance After : ", userBalance);

    let tx = await program.methods
      .createTrustlockAccount()
      .accounts({
        signer: user.publicKey,
        trustLockConfigAccount: initialize_trustlock_configuration,
        createTrustlockAccount: create_trustlock_account,
      })
      .signers([user])
      .rpc({ commitment: "confirmed" });

    const trust_lock_account_data =
      await program.account.createTrustLockAccountState.fetch(
        create_trustlock_account,
        "confirmed"
      );
    console.log("Account Data:", JSON.stringify(trust_lock_account_data));

    expect(trust_lock_account_data.holder.toBase58()).to.equal(
      user.publicKey.toBase58()
    );

    expect(trust_lock_account_data.accountNo.toNumber()).to.equal(0);
  });

  it("Admin should create a vault for the token", async () => {
    let tx = await program.methods
      .createVault()
      .accounts({
        trustlockConfigAccount: initialize_trustlock_configuration,
        tokenMint: mint,
        tokenVault: token_vault, // Pass the associated token account
        createVaultState: create_vault_state,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc({ commitment: "confirmed" });

    console.log("Transaction: ", tx);

    // Fetch and verify the vault state
    const vaultState = await program.account.createVaultState.fetch(
      create_vault_state
    );

    expect(vaultState.tokenMint.toBase58()).to.equal(mint.toBase58());
  });

  it("It Should Create Order", async () => {
    let tx1 = await program.methods
      .createVault()
      .accounts({
        trustlockConfigAccount: initialize_trustlock_configuration,
        tokenMint: mint,
        tokenVault: token_vault, // Pass the associated token account
        createVaultState: create_vault_state,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc({ commitment: "confirmed" });

    let tx2 = await program.methods
      .createTrustlockAccount()
      .accounts({
        signer: user.publicKey,
        trustLockConfigAccount: initialize_trustlock_configuration,
        createTrustlockAccount: create_trustlock_account,
      })
      .signers([user])
      .rpc({ commitment: "confirmed" });

    // Here is the new code
    const userTokenAccount = await createAssociatedTokenAccount(
      banksClient,
      user, // Payer
      mint, // Mint
      user.publicKey // Owner of the new token account
    );

    await mintTo(
      banksClient,
      admin, // Payer (admin mints the tokens)
      mint, // Mint
      userTokenAccount, // Destination (user's token account)
      admin, // Authority (admin can mint tokens)
      1000 * anchor.web3.LAMPORTS_PER_SOL // Mint amount
    );

    const tx = await program.methods
      .createOrder(
        index,
        "Sample Demand",
        null,
        new anchor.BN(100 * anchor.web3.LAMPORTS_PER_SOL)
      )
      .accounts({
        signer: user.publicKey,
        createOrderAccount: create_order_account,
        userTokenAccount: userTokenAccount,
        tokenMint: mint,
        tokenVaultAccount: token_vault,
        trustlockConfigAccount: initialize_trustlock_configuration,
        trustlockAccount: create_trustlock_account,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Transaction in create order : ", tx);

    const order_account_data = await program.account.createOrderAccount.fetch(
      create_order_account,
      "confirmed"
    );

    const user_trust_lock_account =
      await program.account.createTrustLockAccountState.fetch(
        create_trustlock_account,
        "confirmed"
      );

    console.log("Order Account Data:", JSON.stringify(order_account_data));

    console.log(
      "User TrustLock Account Data:",
      JSON.stringify(user_trust_lock_account)
    );

    expect(order_account_data.orderId.toNumber()).to.equal(0); // Verify Order ID
    expect(order_account_data.createdBy.toBase58()).to.equal(
      user.publicKey.toBase58()
    );
    expect(order_account_data.amount.toNumber()).to.equal(100000000000);

    expect(user_trust_lock_account.accountNo.toNumber()).to.equal(0);
  });

  it("Pitch for Order", async () => {});
});
