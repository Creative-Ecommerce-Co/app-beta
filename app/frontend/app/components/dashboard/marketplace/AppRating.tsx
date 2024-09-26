import React from 'react';
import { Box, Typography } from '@mui/material';
import { Star, StarBorder } from '@mui/icons-material';

const AppRating: React.FC<{ rating: number }> = ({ rating }) => {
    return (
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
            {[1, 2, 3, 4, 5].map((value) => (
                value <= rating ? <Star key={value} color="primary" /> : <StarBorder key={value} color="primary" />
            ))}
            <Typography variant="body2" sx={{ marginLeft: 1 }}>
                {rating.toFixed(1)}
            </Typography>
        </Box>
    );
};

export default AppRating;