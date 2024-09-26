import React, { useState, useEffect } from 'react';
import { Box, Typography, CircularProgress, Grid } from '@mui/material';
import SalesChart from './SalesChart';
import TrafficChart from './TrafficChart';
import ConversionRate from './ConversionRate';
import { fetchAnalyticsData } from 'api/analytics'; // Assume this is an API call to fetch analytics data

const AnalyticsDashboard: React.FC = () => {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [data, setData] = useState({
        sales: [],
        traffic: [],
        conversionRate: 0,
    });

    useEffect(() => {
        const loadData = async () => {
            try {
                const response = await fetchAnalyticsData();
                setData(response);
            } catch (err) {
                setError(err.message);
            } finally {
                setLoading(false);
            }
        };

        loadData();
    }, []);

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
                Analytics Dashboard
            </Typography>
            <Grid container spacing={3}>
                <Grid item xs={12} md={6}>
                    <SalesChart data={data.sales} />
                </Grid>
                <Grid item xs={12} md={6}>
                    <TrafficChart data={data.traffic} />
                </Grid>
                <Grid item xs={12}>
                    <ConversionRate rate={data.conversionRate} />
                </Grid>
            </Grid>
        </Box>
    );
};

export default AnalyticsDashboard;