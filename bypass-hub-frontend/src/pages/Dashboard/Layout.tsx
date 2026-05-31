// src/pages/Dashboard/Layout.tsx
import {Layout, Menu, Button, theme, Divider} from 'antd';
import {Outlet, useNavigate, useLocation} from 'react-router-dom';
import {
    GlobalOutlined,
    ApartmentOutlined,
    LogoutOutlined,
    CloudServerOutlined,
    ApiOutlined,
} from '@ant-design/icons';
import {api} from '@/core/api/client';
import {useTranslation} from 'react-i18next';

const {Sider, Header, Content} = Layout;

const DashboardLayout = ({onLogout}: { onLogout: () => void }) => {
    const navigate = useNavigate();
    const location = useLocation();
    const {token} = theme.useToken();
    const {i18n, t} = useTranslation();

    const XUI_ITEMS = [
        {key: '/xui/routes', icon: <GlobalOutlined/>, label: t('routes.title')},
        {key: '/xui/groups', icon: <ApartmentOutlined/>, label: t('groups.title')},
    ];

    const MIKROTIK_ITEMS = [
        {key: '/mikrotik/routes', icon: <GlobalOutlined/>, label: t('routes.title')},
        {key: '/mikrotik/groups', icon: <ApartmentOutlined/>, label: t('groups.title')},
    ];

    const INFRASTRUCTURE_ITEMS = [
        {key: '/infrastructure/servers', icon: <CloudServerOutlined/>, label: 'Servers'},
        {key: '/infrastructure/mikrotiks', icon: <ApiOutlined/>, label: 'Mikrotiks'},
    ];

    const handleLogout = async () => {
        await api.auth.logout();
        onLogout();
    };

    const sectionLabel = (text: string) => (
        <div style={{
            padding: '12px 16px 4px',
            fontSize: 11,
            fontWeight: 600,
            letterSpacing: '0.08em',
            textTransform: 'uppercase',
            color: token.colorTextTertiary,
        }}>
            {text}
        </div>
    );

    return (
        <Layout style={{minHeight: '100vh'}}>
            <Sider
                theme="dark"
                style={{
                    background: token.colorBgContainer,
                    borderRight: `1px solid ${token.colorBorderSecondary}`,
                }}
            >
                <div style={{
                    height: 64,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    fontWeight: 700,
                    fontSize: 16,
                    color: token.colorText,
                    borderBottom: `1px solid ${token.colorBorderSecondary}`,
                }}>
                    bypass-hub
                </div>

                {sectionLabel('3X-UI')}
                <Menu
                    mode="inline"
                    selectedKeys={[location.pathname]}
                    items={XUI_ITEMS}
                    style={{background: 'transparent', border: 'none'}}
                    onClick={({key}) => navigate(key)}
                />

                <Divider style={{margin: '8px 0'}}/>

                {sectionLabel('MikroTik')}
                <Menu
                    mode="inline"
                    selectedKeys={[location.pathname]}
                    items={MIKROTIK_ITEMS}
                    style={{background: 'transparent', border: 'none'}}
                    onClick={({key}) => navigate(key)}
                />

                <Divider style={{margin: '8px 0'}}/>

                {sectionLabel('Infrastructure')}
                <Menu
                    mode="inline"
                    selectedKeys={[location.pathname]}
                    items={INFRASTRUCTURE_ITEMS}
                    style={{background: 'transparent', border: 'none'}}
                    onClick={({key}) => navigate(key)}
                />
            </Sider>

            <Layout>
                <Header style={{
                    background: token.colorBgContainer,
                    borderBottom: `1px solid ${token.colorBorderSecondary}`,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'flex-end',
                    padding: '0 24px',
                    gap: 8,
                }}>
                    <Button onClick={() => i18n.changeLanguage(i18n.language === 'en' ? 'ru' : 'en')}>
                        {i18n.language === 'en' ? 'EN' : 'RU'}
                    </Button>
                    <Button icon={<LogoutOutlined/>} onClick={handleLogout} type="text">
                        {t('auth.logout')}
                    </Button>
                </Header>

                <Content style={{
                    margin: 24,
                    padding: 24,
                    background: token.colorBgContainer,
                    borderRadius: token.borderRadius,
                    minHeight: 'calc(100vh - 64px - 48px)',
                }}>
                    <Outlet/>
                </Content>
            </Layout>
        </Layout>
    );
};

export default DashboardLayout;