import React, { useEffect, useState } from 'react';
import { ProtectedApi, Configuration } from '../ts-client';
import { getSessionToken, clearSession } from '../session';
import { useNavigate } from 'react-router-dom';

const api = new ProtectedApi(new Configuration({ basePath: 'http://localhost:3000', accessToken: localStorage.getItem('accessToken') || '' }));

const Profile: React.FC = () => {
  const [user, setUser] = useState<any>(null);
  const [error, setError] = useState('');
  const navigate = useNavigate();

  useEffect(() => {
    const fetchProfile = async () => {
      try {
        const token = getSessionToken();
        if (!token) {
          clearSession();
          navigate('/login');
          return;
        }
        const res = await api.userRoute({ headers: { Authorization: `Bearer ${token}` } });
        setUser(res.data);
      } catch (err: any) {
        setError(err?.response?.data?.error || 'Failed to fetch profile');
        clearSession();
        navigate('/login');
      }
    };
    fetchProfile();
  }, [navigate]);

  if (error) return <div style={{color: 'red'}}>{error}</div>;
  if (!user) return <div>Loading...</div>;

  return (
    <div>
      <h2>Profile</h2>
      <div>Email: {user.email}</div>
      <div>First Name: {user.first_name}</div>
      <div>Last Name: {user.last_name}</div>
      <div>Role: {user.role}</div>
    </div>
  );
};

export default Profile; 