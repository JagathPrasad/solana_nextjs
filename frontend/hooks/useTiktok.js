import { useEffect } from "react";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { useWallet } from "@solana/wallet-adapter-react";
import { PublicKey } from "@solana/web3.js";
import { getProgramInstance } from "../utils/utils";
import { SOLANA_HOST } from "../utils/const";


const anchor = require('@project-serum/anchor');
const utf8 = anchor.utils.bytes.utf8;
const { BN, web3 } = anchor;
const { SystemProgram } = web3;

const defaultAccounts = {
    tokenProgram: TOKEN_PROGRAM_ID,
    clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
    systemProgram: SystemProgram.programId
}

const useTiktok = (setTiktoks, userDetail, videoUrl, description,
    setDescription, setVideoUrl, setNewVideoShow) => {

    const wallet = useWallet();
    const connection = new anchor.web3.Connection(SOLANA_HOST);
    const program = getProgramInstance(connection, wallet);

    //get all videos
    const getTiktoks = async () => {
        const videos = await program.account.videoAccount.all();
        console.log('videos', videos);
        setTiktoks(videos);
    }

    const likeVideo = async (address) => {

    }

    const createComment = async (address, count, comment) => {

    }

    const newVideo = async () => {
        const randomKey = anchor.web3.Keypair.generate().publicKey;
        let [video_pda] = await anchor.web3.publicKey.findProgramAddress(
            [utf8.encode('video'), randomKey.toBuffer()],
            program.programId
        )

        const tx = await program.rpc.createVideo(
            description,
            videoUrl,
            userDetail, userDetail.userProfileImageUrl, {
            accounts: video_pda,
            randomKey: randomKey,
            authority: wallet.publicKey,
            ...defaultAccounts
        })
        setDescription('');
        setVideoUrl('');
        setNewVideoShow(false);
    }

    const getComments = async (address, count) => {

    }

    return { getTiktoks, likeVideo, createComment, newVideo, getComments };
}
export default useTiktok;