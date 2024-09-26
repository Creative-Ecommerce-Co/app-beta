import React, { useState, useEffect } from 'react';
import { Box, Typography, CircularProgress } from '@mui/material';
import { Bar } from 'react-chartjs-2';
import { fetchTrafficData } from 'api/analytics'; // Assume this is an API call to fetch traffic data

const TrafficChart: React.FC = () => {
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    const [data, setData] = useState({
        labels: [],
        datasets: [
            {
                label: 'Traffic',
                data: [],
                backgroundColor: 'rgba(153,102,255,1)',
            },
        ],
    });

    useEffect(() => {
        const loadData = async () => {
            try {
                const response = await fetchTrafficData();
                setData({
                    labels: response.labels,
                    datasets: [
                        {
                            label: 'Traffic',
                            data: response.data,
                            backgroundColor: 'rgba(153,102,255,1)',
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
                Website Traffic
            </Typography>
            <Bar
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
                                text: 'Week',
                            },
                        },
                        y: {
                            display: true,
                            title: {
                                display: true,
                                text: 'Traffic',
                            },
                        },
                    },
                }}
            />
        </Box>
    );
};

export default TrafficChart;