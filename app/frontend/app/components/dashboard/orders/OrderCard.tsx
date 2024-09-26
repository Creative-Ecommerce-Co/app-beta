import React from 'react';
import { Card, CardContent, Typography, CardActions, Box } from '@mui/material';
import OrderActions from './OrderActions';

const OrderCard: React.FC<{ order: any }> = ({ order }) => {
    return (
        <Card>
            <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', marginBottom: 2 }}>
                    <Typography variant="h5" component="div">
                        Order #{order.id}
                    </Typography>
                </Box>
                <Typography variant="body2" color="text.secondary" sx={{ marginBottom: 2 }}>
                    Customer: {order.customerName}
                </Typography>
                <Typography variant="body2" color="text.secondary" sx={{ marginBottom: 2 }}>
                    Status: {order.status}
                </Typography>
                <Typography variant="body2" color="text.secondary" sx={{ marginBottom: 2 }}>
                    Total: ${order.total.toFixed(2)}
                </Typography>
            </CardContent>
            <CardActions>
                <OrderActions orderId={order.id} />
            </CardActions>
        </Card>
    );
};

export default OrderCard;