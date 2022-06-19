import { clusterApiUrl, PublicKey } from '@solana/web3.js';
import tiktok from './tiktokclone.json';
export const TIKTOK_PROGRAM_ID = new PublicKey('Rkv9eZwXuDZezacPK5SRSQyqbaYKa7upXuF4kr949ri');

export const TIKTOK_IDL = tiktok;

export const SOLANA_HOST = clusterApiUrl('devnet');
