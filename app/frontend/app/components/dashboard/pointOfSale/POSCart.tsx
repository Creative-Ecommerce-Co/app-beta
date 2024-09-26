import React from 'react';
import { Box, Typography, List, ListItem, ListItemText, Button } from '@mui/material';

const POSCart: React.FC<{ cart: any[]; onRemoveFromCart: (itemId: string) => void }> = ({ cart, onRemoveFromCart }) => {
    return (
        <Box>
            <Typography variant="h6" gutterBottom>
                Cart
            </Typography>
            <List>
                {cart.map(item => (
                    <ListItem key={item.id} secondaryAction={
                        <Button onClick={() => onRemoveFromCart(item.id)} color="secondary">Remove</Button>
                    }>
                        <ListItemText
                            primary={item.name}
                            secondary={`Price: $${item.price.toFixed(2)}`}
                        />
                    </ListItem>
                ))}
            </List>
            <Typography variant="h6" gutterBottom>
                Total: ${cart.reduce((total, item) => total + item.price, 0).toFixed(2)}
            </Typography>
        </Box>
    );
};

export default POSCart;