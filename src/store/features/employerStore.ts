import { createSlice, createAsyncThunk } from '@reduxjs/toolkit'
import type { PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'
import { ChainMetadata } from '@soroban-react/types'
import { Networks } from '@stellar/stellar-sdk'

export interface EmployerState {
    address: string;
}

// Define the initial state using that type
const initialState: EmployerState = {
    address: '',
}


export const employerSlice = createSlice({
    name: 'employer',
    initialState,
    reducers: {
        setAddress: (state, action: PayloadAction<string>) => {
            state.address = action.payload
        },
        resetAddress: (state) => {
            state.address = ''
        },
    },
    // extraReducers(builder) {
    //     builder.addCase(fetchDefaultAddresses.pending, (state) => {
    //         state.vaults.isLoading = true
    //     })
    //     builder.addCase(fetchDefaultAddresses.fulfilled, (state, action) => {
    //         state.vaults.isLoading = false
    //         state.vaults.createdVaults = action.payload!
    //     })
    //     builder.addCase(fetchDefaultAddresses.rejected, (state) => {
    //         state.vaults.isLoading = false
    //         state.vaults.hasError = true
    //     })
    // },
})

export const {
    setAddress,
    resetAddress,
} = employerSlice.actions

// Other code such as selectors can use the imported `RootState` type
export const selectAddress = (state: RootState) => state.employer.address
// export const selectChainMetadata = (state: RootState) => state.employer.selectedChain

export default employerSlice.reducer