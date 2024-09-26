import React from 'react';
import { Button, Box } from '@mui/material';

const POSCheckout: React.FC<{ cart: any[]; onCheckout: () => void }> = ({ cart, onCheckout }) => {
    return (
        <Box sx={{ marginTop: 2 }}>
            <Button
                variant="contained"
                color="primary"
                onClick={onCheckout}
                disabled={cart.length === 0}
                fullWidth
            >
                Checkout
            </Button>
        </Box>
    );
};

export default POSCheckout;