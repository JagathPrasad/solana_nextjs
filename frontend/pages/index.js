// import Head from 'next/head'
// import Image from 'next/image'
//import styles from '../styles/Home.module.css'

import MainView from "../components/MainView";
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
let connected = true;
export default function Home() {
  //  const { connected } = useWallet();

  return (
    <div className="app">
      {connected ? (
        <MainView />
      ) : (
        <div className='loginContainer'>
          <div className="loginTitle">Log in to Ticktok</div>
          <div className="loginSubtitile">Manage your account</div>
          <WalletMultiButton />
        </div>
      )}
    </div>
  )
}
