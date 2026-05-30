import {StrictMode} from 'react';
import {createRoot} from 'react-dom/client';
import {ConfigProvider, theme} from 'antd';
import App from './App';
import './index.css';
import '@/i18n';

createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <ConfigProvider theme={{
            algorithm: theme.darkAlgorithm,
            components: {
                Collapse: {
                    motionDurationMid: '0ms',
                    motionDurationSlow: '0ms',
                }
            }
        }}>
            <App/>
        </ConfigProvider>
    </StrictMode>
);