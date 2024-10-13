import { CenterBody } from '@/components/layout/CenterBody'
import { ConnectButton } from '@/components/web3/ConnectButton'
import type { NextPage } from 'next'
import { Example } from '@/components/example'
import 'twin.macro'
import { useSorobanReact } from '@soroban-react/core'
import MainPage from '@/components/layout/MainPage'

const HomePage: NextPage = () => {
  const { address } = useSorobanReact()

  return (
    <>
      <CenterBody tw="mt-4 mb-10 px-5 max-w-full">
        {!address ? (
          <ConnectButton />
        ) : (
          <MainPage />
        )}
        {/* <Example /> */}
        <Example />
      </CenterBody>
    </>
  )
}

export default HomePage
