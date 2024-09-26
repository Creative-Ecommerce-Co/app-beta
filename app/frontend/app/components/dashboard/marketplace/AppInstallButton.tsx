import React, { useState } from 'react';
import { Button, CircularProgress } from '@mui/material';
import { installApp } from 'api/marketplace'; // Assume this is an API call to install an app

const AppInstallButton: React.FC<{ appId: string }> = ({ appId }) => {
    const [loading, setLoading] = useState(false);
    const [installed, setInstalled] = useState(false);
    const [error, setError] = useState(null);

    const handleInstall = async () => {
        setLoading(true);
        setError(null);
        try {
            await installApp(appId);
            setInstalled(true);
        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    if (loading) {
        return <CircularProgress size={24} />;
    }

    if (installed) {
        return <Button variant="contained" color="success" disabled>Installed</Button>;
    }

    return (
        <>
            <Button variant="contained" color="primary" onClick={handleInstall}>
                Install
            </Button>
            {error && <Typography variant="body2" color="error">{error}</Typography>}
        </>
    );
};

export default AppInstallButton;