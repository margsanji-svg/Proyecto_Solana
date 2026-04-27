describe("libros", () => {
  it("CRUD libros", async () => {
    const usuario = pg.wallet.publicKey;

    const [usuarioPDA] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("usuario"), usuario.toBuffer()],
      pg.program.programId
    );

    await pg.program.methods
      .crearUsuario("Maria")
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    await pg.program.methods
      .agregarLibro("1984", "George Orwell")
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
      })
      .rpc();

    await pg.program.methods
      .marcarLeido(0)
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
      })
      .rpc();

    await pg.program.methods
      .eliminarLibro(0)
      .accounts({
        owner: usuario,
        usuario: usuarioPDA,
      })
      .rpc();

    console.log("CRUD libros completado");
  });
});
