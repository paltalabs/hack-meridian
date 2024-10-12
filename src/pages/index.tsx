import { CenterBody } from '@/components/layout/CenterBody'
import { ConnectButton } from '@/components/web3/ConnectButton'
import type { NextPage } from 'next'
import { Example } from '@/components/example'
import 'twin.macro'

const HomePage: NextPage = () => {

  return (
    <>
      <CenterBody tw="mt-4 mb-10 px-5">
        <ConnectButton />
        <Example />
      </CenterBody>
    </>
  )
}

export default HomePage
