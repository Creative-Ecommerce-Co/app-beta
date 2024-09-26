import React from 'react';
import { Box, Button, TextField, FormControlLabel, Checkbox, Typography } from '@mui/material';

interface SettingsFormProps {
    title: string;
    fields: { name: string; label: string; type: string }[];
    onSave: (data: any) => void;
}

const SettingsForm: React.FC<SettingsFormProps> = ({ title, fields, onSave }) => {
    const [formData, setFormData] = React.useState({});

    const handleChange = (e) => {
        const { name, value, type, checked } = e.target;
        setFormData({
            ...formData,
            [name]: type === 'checkbox' ? checked : value,
        });
    };

    const handleSubmit = (e) => {
        e.preventDefault();
        onSave(formData);
    };

    return (
        <Box component="form" onSubmit={handleSubmit} sx={{ marginBottom: 2 }}>
            <Typography variant="h6" gutterBottom>
                {title}
            </Typography>
            {fields.map((field) => (
                <Box key={field.name} sx={{ marginBottom: 2 }}>
                    {field.type === 'checkbox' ? (
                        <FormControlLabel
                            control={
                                <Checkbox
                                    name={field.name}
                                    checked={formData[field.name] || false}
                                    onChange={handleChange}
                                />
                            }
                            label={field.label}
                        />
                    ) : (
                        <TextField
                            fullWidth
                            variant="outlined"
                            label={field.label}
                            name={field.name}
                            type={field.type}
                            value={formData[field.name] || ''}
                            onChange={handleChange}
                        />
                    )}
                </Box>
            ))}
            <Button type="submit" variant="contained" color="primary">
                Save
            </Button>
        </Box>
    );
};

export default SettingsForm;