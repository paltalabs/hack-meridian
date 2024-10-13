import React from 'react'
import {SorobanReactProvider} from '@soroban-react/core';
import {testnet} from '@soroban-react/chains';
import {freighter} from '@soroban-react/freighter';
import type { ChainMetadata, Connector } from "@soroban-react/types";
import { hana } from '@soroban-react/hana';
import { lobstr } from '@soroban-react/lobstr';

import deployments from '../../../contracts/deployments.json';

const chains: ChainMetadata[] = [testnet];
export const connectors: Connector[] = [freighter(), hana(), lobstr()];


export default function MySorobanReactProvider({children}:{children: React.ReactNode}) {

    return (
      <SorobanReactProvider
        chains={chains}
        appName={"Example Stellar App"}
        activeChain={testnet}
        connectors={connectors}
        deployments={deployments}>
          {children}
      </SorobanReactProvider>
    )
  }