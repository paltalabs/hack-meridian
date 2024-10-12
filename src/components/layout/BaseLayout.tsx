import { Grid, GridItem } from '@chakra-ui/react'
import type { FC, PropsWithChildren } from 'react'
import 'twin.macro'

export const BaseLayout: FC<PropsWithChildren> = ({ children }) => {
  return (
    <>
      <Grid h={'100vh'} alignItems={'center'} justifyContent={'center'}>
        <GridItem>
          {children}
        </GridItem>
      </Grid>
    </>
  )
}
