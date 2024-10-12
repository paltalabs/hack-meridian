import { CenterBody } from '@/components/layout/CenterBody'
import UploadComponent from '@/components/upload'
import { ConnectButton } from '@/components/web3/ConnectButton'
import type { NextPage } from 'next'
import 'twin.macro'

const HomePage: NextPage = () => {

  return (
    <>
      <CenterBody tw="mt-4 mb-10 px-5">
        <ConnectButton />
        <UploadComponent />
      </CenterBody>
    </>
  )
}

export default HomePage
