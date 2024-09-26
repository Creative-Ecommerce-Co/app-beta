import React from 'react';
import { Grid } from '@mui/material';
import AppCard from './AppCard';

const AppList: React.FC<{ apps: any[] }> = ({ apps }) => {
    return (
        <Grid container spacing={3}>
            {apps.map(app => (
                <Grid item xs={12} sm={6} md={4} key={app.id}>
                    <AppCard app={app} />
                </Grid>
            ))}
        </Grid>
    );
};

export default AppList;