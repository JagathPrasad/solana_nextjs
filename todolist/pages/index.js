import Head from 'next/head'
import Image from 'next/image'
import styles from '../styles/Home.module.css'
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
export default function Home() {
  return (
    <div className={styles.container}>
      
      <main className={styles.main}>
        
        <WalletMultiButton />
      </main>

     
    </div>
  )
}
