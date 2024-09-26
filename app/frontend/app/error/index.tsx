import { useSelector } from 'react-redux';
import { RootState } from 'store/types'; // Assuming you have a RootState type defined for your Redux store

// material-ui
import { CssBaseline, StyledEngineProvider } from '@mui/material';
import { ThemeProvider } from '@mui/material/styles';

// project imports
import NavigationScroll from 'layout/NavigationScroll';
import theme from 'themes';
import ErrorPage from './ErrorPage';

const Error: React.FC = () => {
    const customization = useSelector((state: RootState) => state.customization);
    return (
        <StyledEngineProvider injectFirst>
            <ThemeProvider theme={theme(customization)}>
                <CssBaseline />
                <NavigationScroll>
                    <ErrorPage />
                </NavigationScroll>
            </ThemeProvider>
        </StyledEngineProvider>
    );
};

export default Error;