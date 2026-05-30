import {Routes, Route, Navigate} from 'react-router-dom';
import DashboardLayout from '@/pages/Dashboard/Layout';
import RoutesTab from '@/pages/Dashboard/RoutesTab';
import GroupsTab from '@/pages/Dashboard/GroupsTab';

const Dashboard = ({onLogout}: { onLogout: () => void }) => (
    <Routes>
        <Route element={<DashboardLayout onLogout={onLogout}/>}>
            <Route index element={<Navigate to="routes"/>}/>
            <Route path="routes" element={<RoutesTab/>}/>
            <Route path="groups" element={<GroupsTab/>}/>
        </Route>
    </Routes>
);

export default Dashboard;