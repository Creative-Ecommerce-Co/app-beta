import React from 'react';
import { Box, Typography, Grid } from '@mui/material';
import GeneralSettings from './GeneralSettings';
import AccountSettings from './AccountSettings';
import NotificationSettings from './NotificationSettings';
import SecuritySettings from './SecuritySettings';

const SettingsDashboard: React.FC = () => {
    return (
        <Box sx={{ padding: 2 }}>
            <Typography variant="h4" gutterBottom>
                Settings
            </Typography>
            <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                    <GeneralSettings />
                </Grid>
                <Grid item xs={12} md={6}>
                    <AccountSettings />
                </Grid>
                <Grid item xs={12} md={6}>
                    <NotificationSettings />
                </Grid>
                <Grid item xs={12} md={6}>
                    <SecuritySettings />
                </Grid>
            </Grid>
        </Box>
    );
};

export default SettingsDashboard;