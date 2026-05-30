import {useEffect, useState} from 'react';
import {Collapse, Button, Table, Popconfirm, Space, Modal, Form, Input, Select, Typography, Tag} from 'antd';
import {PlusOutlined, DeleteOutlined} from '@ant-design/icons';
import {api} from '@/core/api/client';
import type {Route, GroupedRoutes} from '@/core/api/modules/routes';
import type {Group} from '@/core/api/modules/groups';
import {useTranslation} from "react-i18next";

const {Title, Text} = Typography;

const IP_REGEX = /^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$/;

const parseRoutes = (text: string) =>
    text.split('\n')
        .map(line => line.trim())
        .filter(Boolean)
        .map(value => ({value, type: IP_REGEX.test(value) ? 'ip' as const : 'domain' as const}));


const RoutesTab = () => {
    const [grouped, setGrouped] = useState<GroupedRoutes[]>([]);
    const [groups, setGroups] = useState<Group[]>([]);
    const [loading, setLoading] = useState(false);
    const [modalOpen, setModalOpen] = useState(false);
    const [bulkOpen, setBulkOpen] = useState(false);
    const [submitting, setSubmitting] = useState(false);
    const [bulkSubmitting, setBulkSubmitting] = useState(false);
    const [bulkText, setBulkText] = useState('');
    const [form] = Form.useForm();
    const [bulkForm] = Form.useForm();

    const {t} = useTranslation()


    const columns = (onDelete: (id: number) => void) => [
        {title: t("common.value"), dataIndex: 'value', key: 'value'},
        {
            title: t("common.type"),
            dataIndex: 'type',
            key: 'type',
            width: 100,
            render: (type: string) => (
                <Tag color={type === 'ip' ? 'blue' : 'green'}>
                    {type === "ip" ? t('routes.ip') : t("routes.domain")}
                </Tag>
            ),
        },
        {
            title: '',
            key: 'actions',
            width: 60,
            render: (_: unknown, record: Route) => (
                <Popconfirm
                    title="Delete route?"
                    onConfirm={() => onDelete(record.id)}
                    okText="Yes"
                    cancelText="No"
                >
                    <Button danger icon={<DeleteOutlined/>} size="small"/>
                </Popconfirm>
            ),
        },
    ];


    const load = async () => {
        setLoading(true);
        try {
            const [groupedData, groupsData] = await Promise.all([
                api.routes.listGrouped(),
                api.groups.list(),
            ]);
            setGrouped(groupedData);
            setGroups(groupsData);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        load();
    }, []);

    const handleDelete = async (id: number) => {
        await api.routes.delete(id);
        await load();
    };

    const handleCreate = async (values: { value: string; type: 'domain' | 'ip'; group_ids?: number[] }) => {
        setSubmitting(true);
        try {
            await api.routes.create(values);
            setModalOpen(false);
            form.resetFields();
            await load();
        } finally {
            setSubmitting(false);
        }
    };

    const handleBulkCreate = async (values: { group_ids?: number[] }) => {
        const routes = parseRoutes(bulkText);
        if (!routes.length) return;
        setBulkSubmitting(true);
        try {
            await api.routes.bulkCreate({routes, group_ids: values.group_ids});
            setBulkOpen(false);
            setBulkText('');
            bulkForm.resetFields();
            await load();
        } finally {
            setBulkSubmitting(false);
        }
    };

    const groupOptions = groups.map(g => ({value: g.id, label: g.name}));

    const items = grouped.map((section, idx) => ({
        key: idx,
        label: (
            <Space>
                <span>{section.group ? section.group.name : t('routes.ungrouped')}</span>
                <Tag>{section.routes.length}</Tag>
            </Space>
        ),
        children: (
            <Table
                virtual
                rowKey="id"
                size="small"
                columns={columns(handleDelete)}
                dataSource={section.routes}
                pagination={false}
                scroll={{y: 400}}

            />
        ),
    }));

    return (
        <>
            <Space style={{width: '100%', justifyContent: 'space-between', marginBottom: 16}}>
                <Title level={4} style={{margin: 0}}>{t('routes.title')}</Title>
                <Space>
                    <Button icon={<PlusOutlined/>} onClick={() => setBulkOpen(true)}>{t('common.bulk_add')}</Button>
                    <Button type="primary" icon={<PlusOutlined/>}
                            onClick={() => setModalOpen(true)}>{t('common.add')}</Button>
                </Space>
            </Space>

            <Collapse
                items={items}
            />

            <Modal
                title={t('common.add')}
                open={modalOpen}
                onCancel={() => {
                    setModalOpen(false);
                    form.resetFields();
                }}
                onOk={() => form.submit()}
                confirmLoading={submitting}
            >
                <Form form={form} layout="vertical" onFinish={handleCreate}>
                    <Form.Item name="type" label="Type" rules={[{required: true}]} initialValue="domain">
                        <Select options={[{value: 'domain', label: 'Domain'}, {value: 'ip', label: 'IP'}]}/>
                    </Form.Item>
                    <Form.Item name="value" label="Value" rules={[{required: true, message: 'Required'}]}>
                        <Input placeholder="example.com or 1.2.3.4"/>
                    </Form.Item>
                    <Form.Item name="group_ids" label="Groups">
                        <Select mode="multiple" options={groupOptions} placeholder="Optional"/>
                    </Form.Item>
                </Form>
            </Modal>

            <Modal
                title={t('common.bulk_add')}
                open={bulkOpen}
                onCancel={() => {
                    setBulkOpen(false);
                    setBulkText('');
                    bulkForm.resetFields();
                }}
                onOk={() => bulkForm.submit()}
                confirmLoading={bulkSubmitting}
            >
                <Form form={bulkForm} layout="vertical" onFinish={handleBulkCreate}>
                    <Form.Item label={t('routes.title')}>
                        <Input.TextArea
                            rows={12}
                            placeholder={'example.com\n1.2.3.4\ngoogle.com'}
                            value={bulkText}
                            onChange={e => setBulkText(e.target.value)}
                        />
                        <div style={{marginTop: 4, fontSize: 12, color: '#666'}}>
                            {t("routes.bulk_hint")}
                        </div>
                    </Form.Item>
                    <Form.Item name="group_ids" label={t('groups.title')}>
                        <Select mode="multiple" options={groupOptions} placeholder={t("common.optional")}/>
                    </Form.Item>
                </Form>
            </Modal>
        </>
    );
};

export default RoutesTab;