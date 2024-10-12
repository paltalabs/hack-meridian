import { BaseLayout } from '@/components/layout/BaseLayout'
import { HotToastConfig } from '@/components/layout/HotToastConfig'
import GlobalStyles from '@/styles/GlobalStyles'
import { ChakraProvider, DarkMode } from '@chakra-ui/react'
import { cache } from '@emotion/css'
import { CacheProvider } from '@emotion/react'
import { DefaultSeo } from 'next-seo'
import type { AppProps } from 'next/app'
import { Inconsolata } from 'next/font/google'
import Head from 'next/head'
import { StoreProvider } from '../store-provider'

import MySorobanReactProvider from "../components/web3/MySorobanReactProvider"

// Google Font(s) via `next/font`
const inconsolata = Inconsolata({ subsets: ['latin'] })

function MyApp({ Component, pageProps }: AppProps) {
  return (
    <>
      <DefaultSeo
        defaultTitle="Payroll" // TODO
        titleTemplate="" // TODO
        description="TBD" // TODO
        openGraph={{
          type: 'website',
          locale: 'en',
          // url: env.url,
          site_name: 'soroban-react-dapp', // TODO
          images: [],
        }}
      />

      <Head>
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />

        {/* Set Font Variables */}
        <style>{`
          :root {
            --font-inconsolata: ${inconsolata.style.fontFamily}, 'Inconsolata';
          }
        `}</style>
      </Head>
      <StoreProvider>

        <MySorobanReactProvider>
          <CacheProvider value={cache}>
            <ChakraProvider>
              <GlobalStyles />

              <BaseLayout>
                <Component {...pageProps} />
              </BaseLayout>

              <HotToastConfig />
            </ChakraProvider>
          </CacheProvider>
        </MySorobanReactProvider>
      </StoreProvider>
    </>
  )
}

export default MyApp
