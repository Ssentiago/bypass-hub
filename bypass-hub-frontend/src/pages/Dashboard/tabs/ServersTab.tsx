// src/pages/Dashboard/tabs/ServersTab.tsx
import {useEffect, useState} from 'react';
import {Button, Table, Popconfirm, Space, Modal, Form, Input, Typography, Tag, Tooltip, Badge} from 'antd';
import {PlusOutlined, DeleteOutlined, CopyOutlined, ReloadOutlined} from '@ant-design/icons';
import {api} from '@/core/api/client';
import type {Server, ServerInbound, XuiInbound} from '@/core/api/modules/infrastructure/servers';
import {useTranslation} from 'react-i18next';

const {Title} = Typography;

const ServersTab = () => {
    const [servers, setServers] = useState<Server[]>([]);
    const [loading, setLoading] = useState(false);
    const [modalOpen, setModalOpen] = useState(false);
    const [submitting, setSubmitting] = useState(false);
    const [inbounds, setInbounds] = useState<Record<number, ServerInbound[]>>({});
    const [xuiInbounds, setXuiInbounds] = useState<Record<number, XuiInbound[]>>({});
    const [xuiLoading, setXuiLoading] = useState<Record<number, boolean>>({});
    const [addingInbound, setAddingInbound] = useState<Record<number, boolean>>({});
    const [statuses, setStatuses] = useState<Record<number, boolean | null>>({});
    const [statusLoading, setStatusLoading] = useState<Record<number, boolean>>({});
    const [form] = Form.useForm();
    const {t} = useTranslation();

    const checkStatus = async (id: number) => {
        setStatusLoading(prev => ({...prev, [id]: true}));
        try {
            const online = await api.infrastructure.servers.checkStatus(id);
            setStatuses(prev => ({...prev, [id]: online}));
        } finally {
            setStatusLoading(prev => ({...prev, [id]: false}));
        }
    };

    const load = async () => {
        setLoading(true);
        try {
            const data = await api.infrastructure.servers.list();
            setServers(data);
            data.forEach(s => checkStatus(s.id));
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        load();
    }, []);

    const handleCreate = async (values: { name: string; address: string; xui_api_key: string }) => {
        setSubmitting(true);
        try {
            const {id} = await api.infrastructure.servers.create(values);
            setModalOpen(false);
            form.resetFields();
            await load();
            checkStatus(id);
        } finally {
            setSubmitting(false);
        }
    };

    const handleDelete = async (id: number) => {
        await api.infrastructure.servers.delete(id);
        setServers(prev => prev.filter(s => s.id !== id));
        setStatuses(prev => {
            const next = {...prev};
            delete next[id];
            return next;
        });
    };

    const handleExpand = async (expanded: boolean, server: Server) => {
        if (!expanded) return;
        if (inbounds[server.id]) return;

        const data = await api.infrastructure.servers.listInbounds(server.id);
        setInbounds(prev => ({...prev, [server.id]: data}));

        setXuiLoading(prev => ({...prev, [server.id]: true}));
        try {
            const xui = await api.infrastructure.servers.listXuiInbounds(server.id);
            setXuiInbounds(prev => ({...prev, [server.id]: xui}));
        } finally {
            setXuiLoading(prev => ({...prev, [server.id]: false}));
        }
    };

    const handleAddInbound = async (serverId: number, inboundId: number) => {
        setAddingInbound(prev => ({...prev, [serverId]: true}));
        try {
            await api.infrastructure.servers.addInbound(serverId, inboundId);
            const data = await api.infrastructure.servers.listInbounds(serverId);
            setInbounds(prev => ({...prev, [serverId]: data}));
        } finally {
            setAddingInbound(prev => ({...prev, [serverId]: false}));
        }
    };

    const handleRemoveInbound = async (serverId: number, inboundId: number) => {
        await api.infrastructure.servers.removeInbound(serverId, inboundId);
        setInbounds(prev => ({
            ...prev,
            [serverId]: prev[serverId]?.filter(i => i.inbound_id !== inboundId) ?? [],
        }));
    };

    const expandedRowRender = (server: Server) => {
        const registered = inbounds[server.id] ?? [];
        const registeredIds = new Set(registered.map(i => i.inbound_id));
        const available = (xuiInbounds[server.id] ?? []).filter(i => !registeredIds.has(i.id));
        const isXuiLoading = xuiLoading[server.id] ?? false;

        return (
            <div style={{padding: '8px 16px'}}>
                <Space direction="vertical" style={{width: '100%'}} size={8}>
                    {registered.length === 0 && !isXuiLoading && (
                        <Typography.Text type="secondary">No inbounds registered.</Typography.Text>
                    )}
                    {registered.map(inbound => (
                        <Space key={inbound.id}>
                            <Tag color="blue">WireGuard</Tag>
                            <Typography.Text>inbound_id: {inbound.inbound_id}</Typography.Text>
                            <Popconfirm
                                title="Remove inbound?"
                                onConfirm={() => handleRemoveInbound(server.id, inbound.inbound_id)}
                                okText="Yes"
                                cancelText="No"
                            >
                                <Button danger icon={<DeleteOutlined/>} size="small"/>
                            </Popconfirm>
                        </Space>
                    ))}

                    {available.length > 0 && (
                        <>
                            <Typography.Text type="secondary" style={{fontSize: 12}}>
                                Available from 3x-ui:
                            </Typography.Text>
                            {available.map(i => (
                                <Space key={i.id}>
                                    <Tag>{i.protocol}</Tag>
                                    <Typography.Text>:{i.port}{i.remark && ` — ${i.remark}`}</Typography.Text>
                                    <Button
                                        size="small"
                                        icon={<PlusOutlined/>}
                                        loading={addingInbound[server.id]}
                                        onClick={() => handleAddInbound(server.id, i.id)}
                                    >
                                        Add
                                    </Button>
                                </Space>
                            ))}
                        </>
                    )}

                    {!isXuiLoading && available.length === 0 && registered.length === 0 && (
                        <Typography.Text type="warning">
                            No WireGuard inbounds found. Create one in 3x-ui first.
                        </Typography.Text>
                    )}
                </Space>
            </div>
        );
    };

    const columns = [
        {
            title: 'Status',
            key: 'status',
            width: 90,
            render: (_: unknown, record: Server) => {
                const online = statuses[record.id];
                const isLoading = statusLoading[record.id] ?? false;
                return (
                    <Space size={4}>
                        {isLoading ? (
                            <Badge status="processing" text="..."/>
                        ) : online === null || online === undefined ? (
                            <Badge status="default" text="—"/>
                        ) : online ? (
                            <Badge status="success" text="Online"/>
                        ) : (
                            <Badge status="error" text="Offline"/>
                        )}
                        <Tooltip title="Refresh">
                            <Button
                                icon={<ReloadOutlined/>}
                                size="small"
                                type="text"
                                loading={isLoading}
                                onClick={() => checkStatus(record.id)}
                            />
                        </Tooltip>
                    </Space>
                );
            },
        },
        {
            title: t('common.name'),
            dataIndex: 'name',
            key: 'name',
        },
        {
            title: 'Address',
            dataIndex: 'address',
            key: 'address',
        },
        {
            title: 'UUID',
            dataIndex: 'uuid',
            key: 'uuid',
            render: (uuid: string) => (
                <Space>
                    <Typography.Text code style={{fontSize: 11}}>
                        {uuid.slice(0, 8)}…
                    </Typography.Text>
                    <Tooltip title="Copy UUID">
                        <Button
                            icon={<CopyOutlined/>}
                            size="small"
                            onClick={() => navigator.clipboard.writeText(uuid)}
                        />
                    </Tooltip>
                </Space>
            ),
        },
        {
            title: '',
            key: 'actions',
            width: 60,
            render: (_: unknown, record: Server) => (
                <Popconfirm
                    title="Delete server?"
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
                <Title level={4} style={{margin: 0}}>Servers</Title>
                <Button type="primary" icon={<PlusOutlined/>} onClick={() => setModalOpen(true)}>
                    {t('common.add')}
                </Button>
            </Space>

            <Table
                rowKey="id"
                columns={columns}
                dataSource={servers}
                loading={loading}
                pagination={false}
                expandable={{
                    expandedRowRender,
                    onExpand: handleExpand,
                }}
            />

            <Modal
                title="Add Server"
                open={modalOpen}
                onCancel={() => {
                    setModalOpen(false);
                    form.resetFields();
                }}
                onOk={() => form.submit()}
                confirmLoading={submitting}
            >
                <Form form={form} layout="vertical" onFinish={handleCreate}>
                    <Form.Item name="name" label={t('common.name')}
                               rules={[{required: true, message: t('common.required')}]}>
                        <Input placeholder="Production"/>
                    </Form.Item>
                    <Form.Item name="address" label="Address"
                               rules={[{required: true, message: t('common.required')}]}>
                        <Input placeholder="https://your-3xui-server.com/path"/>
                    </Form.Item>
                    <Form.Item name="xui_api_key" label="API Key"
                               rules={[{required: true, message: t('common.required')}]}>
                        <Input.Password/>
                    </Form.Item>
                </Form>
            </Modal>
        </>
    );
};

export default ServersTab;