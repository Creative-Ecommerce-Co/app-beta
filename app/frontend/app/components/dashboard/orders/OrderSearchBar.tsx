import React from 'react';
import { TextField, Box } from '@mui/material';

const OrderSearchBar: React.FC<{ value: string; onChange: (value: string) => void }> = ({ value, onChange }) => {
    return (
        <Box sx={{ marginBottom: 2 }}>
            <TextField
                label="Search Orders"
                variant="outlined"
                fullWidth
                value={value}
                onChange={(e) => onChange(e.target.value)}
            />
        </Box>
    );
};

export default OrderSearchBar;