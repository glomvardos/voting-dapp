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
  let poll_id: number
  // console.log('Cluster:', anchor.getProvider().connection.rpcEndpoint)
  // console.log('Program ID from workspace:', program.programId.toString())
  beforeAll(async () => {
    poll_id = Math.floor(Math.random() * Number.MAX_SAFE_INTEGER)
  })

  it('Initialize Poll', async () => {
    const tx = await program.methods.initializePoll(new BN(poll_id), 'test poll', new BN(0), new BN(1861166292)).rpc()
    // console.log('Initialize Poll tx: ', tx)
    // console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http://localhost:8899`)
  })

  // it('Update Poll', async () => {
  //   const pda = getPollPda(poll_id)
  //   let poll = await program.account.poll.fetch(pda)
  //   console.log('Stale Description: ', poll.description)
  //   const tx = await program.methods.updatePollDescription(new BN(poll_id), 'What is your favorite Pokemon?').rpc()
  //   poll = await program.account.poll.fetch(pda)
  //   console.log('Updated Description: ', poll.description)

  //   console.log('Initialize Poll tx: ', tx)
  //   console.log(`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http://localhost:8899`)
  // })

  it('Initialize Candidate', async () => {
    const txCharizard = await program.methods.initializeCandidate('Charizard', new BN(poll_id)).rpc()
    const txBlastoise = await program.methods.initializeCandidate('Blastoise', new BN(poll_id)).rpc()
    const pdaCharizard = getCandidatePda(poll_id, 'Charizard')
    const pdaBlastoise = getCandidatePda(poll_id, 'Blastoise')
    const candidateCharizard = await program.account.candidate.fetch(pdaCharizard)
    const candidateBlastoise = await program.account.candidate.fetch(pdaBlastoise)
    console.log('Candidate Charizard: ', candidateCharizard)
    console.log('Candidate Blastoise: ', candidateBlastoise)
    console.log(`https://explorer.solana.com/tx/${txCharizard}?cluster=custom&customUrl=http://localhost:8899`)
    console.log(`https://explorer.solana.com/tx/${txBlastoise}?cluster=custom&customUrl=http://localhost:8899`)
  })

  it('vote', async () => {
    const pollPda = getPollPda(poll_id)
    const pdaCharizard = getCandidatePda(poll_id, 'Charizard')
    const txCharizard = await program.methods.vote('Charizard', new BN(poll_id)).rpc()
    const candidateCharizard = await program.account.candidate.fetch(pdaCharizard)
    const poll = await program.account.poll.fetch(pollPda)
    console.log('Candidate Charizard: ', candidateCharizard)
    console.log('Poll: ', poll)
    console.log(`https://explorer.solana.com/tx/${txCharizard}?cluster=custom&customUrl=http://localhost:8899`)
  })

  function getPollPda(pollId: number) {
    const pollIdBn = new BN(pollId)
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [pollIdBn.toArrayLike(Buffer, 'le', 8)],
      program.programId,
    )
    return pda
  }
  function getCandidatePda(pollId: number, candidateName: string) {
    const pollIdBn = new BN(pollId)
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [pollIdBn.toArrayLike(Buffer, 'le', 8), Buffer.from(candidateName)],
      program.programId,
    )
    return pda
  }
})
