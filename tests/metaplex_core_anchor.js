const anchor = require("@coral-xyz/anchor");

describe("metaplex_core_anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("creates a collection and an asset via mpl-core CPI", async () => {
    const program = anchor.workspace.metaplexCoreAnchor;
    const payer = provider.wallet;
    const collection = anchor.web3.Keypair.generate();
    const asset = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(
        payer.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    );

    await program.methods
      .createCollection("Test Collection", "https://example.com/collection.json")
      .accounts({
        collection: collection.publicKey,
        payer: payer.publicKey,
        updateAuthority: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCore: new anchor.web3.PublicKey(
          "CoREENxbYwzQxHzNCvh2fYsG9kB1iG2zAn7SrPjzYh1A"
        ),
      })
      .signers([collection])
      .rpc();

    await program.methods
      .createAsset("Test Asset", "https://example.com/asset.json")
      .accounts({
        asset: asset.publicKey,
        collection: collection.publicKey,
        payer: payer.publicKey,
        owner: payer.publicKey,
        updateAuthority: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCore: new anchor.web3.PublicKey(
          "CoREENxbYwzQxHzNCvh2fYsG9kB1iG2zAn7SrPjzYh1A"
        ),
      })
      .signers([asset])
      .rpc();
  });
});
