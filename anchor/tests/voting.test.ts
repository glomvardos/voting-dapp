import { Voting } from './../target/types/voting'
import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { BN } from 'bn.js'

describe('voting', () => {
  jest.setTimeout(30000)
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet
  console.log('Payer: ', payer.publicKey.toString())
  const program = anchor.workspace.Voting as Program<Voting>

  // console.log('Cluster:', anchor.getProvider().connection.rpcEndpoint)
  // console.log('Program ID from workspace:', program.programId.toString())

  // it('Initialize Poll', async () => {
  //   const tx = await program.methods.initializePoll(new BN(1), 'test poll', new BN(0), new BN(1861166292)).rpc()
  //   console.log('Initialize Poll tx: ', tx)
  //   console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  // })

  it('Update Poll', async () => {
    const pda = getPollPda(1)
    let poll = await program.account.poll.fetch(pda)
    console.log('Stale Description: ', poll.description)
    const tx = await program.methods.updatePollDescription(new BN(1), 'What is your favorite Pokemon?').rpc()
    poll = await program.account.poll.fetch(pda)
    console.log('Updated Description: ', poll.description)

    console.log('Initialize Poll tx: ', tx)
    console.log(`https://explorer.solana.com/tx/${tx}?cluster=devnet`)
  })

  function getPollPda(pollId: number) {
    const pollIdBn = new BN(pollId)
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [pollIdBn.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    )
    return pda
  }
})
