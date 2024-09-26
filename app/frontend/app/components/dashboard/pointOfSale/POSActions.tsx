import React from 'react';
import { Button } from '@mui/material';

const POSActions: React.FC<{ onAddToCart: () => void; onRemoveFromCart: () => void }> = ({ onAddToCart, onRemoveFromCart }) => {
    return (
        <>
            <Button variant="contained" color="primary" onClick={onAddToCart}>
                Add to Cart
            </Button>
            <Button variant="contained" color="secondary" onClick={onRemoveFromCart}>
                Remove from Cart
            </Button>
        </>
    );
};

export default POSActions;