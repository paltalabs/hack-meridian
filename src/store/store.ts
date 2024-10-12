import { configureStore } from '@reduxjs/toolkit'
import employerSlice, { EmployerState } from './features/employerStore'



export const makeStore = () => {
    return configureStore({
        reducer: {
            employer: employerSlice,
        },
    })
}

export type AppStore = ReturnType<typeof makeStore>
export type RootState = ReturnType<AppStore['getState']> & {
    employer: EmployerState,
}
export type AppDispatch = AppStore['dispatch']