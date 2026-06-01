// src/pages/Dashboard/tabs/MikrotiksTab.tsx
import {useEffect, useState} from 'react';
import {
    Button, Table, Popconfirm, Space, Modal, Form, Input,
    Typography, Tag, Select, Tooltip,
} from 'antd';
import {
    PlusOutlined,
    DeleteOutlined,
    CopyOutlined,
    DownloadOutlined,
    KeyOutlined,
    CodeOutlined,
    SyncOutlined
} from '@ant-design/icons';
import {api} from '@/core/api/client';
import type {Mikrotik} from '@/core/api/modules/infrastructure/mikrotiks';
import type {Server, ServerInbound} from '@/core/api/modules/infrastructure/servers';

const {Title, Text} = Typography;

const MikrotiksTab = () => {
    const [mikrotiks, setMikrotiks] = useState<Mikrotik[]>([]);
    const [servers, setServers] = useState<Server[]>([]);
    const [serverInbounds, setServerInbounds] = useState<Record<number, ServerInbound[]>>({});
    const [loading, setLoading] = useState(false);
    const [modalOpen, setModalOpen] = useState(false);
    const [submitting, setSubmitting] = useState(false);
    const [keyModalId, setKeyModalId] = useState<number | null>(null);
    const [keyValue, setKeyValue] = useState('');
    const [keySubmitting, setKeySubmitting] = useState(false);
    const [inboundsLoading, setInboundsLoading] = useState(false);
    const [form] = Form.useForm<{ name: string; server_id: number; inbound_id: number }>();
    const selectedServerId = Form.useWatch('server_id', form);

    const load = async () => {
        setLoading(true);
        try {
            const [mt, sv] = await Promise.all([
                api.infrastructure.mikrotiks.list(),
                api.infrastructure.servers.list(),
            ]);
            setMikrotiks(mt);
            setServers(sv);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        load();
    }, []);

    const handleServerSelect = async (serverId: number) => {
        form.setFieldValue('inbound_id', undefined);
        if (serverInbounds[serverId]) return;
        setInboundsLoading(true);
        try {
            const data = await api.infrastructure.servers.listInbounds(serverId);
            setServerInbounds(prev => ({...prev, [serverId]: data}));
        } finally {
            setInboundsLoading(false);
        }
    };

    const handleRetry = async (id: number) => {
        try {
            await api.infrastructure.mikrotiks.retry(id);
            await load();
        } catch (e) {
            // можно message.error если подключён
        }
    };

    const handleCreate = async (values: { name: string; server_id: number; inbound_id: number }) => {
        setSubmitting(true);
        try {
            await api.infrastructure.mikrotiks.create(values);
            setModalOpen(false);
            form.resetFields();
            await load();
        } finally {
            setSubmitting(false);
        }
    };

    const handleDelete = async (id: number) => {
        await api.infrastructure.mikrotiks.delete(id);
        setMikrotiks(prev => prev.filter(m => m.id !== id));
    };

    const handleSetKey = async () => {
        if (!keyModalId || !keyValue.trim()) return;
        setKeySubmitting(true);
        try {
            await api.infrastructure.mikrotiks.setKey(keyModalId, keyValue.trim());
            setKeyModalId(null);
            setKeyValue('');
            await load();
        } finally {
            setKeySubmitting(false);
        }
    };

    const serverName = (id: number) => servers.find(s => s.id === id)?.name ?? `#${id}`;

    const columns = [
        {
            title: 'Name',
            dataIndex: 'name',
            key: 'name',
        },
        {
            title: 'Server',
            dataIndex: 'server_id',
            key: 'server_id',
            render: (id: number) => serverName(id),
        },
        {
            title: 'Inbound ID',
            dataIndex: 'inbound_id',
            key: 'inbound_id',
            render: (id: number) => <Text code>{id}</Text>,
        },
        {
            title: 'Status',
            dataIndex: 'status',
            key: 'status',
            render: (status: string) => (
                <Tag color={status === 'active' ? 'green' : 'orange'}>
                    {status === 'active' ? 'Active' : 'Pending key'}
                </Tag>
            ),
        },
        {
            title: 'Assigned IP',
            dataIndex: 'assigned_ip',
            key: 'assigned_ip',
            render: (ip: string | null) => ip ? <Text code>{ip}</Text> : <Text type="secondary">—</Text>,
        },
        {
            title: 'UUID',
            dataIndex: 'uuid',
            key: 'uuid',
            render: (uuid: string) => (
                <Space>
                    <Text code style={{fontSize: 11}}>{uuid.slice(0, 8)}…</Text>
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
            width: 120,
            render: (_: unknown, record: Mikrotik) => (
                <Space>
                    {record.status === 'pending_key' && (
                        <Tooltip title="Set public key">
                            <Button
                                icon={<KeyOutlined/>}
                                size="small"
                                onClick={() => {
                                    setKeyModalId(record.id);
                                    setKeyValue('');
                                }}
                            />
                        </Tooltip>
                    )}
                    {record.status === 'pending_key' && record.public_key && (
                        <Tooltip title="Retry 3x-ui sync">
                            <Button
                                icon={<SyncOutlined/>}
                                size="small"
                                onClick={() => handleRetry(record.id)}
                            />
                        </Tooltip>
                    )}
                    <Tooltip title="Download init.rsc">
                        <Button
                            icon={<DownloadOutlined/>}
                            size="small"
                            href={api.infrastructure.mikrotiks.scriptUrl(record.id)}
                            target="_blank"
                            rel="noreferrer"
                        />
                    </Tooltip>
                    {record.status === 'active' && (
                        <Tooltip title="Download agent.rsc">
                            <Button
                                icon={<CodeOutlined/>}
                                size="small"
                                href={api.infrastructure.mikrotiks.agentUrl(record.id)}
                                target="_blank"
                                rel="noreferrer"
                            />
                        </Tooltip>
                    )}
                    <Popconfirm
                        title="Delete mikrotik?"
                        onConfirm={() => handleDelete(record.id)}
                        okText="Yes"
                        cancelText="No"
                    >
                        <Button danger icon={<DeleteOutlined/>} size="small"/>
                    </Popconfirm>
                </Space>
            ),
        },
    ];

    const availableInbounds = selectedServerId ? (serverInbounds[selectedServerId] ?? []) : [];

    return (
        <>
            <Space style={{width: '100%', justifyContent: 'space-between', marginBottom: 16}}>
                <Title level={4} style={{margin: 0}}>MikroTiks</Title>
                <Button type="primary" icon={<PlusOutlined/>} onClick={() => setModalOpen(true)}>
                    Add
                </Button>
            </Space>

            <Table
                rowKey="id"
                columns={columns}
                dataSource={mikrotiks}
                loading={loading}
                pagination={false}
            />

            <Modal
                title="Add MikroTik"
                open={modalOpen}
                onCancel={() => {
                    setModalOpen(false);
                    form.resetFields();
                }}
                onOk={() => form.submit()}
                confirmLoading={submitting}
            >
                <Form form={form} layout="vertical" onFinish={handleCreate}>
                    <Form.Item name="name" label="Name" rules={[{required: true}]}>
                        <Input placeholder="Office-1"/>
                    </Form.Item>
                    <Form.Item name="server_id" label="Server" rules={[{required: true}]}>
                        <Select
                            placeholder="Select server"
                            options={servers.map(s => ({value: s.id, label: s.name}))}
                            onSelect={handleServerSelect}
                        />
                    </Form.Item>
                    <Form.Item name="inbound_id" label="WireGuard Inbound" rules={[{required: true}]}>
                        <Select
                            placeholder={selectedServerId ? 'Select inbound' : 'Select server first'}
                            disabled={!selectedServerId}
                            loading={inboundsLoading}
                            options={availableInbounds.map(i => ({
                                value: i.id,
                                label: `inbound_id: ${i.inbound_id}`,
                            }))}
                        />
                    </Form.Item>
                </Form>
            </Modal>

            <Modal
                title="Set WireGuard Public Key"
                open={keyModalId !== null}
                onCancel={() => setKeyModalId(null)}
                onOk={handleSetKey}
                confirmLoading={keySubmitting}
                okText="Submit"
            >
                <Input.TextArea
                    rows={3}
                    placeholder="Base64 WireGuard public key"
                    value={keyValue}
                    onChange={e => setKeyValue(e.target.value)}
                />
            </Modal>
        </>
    );
};

export default MikrotiksTab;