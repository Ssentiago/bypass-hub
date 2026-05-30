import {BrowserRouter, Route, Routes, Navigate} from 'react-router-dom';
import {useState, useEffect} from 'react';
import Login from '@/pages/Login/Login';
import Dashboard from '@/pages/Dashboard/Dashboard';
import {api} from '@/core/api/client';

const App = () => {
    const [authed, setAuthed] = useState<boolean | null>(null);

    useEffect(() => {
        api.auth.me()
            .then(() => setAuthed(true))
            .catch(() => setAuthed(false));
    }, []);

    if (authed === null) return null;

    return (
        <BrowserRouter>
            <Routes>
                <Route
                    path="/login"
                    element={authed ? <Navigate to="/"/> : <Login/>}
                />
                <Route
                    path="/*"
                    element={authed ? <Dashboard onLogout={() => setAuthed(false)}/> : <Navigate to="/login"/>}
                />
            </Routes>
        </BrowserRouter>
    );
};

export default App;