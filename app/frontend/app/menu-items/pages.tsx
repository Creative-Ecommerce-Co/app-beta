// assets
import { IconKey } from '@tabler/icons-react';

// constant
const icons = {
    IconKey
};

// ==============================|| EXTRA PAGES MENU ITEMS ||============================== //

interface MenuItem {
    id: string;
    title: string;
    type: string;
    url?: string;
    icon?: React.ReactNode;
    breadcrumbs?: boolean;
    external?: boolean;
    target?: boolean;
    children?: MenuItem[];
    caption?: string;
}

const pages: MenuItem = {
    id: 'pages',
    title: 'Pages',
    caption: 'Pages Caption',
    type: 'group',
    children: [
        {
            id: 'authentication',
            title: 'Authentication',
            type: 'collapse',
            icon: icons.IconKey,
            children: [
                {
                    id: 'login3',
                    title: 'Login',
                    type: 'item',
                    url: '/pages/login/login3',
                    target: true
                },
                {
                    id: 'register3',
                    title: 'Register',
                    type: 'item',
                    url: '/pages/register/register3',
                    target: true
                }
            ]
        }
    ]
};

export default pages;
