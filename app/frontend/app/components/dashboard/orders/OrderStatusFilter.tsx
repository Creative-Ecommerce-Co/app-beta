import React from 'react';
import { FormControl, InputLabel, Select, MenuItem, Box } from '@mui/material';

interface OrderStatusFilterProps {
    value: string;
    onChange: (value: string) => void;
    statuses: string[];
}

const OrderStatusFilter: React.FC<OrderStatusFilterProps> = ({ value, onChange, statuses }) => {
    return (
        <Box sx={{ marginBottom: 2 }}>
            <FormControl fullWidth variant="outlined">
                <InputLabel>Status</InputLabel>
                <Select
                    value={value}
                    onChange={(e) => onChange(e.target.value)}
                    label="Status"
                >
                    <MenuItem value="">
                        <em>All</em>
                    </MenuItem>
                    {statuses.map((status) => (
                        <MenuItem key={status} value={status}>
                            {status}
                        </MenuItem>
                    ))}
                </Select>
            </FormControl>
        </Box>
    );
};

export default OrderStatusFilter;