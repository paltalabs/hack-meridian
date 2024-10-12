import 'twin.macro'
import {useSorobanReact} from "@soroban-react/core"

import {
  Button,
  HStack,
  Menu,
  MenuButton,
  MenuDivider,
  MenuItem,
  MenuList,
  Text,
  VStack,
} from '@chakra-ui/react'
import { FiChevronDown } from 'react-icons/fi'
import { AiOutlineCheckCircle, AiOutlineDisconnect } from 'react-icons/ai'
import toast from 'react-hot-toast'
import type { WalletChain } from '@soroban-react/types'

export const ConnectButton = () => {
    // Connect Button
    const sorobanContext = useSorobanReact()

    const {activeChain, address, disconnect, setActiveConnectorAndConnect, setActiveChain} = sorobanContext;
    const activeAccount = address;

  const browserWallets = sorobanContext.connectors;

    if (!activeAccount)
      return (
        <Menu>
          <MenuButton
            as={Button}
            size="md"
            rightIcon={<FiChevronDown size={22} />}
            py={6}
            fontWeight="bold"
            rounded="2xl"
            colorScheme="purple"
          >
            Connect Wallet
          </MenuButton>

          <MenuList bgColor="blackAlpha.900" borderColor="whiteAlpha.300" rounded="2xl">
            {/* Installed Wallets */}
            {!activeAccount &&
              browserWallets.map((w) => 
                  <MenuItem
                    key={w.name}
                    onClick={() => {
                      setActiveConnectorAndConnect && setActiveConnectorAndConnect(w)
                    }}
                    tw="bg-transparent hocus:bg-gray-800"
                  >
                    {w.name}
                  </MenuItem>
              )}
          </MenuList>
        </Menu>
      )
      

    // Account Menu & Disconnect Button
    return (
      <Menu matchWidth>
        <HStack>
          {/* Account Name, Address, and AZNS-Domain (if assigned) */}
          <MenuButton
            as={Button}
            rightIcon={<FiChevronDown size={22} />}
            hidden={false}
            py={6}
            pl={5}
            rounded="2xl"
            fontWeight="bold"
          >
            <VStack spacing={0.5}>
              {/* <AccountName account={activeAccount} /> */}
              <Text>{activeChain?.name}</Text>
              <Text fontSize="xs" fontWeight="normal" opacity={0.75}>
                {address}
              </Text>
            </VStack>
          </MenuButton>
        </HStack>

        <MenuList
          bgColor="blackAlpha.900"
          borderColor="whiteAlpha.300"
          rounded="2xl"
          maxHeight="40vh"
        >

          {/* Disconnect Button */}
          <MenuDivider />
          <MenuItem
            onClick={async () => {console.log("Disconnecting"); await disconnect()}}
            icon={<AiOutlineDisconnect size={18} />}
            tw="bg-transparent hocus:bg-gray-800"
          >
            Disconnect
          </MenuItem>
        </MenuList>
      </Menu>
    )
  }