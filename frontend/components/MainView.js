import React from 'react';
import styles from '../styles/MainView.module.css';
import Signup from './Signup';
import { useEffect, useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { SOLANA_HOST } from '../utils/const';
import { getProgramInstance } from '../utils/utils';
import { TOKEN_PROGRAM_ID } from '@solana/spl-token';
import useAccount from '../hooks/useAccount';
import useTiktok from '../hooks/useTiktok';
const anchor = require('@project-serum/anchor');
const utf8 = anchor.utils.utf8;
const { BN, web3 } = anchor;
const { SystemProgram } = web3;

const defaulAccount = {
    tokenProgram: TOKEN_PROGRAM_ID,
    clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
    systemProgram: SystemProgram.programId,
}

const MainView = () => {
    const [isAccount, setAccount] = useState(false);
    const wallet = useWallet();
    const connection = new anchor.web3.Connection(SOLANA_HOST);
    const program = getProgramInstance(connection, wallet);
    const [tiktoks, setTikToks] = useState(false);
    const [newVideoShow, setNewVideoShow] = useState(false);
    const [description, setDescription] = useState('');
    const [videoUrl, setVideoUrl] = useState('');
    const [userDetail, setUserDetail] = useState();

    const { signup } = useAccount();

    const { getTiktoks, likeVideo, createComment, newVideo, getComments } = useTiktok(
        setTikToks, userDetail, videoUrl, description, setDescription, setVideoUrl, setNewVideoShow
    );
    //const isAccount = false;

    useEffect(() => {
        if (wallet.connected) {
            checkAccount();
            getTiktoks();
        }
    }, []);

    const checkAccount = () => {
        let [user_pda] = await anchor.web3.PublicKey.findProgramAddress(
            [utf8.encode('user'), wallet.publicKey.toBuffer()],
            program.programId
        )
        try {
            const userInfo = await program.account.userAccount.fetch(user_pda);
            setUserDetail(userInfo);
            console.log('UserInfo', userInfo);
            setAccount(true);

        } catch (e) {

        }
    }
    return (
        <>
            {isAccount ? (
                <div>Ticktok yes</div>
            ) : (
                <Signup signup={signup} wallet={wallet.publicKey?.toBase58()} />)}
        </>
    )
}

export default MainView;