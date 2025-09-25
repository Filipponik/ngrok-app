import { invoke } from "@tauri-apps/api/core";

export interface Tunnel {
  id: string;
  url: string;
  local_port: number;
  proxy_port: number;
  is_static_domain: boolean;
  request_headers: { name: string; value: string }[];
  response_headers: { name: string; value: string }[];
  basic_auth?: { username: string; password: string };
}

export type TunnelOpen = {
  port: string;
  domain?: string;
  host_rewrite?: string;
  request_headers?: { name: string; value: string }[];
  response_headers?: { name: string; value: string }[];
  basic_auth?: { username: string; password: string };
};

export const getTunnels = async (): Promise<Tunnel[]> => {
  return await invoke("tunnel_list");
};

export const closeTunnel = async (id: string): Promise<void> => {
  await invoke("tunnel_close", { id });
};

export const openTunnel = async (tunnel: TunnelOpen): Promise<void> => {
  await invoke("tunnel_open", {
    command: {
      port: tunnel.port,
      domain: tunnel.domain,
      host_rewrite: tunnel.host_rewrite,
      request_headers: tunnel.request_headers ? tunnel.request_headers : [],
      response_headers: tunnel.response_headers ? tunnel.response_headers : [],
      basic_auth: tunnel.basic_auth,
    },
  });
};
