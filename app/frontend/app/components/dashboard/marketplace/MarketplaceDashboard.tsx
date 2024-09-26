import React, { useState, useEffect } from 'react';
import { Box, Grid, Typography, CircularProgress } from '@mui/material';
import AppList from './AppList';
import AppSearchBar from './AppSearchBar';
import AppCategoryFilter from './AppCategoryFilter';
import { fetchApps } from 'api/marketplace'; // Assume this is an API call to fetch apps

const MarketplaceDashboard: React.FC = () => {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [apps, setApps] = useState([]);
    const [searchQuery, setSearchQuery] = useState('');
    const [category, setCategory] = useState('');

    useEffect(() => {
        const loadData = async () => {
            try {
                const response = await fetchApps();
                setApps(response);
            } catch (err) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        loadData();
    }, []);

    const filteredApps = apps.filter(app => 
        app.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
        (category ? app.category === category : true)
    );

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
                App Marketplace
            </Typography>
            <AppSearchBar value={searchQuery} onChange={setSearchQuery} />
            <AppCategoryFilter value={category} onChange={setCategory} />
            <Grid container spacing={3}>
                <AppList apps={filteredApps} />
            </Grid>
        </Box>
    );
};

export default MarketplaceDashboard;