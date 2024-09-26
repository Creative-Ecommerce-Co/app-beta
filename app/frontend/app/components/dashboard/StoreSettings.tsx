// app/frontend/app/components/dashboard/StoreSettings.tsx

import React, { useState } from 'react';
import { Box, Button, TextField, Typography } from '@mui/material';

const StoreSettings: React.FC = () => {
    const [storeName, setStoreName] = useState('');
    const [storeDescription, setStoreDescription] = useState('');
    const [storeEmail, setStoreEmail] = useState('');
    const [storePhone, setStorePhone] = useState('');

    const handleSave = () => {
        // Logic to save store settings
        console.log('Store settings saved:', { storeName, storeDescription, storeEmail, storePhone });
    };

    return (
        <Box sx={{ padding: 2 }}>
            <Typography variant="h4" gutterBottom>
                Store Settings
            </Typography>
            <TextField
                label="Store Name"
                value={storeName}
                onChange={(e) => setStoreName(e.target.value)}
                fullWidth
                margin="normal"
            />
            <TextField
                label="Store Description"
                value={storeDescription}
                onChange={(e) => setStoreDescription(e.target.value)}
                fullWidth
                margin="normal"
                multiline
                rows={4}
            />
            <TextField
                label="Store Email"
                value={storeEmail}
                onChange={(e) => setStoreEmail(e.target.value)}
                fullWidth
                margin="normal"
                type="email"
            />
            <TextField
                label="Store Phone"
                value={storePhone}
                onChange={(e) => setStorePhone(e.target.value)}
                fullWidth
                margin="normal"
                type="tel"
            />
            <Button variant="contained" color="primary" onClick={handleSave}>
                Save Settings
            </Button>
        </Box>
    );
};

export default StoreSettings;