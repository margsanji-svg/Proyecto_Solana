import * as anchor from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.Libros;

async function main() {
  const owner = provider.wallet.publicKey;

  const [usuarioPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("usuario"), owner.toBuffer()],
    program.programId
  );

  await program.methods
    .crearUsuario("Maria")
    .accounts({
      owner,
      usuario: usuarioPDA,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

  console.log("Usuario creado");

  await program.methods
    .agregarLibro("El Principito", "Antoine de Saint-Exupéry")
    .accounts({
      owner,
      usuario: usuarioPDA,
    })
    .rpc();

  console.log("Libro agregado");

  await program.methods
    .marcarLeido(0)
    .accounts({
      owner,
      usuario: usuarioPDA,
    })
    .rpc();

  console.log("Libro marcado como leído");

  await program.methods
    .eliminarLibro(0)
    .accounts({
      owner,
      usuario: usuarioPDA,
    })
    .rpc();

  console.log("Libro eliminado");
}

main();
