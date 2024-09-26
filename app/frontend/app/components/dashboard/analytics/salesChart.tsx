import React, { useState, useEffect } from 'react';
import { Box, Typography, CircularProgress } from '@mui/material';
import { Line } from 'react-chartjs-2';
import { fetchSalesData } from 'api/analytics'; // Assume this is an API call to fetch sales data

const SalesChart: React.FC = () => {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [data, setData] = useState({
        labels: [],
        datasets: [
            {
                label: 'Sales',
                data: [],
                fill: false,
                backgroundColor: 'rgba(75,192,192,1)',
                borderColor: 'rgba(75,192,192,1)',
            },
        ],
    });

    useEffect(() => {
        const loadData = async () => {
            try {
                const response = await fetchSalesData();
                setData({
                    labels: response.labels,
                    datasets: [
                        {
                            label: 'Sales',
                            data: response.data,
                            fill: false,
                            backgroundColor: 'rgba(75,192,192,1)',
                            borderColor: 'rgba(75,192,192,1)',
                        },
                    ],
                });
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
        <Box sx={{ marginBottom: 4 }}>
            <Typography variant="h6" gutterBottom>
                Sales Over Time
            </Typography>
            <Line
                data={data}
                options={{
                    responsive: true,
                    plugins: {
                        legend: {
                            display: true,
                            position: 'top',
                        },
                        tooltip: {
                            mode: 'index',
                            intersect: false,
                        },
                    },
                    hover: {
                        mode: 'nearest',
                        intersect: true,
                    },
                    scales: {
                        x: {
                            display: true,
                            title: {
                                display: true,
                                text: 'Month',
                            },
                        },
                        y: {
                            display: true,
                            title: {
                                display: true,
                                text: 'Sales',
                            },
                        },
                    },
                }}
            />
        </Box>
    );
};

export default SalesChart;