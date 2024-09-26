import React from 'react';
import { Card, CardContent, Typography, CardActions, Button } from '@mui/material';

const POSItem: React.FC<{ item: any; onAddToCart: (item: any) => void }> = ({ item, onAddToCart }) => {
    return (
        <Card>
            <CardContent>
                <Typography variant="h5" component="div">
                    {item.name}
                </Typography>
                <Typography variant="body2" color="text.secondary">
                    ${item.price.toFixed(2)}
                </Typography>
            </CardContent>
            <CardActions>
                <Button variant="contained" color="primary" onClick={() => onAddToCart(item)}>
                    Add to Cart
                </Button>
            </CardActions>
        </Card>
    );
};

export default POSItem;