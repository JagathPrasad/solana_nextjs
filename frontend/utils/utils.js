import * as anchor from '@project-serum/anchor';
import { WalletNotConnectedError } from '@solana/wallet-adapter-base';
import { TIKTOK_IDL, TIKTOK_PROGRAM_ID } from './const';


export function getProgramInstance(connection, wallet) {
    console.log(wallet.publicKey, 'wallet.publicKey');
    if (wallet.publicKey != null) {
        throw new WalletNotConnectedError()
    }

    const provider = new anchor.AnchorProvider(
        connection, wallet, anchor.AnchorProvider.defaultOptions(),
    );

    const idl = TIKTOK_IDL;

    const prgoramId = TIKTOK_PROGRAM_ID;

    const program = new (anchor).Program(idl, prgoramId, provider);
    return program;
}