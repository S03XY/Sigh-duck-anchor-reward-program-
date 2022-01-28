import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Sighduck } from "../target/types/sighduck";
import * as bs58 from "bs58";

describe("sighduck", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());
  // @ts-ignore
  const program = anchor.workspace.Sighduck as Program<Sighduck>;

  it("Interaction with program!", async () => {
    // Add your test here.

    const _wallet = anchor.Provider.env().wallet;
    const rewardAccount = anchor.web3.Keypair.generate();

    const _mintid = "2MGzbVhVmw6dQjtTUBsMxAT8n6jftcmbdrWitKfuceRF";

    const tx = await program.rpc.initialize(_mintid, new anchor.BN(0), {
      accounts: {
        rewardAccount: rewardAccount.publicKey,
        user: _wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [rewardAccount],
    });
    console.log("account initilized", tx);

    console.log("updating accounts....");

    let s_key = bs58.decode(
      "67nQFtnfR4BuX13u2mwusQQwbewGH6S19T8BvvcoFJ56fZoVYfR86epfZKhWa5cCbtspE2dBWmx5tjkVcmSpQWzS"
    );

    let secret_key = Uint8Array.from(s_key);
    let updater = anchor.web3.Keypair.fromSecretKey(secret_key);
    console.log(updater.publicKey.toBase58());

    const update_ins = await program.rpc.update(true, {
      accounts: {
        rewardAccount: rewardAccount.publicKey,
        updator: updater.publicKey,
      },
      signers: [updater],
    });







    // let connection = anchor.Provider.env().connection;
    // let transaction = new anchor.web3.Transaction().add(update_ins);
    // transaction.feePayer = updater.publicKey;
    // let recentBlockhash = await connection.getRecentBlockhash();
    // transaction.recentBlockhash = recentBlockhash.blockhash;

    // let update_tx = await connection.sendTransaction(transaction, [updater]);

    console.log("update", update_ins);


    let programid = new anchor.web3.PublicKey("5syVwzbtS5UsikRbPXuiZBvjv4Hy7QPJmDUhSctjzQ8h")

    let [program_Wallet,bump] = await anchor.web3.PublicKey.findProgramAddress([Buffer.from("shashank")],programid)

    console.log("program wallet",program_Wallet.toBase58())











  });
});
