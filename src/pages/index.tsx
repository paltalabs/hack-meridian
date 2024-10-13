import { CenterBody } from '@/components/layout/CenterBody'
import { ConnectButton } from '@/components/web3/ConnectButton'
import type { NextPage } from 'next'
import 'twin.macro'
import { useSorobanReact } from '@soroban-react/core'
import MainPage from '@/components/layout/MainPage'
import Image from 'next/image'
import logo from '@/assets/logo.png'

const HomePage: NextPage = () => {
  const { address } = useSorobanReact()

  return (
    <>
      <CenterBody tw="mt-4 mb-10 px-5 max-w-full">
        {!address ? (
          <>
            <div tw="flex justify-center mb-8">
              <Image src={logo} alt="Logo" width={303} height={155} />
            </div>
            <ConnectButton />
          </>
        ) : (
          <MainPage />
        )}
        {/* <Example /> */}
        {/* <Example /> */}
      </CenterBody>
    </>
  )
}

export default HomePage
