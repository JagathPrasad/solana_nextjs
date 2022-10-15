import * as anchor from '@project-serum/anchor';
import { useEffect, useMemo } from 'react';
import { TODO_PROGRAM_PUBKEY } from '../constants/index';
//import todoIdl from '../constants/todo.json';
import toast from 'react-hot-toast';
import { SystemProgram } from '@solana/web3.js';