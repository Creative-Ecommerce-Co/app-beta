import React from 'react';
import { FormControl, InputLabel, Select, MenuItem, Box } from '@mui/material';

interface AppCategoryFilterProps {
    value: string;
    onChange: (value: string) => void;
    categories: string[];
}

const AppCategoryFilter: React.FC<AppCategoryFilterProps> = ({ value, onChange, categories }) => {
    return (
        <Box sx={{ marginBottom: 2 }}>
            <FormControl fullWidth variant="outlined">
                <InputLabel>Category</InputLabel>
                <Select
                    value={value}
                    onChange={(e) => onChange(e.target.value)}
                    label="Category"
                >
                    <MenuItem value="">
                        <em>All</em>
                    </MenuItem>
                    {categories.map((category) => (
                        <MenuItem key={category} value={category}>
                            {category}
                        </MenuItem>
                    ))}
                </Select>
            </FormControl>
        </Box>
    );
};

export default AppCategoryFilter;