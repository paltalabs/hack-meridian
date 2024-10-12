import { Text } from "@chakra-ui/react"
import { useSorobanReact } from "@soroban-react/core"
import { useSelector } from 'react-redux';
import {
    selectEmployerAddress,
    selectEmployerName,
    selectEmployerEmployees,
    selectTotalLiabilities
} from '@/store/features/employerStore';

export const Example = () => {

    const { address } = useSorobanReact();

    // Use selectors to get the required data from the Redux store
    const name = useSelector(selectEmployerName);
    const employees = useSelector(selectEmployerEmployees);
    const totalLiabilities = useSelector(selectTotalLiabilities);

    return (
        <div>
            <button>
                HOLI
            </button>
            <h2>Employer Information</h2>
            <p><strong>Address:</strong> {address}</p>
            <p><strong>Name:</strong> {name}</p>
            <p><strong>Total Liabilities:</strong> {totalLiabilities}</p>

            <h3>Employees:</h3>
            <ul>
                {Object.keys(employees).length === 0 ? (
                    <p>No employees found.</p>
                ) : (
                    Object.keys(employees).map((employeeAddress) => {
                        const employee = employees[employeeAddress];
                        return (
                            <li key={employeeAddress}>
                                <p><strong>Employee Address:</strong> {employee.employee.address}</p>
                                <p><strong>Employee Name:</strong> {employee.employee.name}</p>
                                <p><strong>Salary:</strong> {employee.salary}</p>
                                <p><strong>Payment Period:</strong> {employee.payment_period}</p>
                                <p><strong>Notice Period:</strong> {employee.notice_period}</p>
                                <p><strong>Employed At:</strong> {employee.employed_at}</p>
                                <p><strong>Is Active:</strong> {employee.is_active ? 'Yes' : 'No'}</p>
                                {employee.unemployed_at && (
                                    <p><strong>Unemployed At:</strong> {employee.unemployed_at}</p>
                                )}
                                <p><strong>Notice Period Payments Made:</strong> {employee.notice_period_payments_made}</p>
                                <hr />
                            </li>
                        );
                    })
                )}
            </ul>
        </div>
    );
}