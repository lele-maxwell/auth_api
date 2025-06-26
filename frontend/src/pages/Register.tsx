import React, { useState } from "react";
import { authApi } from "../api";
import { Link, useNavigate } from "react-router-dom";
import { protectedApi } from "../api";

export default function Register() {
  const [form, setForm] = useState({
    email: "",
    password: "",
    first_name: "",
    last_name: "",
  });
  const [message, setMessage] = useState<{ type: "success" | "error"; text: string } | null>(null);
  const navigate = useNavigate();

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setMessage(null);
    try {
      await authApi.register(form);
      setMessage({ type: "success", text: "Registration successful! Redirecting to login..." });
      setTimeout(() => {
        navigate("/login");
      }, 1200);
    } catch (err) {
      setMessage({ type: "error", text: "Registration failed. Please try again." });
    }
  };

  return (
    <form className="auth-form" onSubmit={handleSubmit}>
      <h2>Register</h2>
      {message && (
        <div className={`form-message ${message.type}`}>{message.text}</div>
      )}
      <input className="auth-input" placeholder="Email" value={form.email} onChange={e => setForm({ ...form, email: e.target.value })} />
      <input className="auth-input" placeholder="First Name" value={form.first_name} onChange={e => setForm({ ...form, first_name: e.target.value })} />
      <input className="auth-input" placeholder="Last Name" value={form.last_name} onChange={e => setForm({ ...form, last_name: e.target.value })} />
      <input className="auth-input" placeholder="Password" type="password" value={form.password} onChange={e => setForm({ ...form, password: e.target.value })} />
      <button className="auth-btn" type="submit">Register</button>
      <p className="switch-link">Already have an account? <Link to="/login">Login here</Link></p>
    </form>
  );
}
