# sea_on_chain_voting
# Built with Seahorse v0.1.3

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')


class VoteBank(Account):
    is_open_to_vote: bool
    gm: u64
    gn: u64


class VoteType(Enum):
    GM = 0,
    GN = 1


@instruction
def init_vote_bank(signer: Signer, vote_account: Empty[VoteBank]):
    vote_account = vote_account.init(
        payer=signer,
        seeds=["vote_account"]
    )
    vote_account.is_open_to_vote = True


@instruction
def gib_vote(vote_account: VoteBank, vote_type: VoteType):
    if vote_type == VoteType.GM:
        print("Voted for GM")
        vote_account.gm += 1
    elif vote_type == VoteType.GN:
        print("Voted for GN")
        vote_account.gn += 1
