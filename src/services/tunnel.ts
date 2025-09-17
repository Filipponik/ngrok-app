import { invoke } from "@tauri-apps/api/core";

export interface Tunnel {
  id: string,
  url: string,
}

export const getTunnels = async (): Promise<Tunnel[]> => {
  return await invoke('tunnel_list');
}

export const closeTunnel = async (id: string): Promise<void> => {
  await invoke('tunnel_close', { id });
}

export const openTunnel = async (port: string, domain?: string, host_rewrite?: string): Promise<void> => {
  await invoke("tunnel_open", {
    port: port,
    domain: domain,
    hostRewrite: host_rewrite
  });
}
