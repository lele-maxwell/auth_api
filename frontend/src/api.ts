import { Configuration, AuthApi, ProtectedApi} from '@maxwell/auth-client';

const token = localStorage.getItem('token');

export const apiConfig = new Configuration({
  basePath: 'http://localhost:3000', // Rust API URL
  accessToken: token ?? undefined,
});

export const authApi = new AuthApi(apiConfig);
export const userApi = new ProtectedApi(apiConfig); // Example: for profile fetch
