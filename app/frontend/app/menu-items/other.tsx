// assets
import { IconBrandChrome, IconHelp } from '@tabler/icons-react';

// constant
const icons = {
    IconBrandChrome,
    IconHelp
};

// ==============================|| SAMPLE PAGE & DOCUMENTATION MENU ITEMS ||============================== //

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
}

const other: MenuItem = {
    id: 'sample-docs-roadmap',
    type: 'group',
    children: [
        {
            id: 'sample-page',
            title: 'Sample Page',
            type: 'item',
            url: '/sample-page',
            icon: icons.IconBrandChrome,
            breadcrumbs: false
        },
        {
            id: 'documentation',
            title: 'Documentation',
            type: 'item',
            url: 'https://codedthemes.gitbook.io/berry/',
            icon: icons.IconHelp,
            external: true,
            target: true
        }
    ]
};

export default other;