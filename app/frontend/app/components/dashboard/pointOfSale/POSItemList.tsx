import React from 'react';
import { Grid } from '@mui/material';
import POSItem from './POSItem';

const POSItemList: React.FC<{ onAddToCart: (item: any) => void }> = ({ onAddToCart }) => {
    const items = [
        { id: '1', name: 'Item 1', price: 10.0 },
        { id: '2', name: 'Item 2', price: 20.0 },
        // Add more items as needed
    ];

    return (
        <Grid container spacing={3}>
            {items.map(item => (
                <Grid item xs={12} sm={6} md={4} key={item.id}>
                    <POSItem item={item} onAddToCart={onAddToCart} />
                </Grid>
            ))}
        </Grid>
    );
};

export default POSItemList;