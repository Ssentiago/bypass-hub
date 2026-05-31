// src/pages/Dashboard/MikrotiksTab.tsx
import {Typography, Empty} from 'antd';
import {ApiOutlined} from '@ant-design/icons';

const {Title, Text} = Typography;

const MikrotiksTab = () => (
    <div>
        <Title level={4} style={{margin: '0 0 16px'}}>MikroTiks</Title>
        <Empty
            image={<ApiOutlined style={{fontSize: 48, color: '#ccc'}}/>}
            imageStyle={{height: 60}}
            description={<Text type="secondary">Coming soon</Text>}
        />
    </div>
);

export default MikrotiksTab;