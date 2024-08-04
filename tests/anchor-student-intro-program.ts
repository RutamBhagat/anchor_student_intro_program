import * as anchor from "@coral-xyz/anchor";

import { AnchorStudentIntroProgram } from "../target/types/anchor_student_intro_program";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

describe("anchor-studen-intro-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .AnchorStudentIntroProgram as Program<AnchorStudentIntroProgram>;

  const studentDemo = {
    name: "Ludwig",
    introduction: "I am the best player in the world!!!",
  };

  const [studentPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(studentDemo.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Student is added!", async () => {
    // Add your test here.
    const tx = await program.methods
      .addStudent(studentDemo.name, studentDemo.introduction)
      .rpc();

    const account = await program.account.studentAccountState.fetch(studentPDA);
    expect(studentDemo.name === account.name);
    expect(studentDemo.introduction === account.introduction);
    expect(account.studentAddress === provider.wallet.publicKey);
  });

  it("Student introduction is updated", async () => {
    const newIntroduction = "I am the worst player in the world!!!";

    const tx = await program.methods.updateIntro(newIntroduction).rpc();

    const account = await program.account.studentAccountState.fetch(studentPDA);
    expect(studentDemo.name === account.name);
    expect(newIntroduction === account.introduction);
    expect(account.studentAddress === provider.wallet.publicKey);
  });

  it("Deletes the student", async () => {
    const tx = await program.methods.deleteStudent(studentDemo.name).rpc();
  });
});
