const SESSION_KEY = 'user_session';
const SESSION_DURATION = 10 * 60 * 1000; // 10 minutes

export function setSession(token: string) {
  const expiresAt = Date.now() + SESSION_DURATION;
  localStorage.setItem(SESSION_KEY, JSON.stringify({ token, expiresAt }));
}

export function getSession() {
  const session = localStorage.getItem(SESSION_KEY);
  if (!session) return null;
  const { token, expiresAt } = JSON.parse(session);
  if (Date.now() > expiresAt) {
    clearSession();
    return null;
  }
  return { token, expiresAt };
}

export function clearSession() {
  localStorage.removeItem(SESSION_KEY);
}

export function isSessionValid() {
  return !!getSession();
}

export function getSessionToken() {
  const session = getSession();
  return session ? session.token : null;
} 