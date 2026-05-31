// src/pages/Dashboard/GroupsTab.tsx
import {useEffect, useState} from 'react';
import {Button, Table, Popconfirm, Space, Modal, Form, Input, Typography} from 'antd';
import {PlusOutlined, DeleteOutlined} from '@ant-design/icons';
import {api} from '@/core/api/client';
import type {Group} from '@/core/api/modules/groups.ts';
import {useTranslation} from 'react-i18next';

const {Title} = Typography;

interface Props {
    scope: 'xui' | 'mikrotik';
}

const GroupsTab = ({scope}: Props) => {
    const [groups, setGroups] = useState<Group[]>([]);
    const [loading, setLoading] = useState(false);
    const [modalOpen, setModalOpen] = useState(false);
    const [submitting, setSubmitting] = useState(false);
    const [form] = Form.useForm();
    const {t} = useTranslation();

    const scopeApi = api[scope];

    const load = async () => {
        setLoading(true);
        try {
            const data = await scopeApi.groups.list();
            setGroups(data);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        load();
    }, [scope]);

    const handleDelete = async (id: number) => {
        await scopeApi.groups.delete(id);
        setGroups(prev => prev.filter(g => g.id !== id));
    };

    const handleCreate = async (values: { name: string; description?: string }) => {
        setSubmitting(true);
        try {
            await scopeApi.groups.create(values);
            setModalOpen(false);
            form.resetFields();
            await load();
        } finally {
            setSubmitting(false);
        }
    };

    const columns = [
        {title: t('common.name'), dataIndex: 'name', key: 'name'},
        {title: t('common.description'), dataIndex: 'description', key: 'description'},
        {
            title: '',
            key: 'actions',
            width: 60,
            render: (_: unknown, record: Group) => (
                <Popconfirm
                    title="Delete group?"
                    onConfirm={() => handleDelete(record.id)}
                    okText="Yes"
                    cancelText="No"
                >
                    <Button danger icon={<DeleteOutlined/>} size="small"/>
                </Popconfirm>
            ),
        },
    ];

    return (
        <>
            <Space style={{width: '100%', justifyContent: 'space-between', marginBottom: 16}}>
                <Title level={4} style={{margin: 0}}>{t('groups.title')}</Title>
                <Button type="primary" icon={<PlusOutlined/>} onClick={() => setModalOpen(true)}>
                    {t('common.add')}
                </Button>
            </Space>

            <Table
                rowKey="id"
                columns={columns}
                dataSource={groups}
                loading={loading}
                pagination={{pageSize: 20}}
            />

            <Modal
                title="Add Group"
                open={modalOpen}
                onCancel={() => {
                    setModalOpen(false);
                    form.resetFields();
                }}
                onOk={() => form.submit()}
                confirmLoading={submitting}
            >
                <Form form={form} layout="vertical" onFinish={handleCreate}>
                    <Form.Item name="name" label="Name" rules={[{required: true, message: 'Required'}]}>
                        <Input/>
                    </Form.Item>
                    <Form.Item name="description" label="Description">
                        <Input.TextArea rows={2}/>
                    </Form.Item>
                </Form>
            </Modal>
        </>
    );
};

export default GroupsTab;