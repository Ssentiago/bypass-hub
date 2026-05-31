// src/pages/Dashboard/Dashboard.tsx
import {Routes, Route, Navigate} from 'react-router-dom';
import DashboardLayout from '@/pages/Dashboard/Layout';
import RoutesTab from './tabs/RoutesTab';
import GroupsTab from './tabs/GroupsTab';
import ServersTab from './tabs/ServersTab';
import MikrotiksTab from './tabs/MikrotiksTab';

const Dashboard = ({onLogout}: { onLogout: () => void }) => (
    <Routes>
        <Route element={<DashboardLayout onLogout={onLogout}/>}>
            <Route index element={<Navigate to="xui/routes"/>}/>
            <Route path="xui/routes" element={<RoutesTab scope="xui"/>}/>
            <Route path="xui/groups" element={<GroupsTab scope="xui"/>}/>
            <Route path="mikrotik/routes" element={<RoutesTab scope="mikrotik"/>}/>
            <Route path="mikrotik/groups" element={<GroupsTab scope="mikrotik"/>}/>
            <Route path="infrastructure/servers" element={<ServersTab/>}/>
            <Route path="infrastructure/mikrotiks" element={<MikrotiksTab/>}/>
        </Route>
    </Routes>
);

export default Dashboard;