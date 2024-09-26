import React, { useState } from 'react';
import { Box, Grid, Typography } from '@mui/material';
import POSItemList from './POSItemList';
import POSCart from './POSCart';
import POSCheckout from './POSCheckout';

const POSDashboard: React.FC = () => {
    const [cart, setCart] = useState([]);

    const handleAddToCart = (item) => {
        setCart([...cart, item]);
    };

    const handleRemoveFromCart = (itemId) => {
        setCart(cart.filter(item => item.id !== itemId));
    };

    const handleCheckout = () => {
        // Implement checkout functionality
    };

    return (
        <Box sx={{ padding: 2 }}>
            <Typography variant="h4" gutterBottom>
                Point of Sale
            </Typography>
            <Grid container spacing={3}>
                <Grid item xs={8}>
                    <POSItemList onAddToCart={handleAddToCart} />
                </Grid>
                <Grid item xs={4}>
                    <POSCart cart={cart} onRemoveFromCart={handleRemoveFromCart} />
                    <POSCheckout cart={cart} onCheckout={handleCheckout} />
                </Grid>
            </Grid>
        </Box>
    );
};

export default POSDashboard;