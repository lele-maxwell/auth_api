import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthApi, Configuration } from '../ts-client';
import { setSession } from '../session';

const api = new AuthApi(new Configuration({ basePath: 'http://localhost:3000' }));

const Login: React.FC = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    try {
      const res = await api.login({ email: username, password });
      localStorage.setItem('accessToken', res.data.token);
      setSession(res.data.token);
      navigate('/profile');
    } catch (err: any) {
      setError(err?.response?.data?.error || 'Login failed');
    }
  };

  return (
    <div>
      <h2>Login</h2>
      <form onSubmit={handleSubmit}>
        <input type="text" placeholder="Email" value={username} onChange={e => setUsername(e.target.value)} required />
        <input type="password" placeholder="Password" value={password} onChange={e => setPassword(e.target.value)} required />
        <button type="submit">Login</button>
      </form>
      {error && <div style={{color: 'red'}}>{error}</div>}
    </div>
  );
};

export default Login; 