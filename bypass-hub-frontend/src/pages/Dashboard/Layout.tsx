import {Layout, Menu, Button, theme} from 'antd';
import {Outlet, useNavigate, useLocation} from 'react-router-dom';
import {GlobalOutlined, ApartmentOutlined, LogoutOutlined} from '@ant-design/icons';
import {api} from '@/core/api/client';
import {useTranslation} from "react-i18next";

const {Sider, Header, Content} = Layout;


const DashboardLayout = ({onLogout}: { onLogout: () => void }) => {
    const navigate = useNavigate();
    const location = useLocation();
    const {token} = theme.useToken();

    const {i18n, t} = useTranslation();

    const MENU_ITEMS = [
        {key: '/routes', icon: <GlobalOutlined/>, label: t('routes.title')},
        {key: '/groups', icon: <ApartmentOutlined/>, label: t('groups.title')},
    ];


    const handleLogout = async () => {
        await api.auth.logout();
        onLogout();
    };

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
                <Menu
                    mode="inline"
                    selectedKeys={[location.pathname]}
                    items={MENU_ITEMS}
                    style={{background: 'transparent', border: 'none', marginTop: 8}}
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
                }}>
                    <Button onClick={() => i18n.changeLanguage(i18n.language === 'en' ? 'ru' : 'en')}>
                        {i18n.language === 'en' ? 'EN' : 'RU'}
                    </Button>
                    <Button
                        icon={<LogoutOutlined/>}
                        onClick={handleLogout}
                        type="text"
                    >
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