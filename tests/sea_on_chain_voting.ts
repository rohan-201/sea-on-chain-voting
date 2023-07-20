import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SeaOnChainVoting } from "../target/types/sea_on_chain_voting";

describe("sea_on_chain_voting", () => {
	// Configure the client to use the local cluster.
	const provider = anchor.AnchorProvider.env();
	anchor.setProvider(provider);

	const program = anchor.workspace
		.SeaOnChainVoting as Program<SeaOnChainVoting>;

	const owner = provider.wallet.publicKey;
	const voteBank = anchor.web3.PublicKey.findProgramAddressSync(
		[Buffer.from("vote_account")],
		program.programId
	)[0];

	it("Creating vote bank for public to vote", async () => {
		await program.methods
			.initVoteBank()
			.accounts({ signer: owner, voteAccount: voteBank })
			.rpc();
	});

	it("Vote for GM", async () => {
		await program.methods
			.gibVote({ gm: true })
			.accounts({ voteAccount: voteBank })
			.rpc();

		let voteBankAccount = await program.account.voteBank.fetch(voteBank);
		console.log(`Total GMs :: ${voteBankAccount.gm}`);
		console.log(`Total GNs :: ${voteBankAccount.gn}`);
	});

	it("Vote for GN", async () => {
		await program.methods
			.gibVote({ gn: true })
			.accounts({ voteAccount: voteBank })
			.rpc();
		let voteBankAccount = await program.account.voteBank.fetch(voteBank);
		console.log(`Total GMs :: ${voteBankAccount.gm}`);
		console.log(`Total GNs :: ${voteBankAccount.gn}`);
	});
});
