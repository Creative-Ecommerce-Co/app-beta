import React from 'react';
import { TextField, Box } from '@mui/material';

const AppSearchBar: React.FC<{ value: string; onChange: (value: string) => void }> = ({ value, onChange }) => {
    return (
        <Box sx={{ marginBottom: 2 }}>
            <TextField
                label="Search Apps"
                variant="outlined"
                fullWidth
                value={value}
                onChange={(e) => onChange(e.target.value)}
            />
        </Box>
    );
};

export default AppSearchBar;