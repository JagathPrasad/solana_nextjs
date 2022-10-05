import '../styles/globals.css'
import { WalletConnectionProvider } from '../context/WalletConnectionProvider';
import '@solana/wallet-adapter-react-ui/styles.css';

function MyApp({ Component, pageProps }) {
  return
  // <WalletConnectionProvider>
    <Component {...pageProps} />
  // </WalletConnectionProvider>
}

export default MyApp
