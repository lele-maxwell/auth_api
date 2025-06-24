import React, { ReactNode } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate, Link } from 'react-router-dom';
import Login from './pages/Login';
import Register from './pages/Register';
import Profile from './pages/Profile';
import { isSessionValid } from './session';

function RequireAuth({ children }: { children: ReactNode }): React.ReactElement | null {
  return isSessionValid() ? (children as React.ReactElement) : <Navigate to="/login" replace />;
}

function App() {
  return (
    <Router>
      <nav>
        <Link to="/login">Login</Link> | <Link to="/register">Register</Link> | <Link to="/profile">Profile</Link>
      </nav>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route path="/register" element={<Register />} />
        <Route path="/profile" element={<RequireAuth><Profile /></RequireAuth>} />
        <Route path="/" element={<Navigate to="/login" replace />} />
      </Routes>
    </Router>
  );
}

export default App;
