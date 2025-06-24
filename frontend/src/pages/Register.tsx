import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { AuthApi, Configuration } from '@maxwell/auth-client';

const api = new AuthApi(new Configuration({ basePath: 'http://localhost:3000' }));

const Register: React.FC = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [firstName, setFirstName] = useState('');
  const [lastName, setLastName] = useState('');
  const [error, setError] = useState('');
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    try {
      await api.register({ email: username, password, first_name: firstName, last_name: lastName });
      navigate('/login');
    } catch (err: any) {
      setError(err?.response?.data?.error || 'Registration failed');
    }
  };

  return (
    <div>
      <h2>Register</h2>
      <form onSubmit={handleSubmit}>
        <input type="text" placeholder="First Name" value={firstName} onChange={e => setFirstName(e.target.value)} required />
        <input type="text" placeholder="Last Name" value={lastName} onChange={e => setLastName(e.target.value)} required />
        <input type="text" placeholder="Email" value={username} onChange={e => setUsername(e.target.value)} required />
        <input type="password" placeholder="Password" value={password} onChange={e => setPassword(e.target.value)} required />
        <button type="submit">Register</button>
      </form>
      {error && <div style={{color: 'red'}}>{error}</div>}
    </div>
  );
};

export default Register; 