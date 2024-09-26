// app/frontend/app/components/dashboard/promotions/PromotionsList.tsx

import React from 'react';
import { Box, Typography, List, ListItem, ListItemText, Button } from '@mui/material';

interface Promotion {
    id: string;
    title: string;
    discount: number;
    startDate: string;
    endDate: string;
}

interface PromotionsListProps {
    promotions: Promotion[];
    onEdit: (id: string) => void;
    onDelete: (id: string) => void;
}

const PromotionsList: React.FC<PromotionsListProps> = ({ promotions, onEdit, onDelete }) => {
    return (
        <Box>
            <Typography variant="h4" gutterBottom>
                Promotions List
            </Typography>
            <List>
                {promotions.map(promotion => (
                    <ListItem key={promotion.id} secondaryAction={
                        <>
                            <Button onClick={() => onEdit(promotion.id)} color="primary">Edit</Button>
                            <Button onClick={() => onDelete(promotion.id)} color="secondary">Delete</Button>
                        </>
                    }>
                        <ListItemText
                            primary={promotion.title}
                            secondary={`Discount: ${promotion.discount}%, Start: ${promotion.startDate}, End: ${promotion.endDate}`}
                        />
                    </ListItem>
                ))}
            </List>
        </Box>
    );
};

export default PromotionsList;