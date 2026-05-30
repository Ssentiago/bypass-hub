import {useState} from 'react';
import {Button, Card, Form, Input, Typography, Alert, Space} from 'antd';
import {api, ApiError} from '@/core/api/client';
import {useTranslation} from 'react-i18next';
import i18n from '@/i18n';

const {Title} = Typography;

interface LoginForm {
    username: string;
    password: string;
}

const Login = () => {
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const {t} = useTranslation();

    const onFinish = async (values: LoginForm) => {
        setLoading(true);
        setError(null);
        try {
            await api.auth.login(values);
            window.location.href = '/';
        } catch (e) {
            if (e instanceof ApiError) {
                setError(e.message);
            } else {
                setError('Unexpected error');
            }
        } finally {
            setLoading(false);
        }
    };

    return (
        <div style={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            minHeight: '100vh',
            background: '#141414',
        }}>
            <div style={{width: 380}}>
                <div style={{textAlign: 'right', marginBottom: 8}}>
                    <Button
                        size="small"
                        type="text"
                        onClick={() => i18n.changeLanguage(i18n.language === 'en' ? 'ru' : 'en')}
                    >
                        {i18n.language === 'en' ? 'RU' : 'EN'}
                    </Button>
                </div>

                <Card>
                    <Title level={3} style={{textAlign: 'center', marginBottom: 24}}>
                        bypass-hub
                    </Title>

                    {error && (
                        <Alert message={error} type="error" showIcon style={{marginBottom: 16}}/>
                    )}

                    <Form layout="vertical" onFinish={onFinish} autoComplete="off">
                        <Form.Item label={t('auth.username')} name="username"
                                   rules={[{required: true, message: t('common.required')}]}>
                            <Input/>
                        </Form.Item>
                        <Form.Item label={t('auth.password')} name="password"
                                   rules={[{required: true, message: t('common.required')}]}>
                            <Input.Password/>
                        </Form.Item>
                        <Form.Item style={{marginBottom: 0}}>
                            <Button type="primary" htmlType="submit" loading={loading} block>
                                {t('auth.login')}
                            </Button>
                        </Form.Item>
                    </Form>
                </Card>
            </div>
        </div>
    );
};

export default Login;