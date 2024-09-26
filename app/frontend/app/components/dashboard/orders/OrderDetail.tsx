import React, { useState, useEffect } from 'react';
import { Box, Typography, CircularProgress, Button } from '@mui/material';
import { useParams } from 'react-router-dom';
import { fetchOrderDetail } from 'api/orders'; // Assume this is an API call to fetch order details

const OrderDetail: React.FC = () => {
    const { orderId } = useParams();
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [order, setOrder] = useState(null);

    useEffect(() => {
        const loadData = async () => {
            try {
                const response = await fetchOrderDetail(orderId);
                setOrder(response);
            } catch (err) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        loadData();
    }, [orderId]);

    if (loading) {
        return (
            <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
                <CircularProgress />
            </Box>
        );
    }

    if (error) {
        return (
            <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
                <Typography variant="h6" color="error">
                    {error}
                </Typography>
            </Box>
        );
    }

    return (
        <Box sx={{ padding: 2 }}>
            <Typography variant="h4" gutterBottom>
                Order #{order.id}
            </Typography>
            <Typography variant="body1" gutterBottom>
                Customer: {order.customerName}
            </Typography>
            <Typography variant="body2" color="text.secondary" gutterBottom>
                Status: {order.status}
            </Typography>
            <Typography variant="body2" color="text.secondary" gutterBottom>
                Total: ${order.total.toFixed(2)}
            </Typography>
            <Button variant="contained" color="primary">
                Mark as Shipped
            </Button>
        </Box>
    );
};

export default OrderDetail;