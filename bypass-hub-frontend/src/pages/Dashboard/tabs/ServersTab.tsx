// src/pages/Dashboard/ServersTab.tsx
import {Typography, Empty} from 'antd';
import {CloudServerOutlined} from '@ant-design/icons';

const {Title, Text} = Typography;

const ServersTab = () => (
    <div>
        <Title level={4} style={{margin: '0 0 16px'}}>Servers</Title>
        <Empty
            image={<CloudServerOutlined style={{fontSize: 48, color: '#ccc'}}/>}
            imageStyle={{height: 60}}
            description={<Text type="secondary">Coming soon</Text>}
        />
    </div>
);

export default ServersTab;