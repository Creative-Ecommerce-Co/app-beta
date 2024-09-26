import React from 'react';
import SettingsForm from './SettingsForm';

const NotificationSettings: React.FC = () => {
    const handleSave = (data) => {
        // Implement save functionality
    };

    return (
        <SettingsForm
            title="Notification Settings"
            fields={[
                { name: 'emailNotifications', label: 'Email Notifications', type: 'checkbox' },
                { name: 'smsNotifications', label: 'SMS Notifications', type: 'checkbox' },
            ]}
            onSave={handleSave}
        />
    );
};

export default NotificationSettings;