import {
  Keypair,
  Connection,
  clusterApiUrl,
  Commitment,
  PublicKey,
  Transaction,
} from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import {
  createCreateMetadataAccountV3Instruction,
  DataV2,
} from "@metaplex-foundation/mpl-token-metadata";
import fs from "fs";
import os from "os";
import path from "path";

(async () => {
  try {
    // Load your Solana wallet from ~/.config/solana/id.json
    const walletPath = path.join(os.homedir(), ".config", "solana", "id.json");
    const secretKey = JSON.parse(fs.readFileSync(walletPath, "utf8"));
    const keypair = Keypair.fromSecretKey(Uint8Array.from(secretKey));

    const commitment: Commitment = "confirmed";
    const connection = new Connection(clusterApiUrl("devnet"), commitment);

    console.log("Wallet Public Key: 7sNC3GF34zrF2QEcBvsDpYStjUrsQnqSHTpBcYTviGgM");

    // 1. Create the Mint (9 decimals)
    const mint = await createMint(
      connection,
      keypair,
      keypair.publicKey,
      null,
      9
    );

    console.log("Token Minted:", mint.toBase58());

    // 2. Create/get associated token account for payer and mint 100 tokens
    const payerATA = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mint,
      keypair.publicKey
    );

    console.log("Payer ATA:", payerATA.address.toBase58());

    const amount = BigInt(100) * BigInt(10 ** 9); // 100 tokens
    const mintSig = await mintTo(
      connection,
      keypair,
      mint,
      payerATA.address,
      keypair,
      Number(amount)
    );

    console.log("Minted 100 GLXB tokens. Transaction:", mintSig);

    // 3. Create Metadata using Metaplex
    const metadataProgramId = new PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

    const [metadataPDA] = await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        metadataProgramId.toBuffer(),
        mint.toBuffer(),
      ],
      metadataProgramId
    );

    const tokenData: DataV2 = {
      name: "GalaxyBucks",
      symbol: "GLXB",
      uri: "https://raw.githubusercontent.com/nabikp/TurbinePB_Q425_nabikp/main/adv_task01/token-mint/metadata.json",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    };

    const metadataIx = createCreateMetadataAccountV3Instruction(
      {
        metadata: metadataPDA,
        mint,
        mintAuthority: keypair.publicKey,
        payer: keypair.publicKey,
        updateAuthority: keypair.publicKey,
      },
      {
        createMetadataAccountArgsV3: {
          data: tokenData,
          isMutable: true,
          collectionDetails: null,
        },
      }
    );

    // Build and send transaction for metadata
    const tx = new Transaction().add(metadataIx);
    const txSig = await connection.sendTransaction(tx, [keypair], {
      skipPreflight: false,
      preflightCommitment: commitment,
    });

    await connection.confirmTransaction(txSig, commitment);
    console.log("Metadata Created. Transaction:", txSig);

    // Final summary
    console.log("\nToken Mint Successful!");
    console.log("Mint Address:", mint.toBase58());
    console.log("Owner Public Key:", keypair.publicKey.toBase58());
    console.log("Token Account:", payerATA.address.toBase58());
  } catch (e) {
    console.error("Error:", e);
  }
})();
