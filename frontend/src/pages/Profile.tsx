import React, { useEffect, useState } from "react";
import { protectedApi } from "../api";
import type { User } from "@maxwell/auth-client";
import { useNavigate, useLocation } from "react-router-dom";

export default function Profile() {
  const [user, setUser] = useState<User | null>(null);
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    const token = localStorage.getItem("token");
    if (!token) {
      navigate("/login");
      return;
    }
    if (location.state && (location.state as any).user) {
      setUser((location.state as any).user);
      return;
    }
    protectedApi.userRoute()
      .then(res => setUser(res.data))
      .catch(() => {
        navigate("/login");
      });
  }, [navigate, location.state]);

  return (
    <div className="profile-container">
      <h2>Profile</h2>
      {user ? (
        <pre className="profile-data">{JSON.stringify(user, null, 2)}</pre>
      ) : (
        <p>Loading...</p>
      )}
    </div>
  );
}
