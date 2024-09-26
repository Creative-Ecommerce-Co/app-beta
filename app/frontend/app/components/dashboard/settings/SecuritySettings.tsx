import React from 'react';
import SettingsForm from './SettingsForm';

const SecuritySettings: React.FC = () => {
    const handleSave = (data) => {
        // Implement save functionality
    };

    return (
        <SettingsForm
            title="Security Settings"
            fields={[
                { name: 'password', label: 'Password', type: 'password' },
                { name: 'twoFactorAuth', label: 'Two-Factor Authentication', type: 'checkbox' },
            ]}
            onSave={handleSave}
        />
    );
};

export default SecuritySettings;