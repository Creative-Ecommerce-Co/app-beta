import React from 'react';
import { Card, CardContent, Typography, CardActions, Box } from '@mui/material';
import AppInstallButton from './AppInstallButton';
import AppRating from './AppRating';

const AppCard: React.FC<{ app: any }> = ({ app }) => {
    return (
        <Card>
            <CardContent>
                <Box sx={{ display: 'flex', alignItems: 'center', marginBottom: 2 }}>
                    <img src={app.icon} alt={`${app.name} icon`} style={{ width: 50, height: 50, marginRight: 16 }} />
                    <Typography variant="h5" component="div">
                        {app.name}
                    </Typography>
                </Box>
                <Typography variant="body2" color="text.secondary" sx={{ marginBottom: 2 }}>
                    {app.description}
                </Typography>
                <AppRating rating={app.rating} />
            </CardContent>
            <CardActions>
                <AppInstallButton appId={app.id} />
            </CardActions>
        </Card>
    );
};

export default AppCard;