import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { Avatar, Grid, GridItem, IconButton, Input, InputGroup, InputLeftElement, InputRightElement } from '@chakra-ui/react'
import { AddIcon, Search2Icon } from '@chakra-ui/icons'

export const SearchBar = ({ handleOpenDrawer }: { handleOpenDrawer: (value: boolean) => void; }) => {
  const { address } = useSorobanReact()

  if (!address) return null;
  return (
    <Grid templateColumns={'repeat(12, 1fr)'} gap={6} w={'full'} alignItems={'center'} position={'fixed'} top={8} px={6}>
      <GridItem colSpan={2} onClick={() => handleOpenDrawer(true)}>
        <Avatar name='Jhon Doe' src='https://static.vecteezy.com/system/resources/previews/004/511/281/original/default-avatar-photo-placeholder-profile-picture-vector.jpg' />
      </GridItem>
      <GridItem colSpan={10}>
        <InputGroup width={'100%'}>
          <InputLeftElement pointerEvents='none'>
            <Search2Icon color='gray.500' />
          </InputLeftElement>
          <Input
            placeholder='Search employee'
            boxShadow='md'
            rounded={18}
          />
          <InputRightElement>
            <IconButton
              rounded={32}
              size={'sm'}
              aria-label="search-Vault"
              colorScheme="green"
              variant={'solid'}
              icon={<AddIcon />} />
          </InputRightElement>
        </InputGroup>

      </GridItem>
    </Grid>
  )
}

