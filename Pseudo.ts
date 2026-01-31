await program.methods
  .update(new anchor.BN(42))
  .accounts({
    state,
    authority: wallet.publicKey,
  })
  .rpc();
