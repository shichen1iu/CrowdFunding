import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SystemProgram } from "@solana/web3.js";
import { Crowdfunding } from "../target/types/crowdfunding";
const assert = require("assert");

describe("crowdfunding", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Crowdfunding as Program<Crowdfunding>;

  // 创建user账户
  const user = anchor.web3.Keypair.generate();
  const [campaignPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("COMPAIGN_DEMO"), user.publicKey.toBuffer()],
    program.programId
  );

  //测试create指令
  it("Creates a crowdfunding campaign", async () => {
    // 1. 为用户账户请求Airdrop
    await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );

    // 等待 Airdrop 完成
    await new Promise((resolve) => setTimeout(resolve, 5000));

    // 2. 获取租金（rent）信息
    const rent = await provider.connection.getMinimumBalanceForRentExemption(
      9000
    ); // 根据账户的实际大小设置

    // 3. 创建众筹账户
    const tx = await program.methods
      .create("My Campaign", "A campaign to raise funds")
      .accounts({
        campaign: campaignPda,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Transaction signature", tx);

    // 4. 检查是否成功创建众筹账户
    const campaignAccount = await program.account.campaign.fetch(campaignPda);

    // 5. 断言检查
    assert.equal(campaignAccount.name, "My Campaign");
    assert.equal(campaignAccount.description, "A campaign to raise funds");
    assert.equal(campaignAccount.amountDonated.toNumber(), 0);
    assert.equal(campaignAccount.admin.toBase58(), user.publicKey.toBase58());
  });

  //测试withdrew指令
  it("Withdrew from crowdfunding", async () => {
    const tx = await program.methods
      .withdrew(new anchor.BN(0.001 * anchor.web3.LAMPORTS_PER_SOL))
      .accounts({
        campaign: campaignPda,
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    console.log("Transaction signature", tx);
  });

  //测试转账代码
  it("Donate for crowdfunding", async () => {
    const tx = await program.methods
      .donate(new anchor.BN(0.5 * anchor.web3.LAMPORTS_PER_SOL)) // 将 0.5 SOL 转换为 lamports
      .accounts({
        campaign: campaignPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId, // 参考 anchor.web3 的 SystemProgram
      })
      .signers([user])
      .rpc();

    console.log("Transaction signature", tx);
  });
});
