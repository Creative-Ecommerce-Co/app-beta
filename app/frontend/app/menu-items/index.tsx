import dashboard from './dashboard';
import pages from './pages';
import utilities from './utilities';
import other from './other';

// ==============================|| MENU ITEMS ||============================== //

interface MenuItem {
    id: string;
    title: string;
    type: string;
    url?: string;
    icon?: React.ReactNode;
    breadcrumbs?: boolean;
    children?: MenuItem[];
}

const menuItems: { items: MenuItem[] } = {
    items: [dashboard, pages, utilities, other]
};

export default menuItems;