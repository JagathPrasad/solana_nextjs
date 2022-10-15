import '../styles/globals.css'
//import { WalletConnectionProvider } from '../context/WalletConnectionProvider';
import '@solana/wallet-adapter-react-ui/styles.css';
import dynamic from 'next/dynamic';
function MyApp({ Component, pageProps }) {
  const WalletConnectionProvider = dynamic(() =>
    import('../context/WalletConnectionProvider'), { ssr: false }
  );
  return (
    <WalletConnectionProvider>
      <Component {...pageProps} />
    </WalletConnectionProvider>
    )
}

export default MyApp
