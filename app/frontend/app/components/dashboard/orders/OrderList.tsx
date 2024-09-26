import React from 'react';
import { Grid } from '@mui/material';
import OrderCard from './OrderCard';

const OrderList: React.FC<{ orders: any[] }> = ({ orders }) => {
    return (
        <Grid container spacing={3}>
            {orders.map(order => (
                <Grid item xs={12} sm={6} md={4} key={order.id}>
                    <OrderCard order={order} />
                </Grid>
            ))}
        </Grid>
    );
};

export default OrderList;