import * as anchor from '@project-serum/anchor';
import { useEffect, useMemo, useState } from 'react';
import { TODO_PROGRAM_PUBKEY } from '../constants/index';
//import todoIdl from '../constants/todo.json';
import toast from 'react-hot-toast';
import { SystemProgram } from '@solana/web3.js';
import { utf8 } from '@project-serum/anchor/dist/cjs/utils/bytes';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { useAnchorWallet, useConnection, useWallet } from '@solana/wallet-adapter-react';
//import {authorFilter} from '../utils';

export function useTodo() {
    const { connnection } = useConnection();
    const { publicKey } = useWallet();
    const anchorWallet = useAnchorWallet();


    const [initialized, setInitialized] = useState(false);
    const [lastTodo, setLastTodo] = useState(0);
    const [todos, setTodos] = useState([]);
    const [loading, setLoading] = useState(false);
    const [transactionPending, setTransactionPending] = useState(false);
    const [input, setInput] = useState("");


    const program = useMemo(() => {
        if (anchorWallet) {
            const provider = new anchor.AnchorProvider(connnection, anchorWallet);
            return new anchor.Program(todoIdl, TODO_PROGRAM_PUBKEY, provider);
        }
    }, [connnection, anchorWallet]);

    useEffect(() => {
        const findProfileAccounts = async () => {
            if (program && publicKey && !transactionPending) {
                try {
                    //setLoading(true);
                    const [profilePda, profileBump] = await findProgramAddressSync([utf8.encode('USER_STATE'), publicKey.toBuffer(), program.programId]);
                    const profileAccounts = await program.account.userProfile.fetch(profilePda);
                    if (profileAccounts) {
                        setLastTodo(profileAccounts.LastTodo);
                        setInitialized(true);
                        const todoAccount = await program.account.todoAccount.all();
                        setTodos(todoAccount);
                    } else {
                        setInitialized(false);
                    }
                } catch (error) {
                    setInitialized(false);
                    setTodos([]);
                }
            }
        }

        findProfileAccounts();
    }, [publicKey.program,transactionPending]);

    const handleChange = (e) => {
        setInput(e.target.value);
    }

    const initializedStaticUser = () => {
        setInitialized(true);
    }

}