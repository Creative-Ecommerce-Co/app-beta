import React from 'react';
import SettingsForm from './SettingsForm';

const GeneralSettings: React.FC = () => {
    const handleSave = (data) => {
        // Implement save functionality
    };

    return (
        <SettingsForm
            title="General Settings"
            fields={[
                { name: 'siteName', label: 'Site Name', type: 'text' },
                { name: 'siteDescription', label: 'Site Description', type: 'text' },
            ]}
            onSave={handleSave}
        />
    );
};

export default GeneralSettings;