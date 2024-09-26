// assets
import { IconDashboard } from '@tabler/icons-react';

// constant
const icons = {
    IconDashboard
};

// ==============================|| DASHBOARD MENU ITEMS ||============================== //

interface MenuItem {
    id: string;
    title: string;
    type: string;
    url?: string;
    icon?: React.ReactNode;
    breadcrumbs?: boolean;
    children?: MenuItem[];
}

const dashboard: MenuItem = {
    id: 'dashboard',
    title: 'Dashboard',
    type: 'group',
    children: [
        {
            id: 'default',
            title: 'Dashboard',
            type: 'item',
            url: '/dashboard/default',
            icon: icons.IconDashboard,
            breadcrumbs: false
        }
    ]
};

export default dashboard;