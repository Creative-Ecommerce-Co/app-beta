// app/frontend/app/components/dashboard/inventory/InventoryList.tsx

import React from 'react';
import { Box, Typography, List, ListItem, ListItemText, Button } from '@mui/material';

interface InventoryItem {
    id: string;
    name: string;
    quantity: number;
    price: number;
}

interface InventoryListProps {
    items: InventoryItem[];
    onEdit: (id: string) => void;
    onDelete: (id: string) => void;
}

const InventoryList: React.FC<InventoryListProps> = ({ items, onEdit, onDelete }) => {
    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Inventory List
            </Typography>
            <List>
                {items.map(item => (
                    <ListItem key={item.id} secondaryAction={
                        <>
                            <Button onClick={() => onEdit(item.id)} color="primary">Edit</Button>
                            <Button onClick={() => onDelete(item.id)} color="secondary">Delete</Button>
                        </>
                    }>
                        <ListItemText
                            primary={item.name}
                            secondary={`Quantity: ${item.quantity}, Price: $${item.price.toFixed(2)}`}
                        />
                    </ListItem>
                ))}
            </List>
        </Box>
    );
};

export default InventoryList;