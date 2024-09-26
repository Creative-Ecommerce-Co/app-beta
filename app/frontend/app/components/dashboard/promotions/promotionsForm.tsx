// app/frontend/app/components/dashboard/promotions/PromotionForm.tsx

import React, { useState } from 'react';
import { Box, Button, TextField, Typography } from '@mui/material';

interface PromotionFormProps {
    onSubmit: (promotion: { title: string; discount: number; startDate: string; endDate: string }) => void;
    initialData?: { title: string; discount: number; startDate: string; endDate: string };
}

const PromotionForm: React.FC<PromotionFormProps> = ({ onSubmit, initialData }) => {
    const [title, setTitle] = useState(initialData?.title || '');
    const [discount, setDiscount] = useState(initialData?.discount || 0);
    const [startDate, setStartDate] = useState(initialData?.startDate || '');
    const [endDate, setEndDate] = useState(initialData?.endDate || '');

    const handleSubmit = () => {
        onSubmit({ title, discount, startDate, endDate });
        setTitle('');
        setDiscount(0);
        setStartDate('');
        setEndDate('');
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                {initialData ? 'Edit Promotion' : 'Add Promotion'}
            </Typography>
            <TextField
                label="Promotion Title"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                fullWidth
                margin="normal"
            />
            <TextField
                label="Discount (%)"
                type="number"
                value={discount}
                onChange={(e) => setDiscount(Number(e.target.value))}
                fullWidth
                margin="normal"
            />
            <TextField
                label="Start Date"
                type="date"
                value={startDate}
                onChange={(e) => setStartDate(e.target.value)}
                fullWidth
                margin="normal"
                InputLabelProps={{ shrink: true }}
            />
            <TextField
                label="End Date"
                type="date"
                value={endDate}
                onChange={(e) => setEndDate(e.target.value)}
                fullWidth
                margin="normal"
                InputLabelProps={{ shrink: true }}
            />
            <Button variant="contained" color="primary" onClick={handleSubmit}>
                {initialData ? 'Update Promotion' : 'Add Promotion'}
            </Button>
        </Box>
    );
};

export default PromotionForm;