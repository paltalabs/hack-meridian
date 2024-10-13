import { useSorobanReact } from '@soroban-react/core'
import React, { useState } from 'react'
import { ProfileDrawer } from '../Drawer/Drawer'
import { Accordion, AccordionButton, AccordionItem, AccordionPanel, Avatar, Button, Card, CardBody, CardFooter, Grid, GridItem, Heading, IconButton, Input, InputGroup, InputLeftElement, InputRightElement, Stack, Text } from '@chakra-ui/react'
import { AddIcon, Search2Icon } from '@chakra-ui/icons'
import { BsThreeDotsVertical, BsTrash } from "react-icons/bs";
import { SearchBar } from '../layout/SearchBar'
import { shortenAddress } from '@/utils/shortenAdress'

export const TradContractsAccordion = () => {
  const { address } = useSorobanReact()
  const contracts = ['GC2VCACI4VCD3RQF3JNQNYYCGXAMZRQ3LKF3Q5425W7RIY6PR2UUNNEB', 'GC2VCACI4VCD3RQF3JNQNYYCGXAMZRQ3LKF3Q5425W7RIY6PR2UUNNEB', 'GC2VCACI4VCD3RQF3JNQNYYCGXAMZRQ3LKF3Q5425W7RIY6PR2UUNNEB']

  if (!address) return null;
  return (
    <>
      <Accordion allowMultiple allowToggle>
        {contracts.map((contract, index) => (
          <Card
            overflow='hidden'
            w={'95vw'}
            mt={6}
            rounded={16}
            variant='elevated'
          >
            <AccordionItem sx={{ border: 'none' }}>
              <AccordionButton >
                <Grid
                  templateColumns={'repeat(12, 1fr)'}
                  gap={2}
                  alignItems={'center'}
                  width={'100%'}
                  px={6}
                  data-testid={`contract-${index}`}
                >
                  <GridItem colSpan={1} justifySelf={'start'}>
                    <Avatar
                      name='Jhon Doe'
                      src='https://static.vecteezy.com/system/resources/previews/004/511/281/original/default-avatar-photo-placeholder-profile-picture-vector.jpg' />
                  </GridItem>
                  <GridItem colSpan={10}>
                    <CardBody textAlign={'left'}>
                      <Heading size='md'>Pedro Urdemales</Heading>
                      <Text fontSize='sm' as={'sub'}>$ 600 - Monthly</Text>
                    </CardBody>
                  </GridItem>
                  <GridItem colSpan={1} justifySelf={'end'}>
                    <BsThreeDotsVertical />
                  </GridItem>
                </Grid>
              </AccordionButton>

              <AccordionPanel pb={4}>
                <Text as={'p'}>{shortenAddress(contract, 4)}</Text>
                <Text as={'sub'}>Notice 1 Month before</Text>
                <CardFooter>
                  <Stack direction='row' spacing={4} align={'center'}>
                    <IconButton size={'md'} variant={'ghost'} aria-label='delete-contract' icon={<BsTrash />} py={4} />
                    <Button colorScheme='green' size='md' >Review contract</Button>
                  </Stack>
                </CardFooter>
              </AccordionPanel>
            </AccordionItem>
          </Card>
        ))}
      </Accordion>

    </>
  )
}
