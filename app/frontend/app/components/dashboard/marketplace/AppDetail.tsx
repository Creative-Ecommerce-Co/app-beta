import React, { useState, useEffect } from 'react';
import { Box, Typography, CircularProgress, Button } from '@mui/material';
import { useParams } from 'react-router-dom';
import { fetchAppDetail } from 'api/marketplace'; // Assume this is an API call to fetch app details
import AppRating from './AppRating';
import AppInstallButton from './AppInstallButton';

const AppDetail: React.FC = () => {
    const { appId } = useParams();
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [app, setApp] = useState(null);

    useEffect(() => {
        const loadData = async () => {
            try {
                const response = await fetchAppDetail(appId);
                setApp(response);
            } catch (err) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        loadData();
    }, [appId]);

    if (loading) {
        return (
            <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
                <CircularProgress />
            </Box>
        );
    }

    if (error) {
        return (
            <Box sx={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
                <Typography variant="h6" color="error">
                    {error}
                </Typography>
            </Box>
        );
    }

    return (
        <Box sx={{ padding: 2 }}>
            <Typography variant="h4" gutterBottom>
                {app.name}
            </Typography>
            <Box sx={{ display: 'flex', alignItems: 'center', marginBottom: 2 }}>
                <img src={app.icon} alt={`${app.name} icon`} style={{ width: 50, height: 50, marginRight: 16 }} />
                <AppRating rating={app.rating} />
            </Box>
            <Typography variant="body1" gutterBottom>
                {app.description}
            </Typography>
            <Typography variant="body2" color="text.secondary" gutterBottom>
                {app.details}
            </Typography>
            <AppInstallButton appId={app.id} />
        </Box>
    );
};

export default AppDetail;