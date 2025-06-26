import React, { useState } from "react";
import { authApi } from "../api";
import { Link, useNavigate } from "react-router-dom";
import { protectedApi } from "../api";

export default function Login() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);
  const navigate = useNavigate();

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setMessage(null);
    try {
      const { data } = await authApi.login({ email, password });
      localStorage.setItem("token", data.token);
      setMessage({ type: "success", text: "Login successful! Loading your profile..." });
      // Fetch user profile after login
      const profileRes = await protectedApi.userRoute();
      setTimeout(() => {
        navigate("/profile", { state: { user: profileRes.data } });
      }, 800);
    } catch (err: any) {
      let errorMsg = "Login failed. Please check your credentials.";
      if (err && err.response && err.response.data && typeof err.response.data === "string") {
        errorMsg = err.response.data;
      } else if (err && err.response && err.response.data && err.response.data.message) {
        errorMsg = err.response.data.message;
      }
      setMessage({ type: "error", text: errorMsg });
      console.error("Login error:", err);
    }
  };

  return (
    <form className="auth-form" onSubmit={handleLogin}>
      <h2>Login</h2>
      {message && (
        <div className={`form-message ${message.type}`}>{message.text}</div>
      )}
      <input className="auth-input" placeholder="Email" value={email} onChange={e => setEmail(e.target.value)} />
      <input className="auth-input" placeholder="Password" type="password" value={password} onChange={e => setPassword(e.target.value)} />
      <button className="auth-btn" type="submit">Login</button>
      <p className="switch-link">Not registered? <Link to="/register">Register here</Link></p>
    </form>
  );
}
