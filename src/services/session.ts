import { invoke } from "@tauri-apps/api/core";

export const openSession = async (
  auth_token: string | null = null,
): Promise<void> => {
  if (!auth_token) {
    return await invoke("open_session");
  }

  return await invoke("open_session", { authToken: auth_token });
};
