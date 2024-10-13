import { useSorobanReact } from '@soroban-react/core'
import React from 'react'
import { Accordion, AccordionButton, AccordionItem, AccordionPanel, Avatar, Button, Card, CardBody, CardFooter, Grid, GridItem, Heading, IconButton, Stack, Text } from '@chakra-ui/react'
import { BsThreeDotsVertical, BsTrash } from "react-icons/bs";
import { shortenAddress } from '@/utils/shortenAdress'
import { useSelector } from 'react-redux';
import { PaymentPeriod, selectEmployerEmployees } from '@/store/features/employerStore';
import { FireButton } from '../buttons/FireButton';

export const TradContractsAccordion = () => {
  const { address } = useSorobanReact()
  const employees = useSelector(selectEmployerEmployees);

  if (!address) return null;
  return (
    <>
      <Accordion allowToggle>
        {Object.keys(employees).map((contract, index) => (
          <Card
            overflow='hidden'
            w={'95vw'}
            mt={6}
            bg={'#F7F2FA'}
            rounded={16}
            variant='elevated'
            boxShadow='xl'
            _hover={{ boxShadow: '2xl', transform: 'scale(1.02)', transition: 'all 0.2s ease-in-out' }}
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
                      <Heading size='md'>{employees[contract].employee.name}</Heading>
                      <Text fontSize='sm' as={'sub'}>$ {Number(employees[contract].salary) / 10000000} - Payment Period: {employees[contract].payment_period === PaymentPeriod.Weekly ? "Weekly" : employees[contract].payment_period === PaymentPeriod.Monthly ? "Monthly" : "Yearly"}</Text>
                    </CardBody>
                  </GridItem>
                  <GridItem colSpan={1} justifySelf={'end'}>
                    <BsThreeDotsVertical />
                  </GridItem>
                </Grid>
              </AccordionButton>

              <AccordionPanel pb={4}>
                <Text as={'p'}>{shortenAddress(contract, 4)}</Text>
                <Text as={'sub'}>Notice {employees[contract].notice_periods_required} weeks before</Text>
                <CardFooter>
                  <Stack direction='row' spacing={4} align={'center'}>
                    <FireButton employee={employees[contract].employee.address} />
                    <Button bg='#2C8BBAE5' color={"white"} size='md' >Review contract</Button>
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
