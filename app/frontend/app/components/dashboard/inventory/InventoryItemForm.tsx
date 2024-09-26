// app/frontend/app/components/dashboard/inventory/InventoryItemForm.tsx

import React, { useState } from 'react';
import { Box, Button, TextField, Typography } from '@mui/material';

interface InventoryItemFormProps {
    onSubmit: (item: { name: string; quantity: number; price: number }) => void;
    initialData?: { name: string; quantity: number; price: number };
}

const InventoryItemForm: React.FC<InventoryItemFormProps> = ({ onSubmit, initialData }) => {
    const [name, setName] = useState(initialData?.name || '');
    const [quantity, setQuantity] = useState(initialData?.quantity || 0);
    const [price, setPrice] = useState(initialData?.price || 0);

    const handleSubmit = () => {
        onSubmit({ name, quantity, price });
        setName('');
        setQuantity(0);
        setPrice(0);
    };

    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                {initialData ? 'Edit Inventory Item' : 'Add Inventory Item'}
            </Typography>
            <TextField
                label="Item Name"
                value={name}
                onChange={(e) => setName(e.target.value)}
                fullWidth
                margin="normal"
            />
            <TextField
                label="Quantity"
                type="number"
                value={quantity}
                onChange={(e) => setQuantity(Number(e.target.value))}
                fullWidth
                margin="normal"
            />
            <TextField
                label="Price"
                type="number"
                value={price}
                onChange={(e) => setPrice(Number(e.target.value))}
                fullWidth
                margin="normal"
            />
            <Button variant="contained" color="primary" onClick={handleSubmit}>
                {initialData ? 'Update Item' : 'Add Item'}
            </Button>
        </Box>
    );
};

export default InventoryItemForm;