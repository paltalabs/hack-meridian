import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import type { RootState } from '../store'

// Define the structures to match the Soroban contract models

export interface WorkContract {
    employee: Employee;
    payment_period: PaymentPeriod;
    salary: number;
    notice_periods_required: number;
    employment_start_date: number;
    employment_end_date: number;
    is_active: boolean;
    unemployed_at?: number | null;
    notice_period_payments_made?: number;
}

export interface Employee {
    address: string;
    name: string;
}

export enum PaymentPeriod {
    Weekly = 'Weekly',
    Monthly = 'Monthly',
    Annually = 'Annually',
}

export interface EmployerState {
    address: string;
    name: string;
    balance: number;
    employees: Record<string, WorkContract>; // Using a record to store employees by address
    total_liabilities: number;
}

// Define the initial state using that type
const initialState: EmployerState = {
    address: '',
    name: '',
    balance: 0,
    employees: {},
    total_liabilities: 0,
}

// Define the Redux slice
export const employerSlice = createSlice({
    name: 'employer',
    initialState,
    reducers: {
        setAddress: (state, action: PayloadAction<string>) => {
            state.address = action.payload
        },
        setName: (state, action: PayloadAction<string>) => {
            state.name = action.payload
        },
        setBalance: (state, action: PayloadAction<number>) => {
            state.balance = action.payload
        },
        addEmployee: (state, action: PayloadAction<{ address: string, workContract: WorkContract }>) => {
            state.employees[action.payload.address] = action.payload.workContract;
        },
        updateEmployeeContract: (state, action: PayloadAction<{ address: string, updatedContract: Partial<WorkContract> }>) => {
            const employee = state.employees[action.payload.address];
            if (employee) {
                state.employees[action.payload.address] = {
                    ...employee,
                    ...action.payload.updatedContract,
                };
            }
        },
        setTotalLiabilities: (state, action: PayloadAction<number>) => {
            state.total_liabilities = action.payload
        },
        resetEmployer: (state) => {
            state.address = '';
            state.name = '';
            state.employees = {};
            state.total_liabilities = 0;
        },
    },
})

// Export actions
export const {
    setAddress,
    setName,
    addEmployee,
    setBalance,
    updateEmployeeContract,
    setTotalLiabilities,
    resetEmployer,
} = employerSlice.actions

// Selectors
export const selectEmployerAddress = (state: RootState) => state.employer.address
export const selectEmployerName = (state: RootState) => state.employer.name
export const selectBalance = (state: RootState) => state.employer.balance
export const selectEmployerEmployees = (state: RootState) => state.employer.employees
export const selectTotalLiabilities = (state: RootState) => state.employer.total_liabilities

export default employerSlice.reducer
