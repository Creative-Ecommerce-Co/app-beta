import React from 'react';
import SettingsForm from './SettingsForm';

const AccountSettings: React.FC = () => {
    const handleSave = (data) => {
        // Implement save functionality
    };

    return (
        <SettingsForm
            title="Account Settings"
            fields={[
                { name: 'username', label: 'Username', type: 'text' },
                { name: 'email', label: 'Email', type: 'email' },
            ]}
            onSave={handleSave}
        />
    );
};

export default AccountSettings;