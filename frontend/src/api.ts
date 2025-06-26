import {
  AuthApi,
  ProtectedApi,
  Configuration
} from "@maxwell/auth-client";

const config = new Configuration({
  basePath: "http://localhost:3000", // Update to your backend URL
  accessToken: () => localStorage.getItem("token") || ""
});

export const authApi = new AuthApi(config);
export const protectedApi = new ProtectedApi(config);
