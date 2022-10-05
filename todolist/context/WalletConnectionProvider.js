import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { ConnectionProvider, WalletProvider } from '@solana/wallet-adapter-react';
import { WalletModelProvider } from '@solana/wallet-adapter-react-ui';
import { PhantomWalletAdapter, GlowWalletAdapter, SlopeWalletAdapter } from '@solana/wallet-adapter-wallets';
import { clusterApiUrl } from '@solana/web3.js';
import { useMemo } from 'react';


export const WalletConnectionProvider = ({ children }) => {
    const network = WalletAdapterNetwork.Devnet;
    const endpoint = useMemo(() => {
        // if (network == WalletAdapterNetwork.Devnet) {
        //     return '';//quicknode api
        // }
        return clusterApiUrl('devnet');
    }, [network]);

    const wallet = useMemo(() => {
        new PhantomWalletAdapter()
    }, [network])

    return (
        <ConnectionProvider endpoint={endpoint}>
            <WalletProvider wallets={wallet} autoConnect>
                <WalletModelProvider>{children}</WalletModelProvider>
            </WalletProvider>
        </ConnectionProvider>
    );
}

