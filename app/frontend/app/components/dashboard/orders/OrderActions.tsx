import React from 'react';
import { Button } from '@mui/material';

const OrderActions: React.FC<{ orderId: string }> = ({ orderId }) => {
    const handleMarkAsShipped = () => {
        // Implement mark as shipped functionality
    };

    const handleCancelOrder = () => {
        // Implement cancel order functionality
    };

    return (
        <>
            <Button variant="contained" color="primary" onClick={handleMarkAsShipped}>
                Mark as Shipped
            </Button>
            <Button variant="contained" color="secondary" onClick={handleCancelOrder}>
                Cancel Order
            </Button>
        </>
    );
};

export default OrderActions;