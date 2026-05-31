// src/core/api/modules/infrastructure/servers.ts
import type {Api} from '../../client.ts';
import type {ProxyModule} from './proxy.ts';

export interface Server {
    id: number;
    name: string;
    address: string;
    xui_api_key: string;
    uuid: string;
}

export interface ServerInbound {
    id: number;
    server_id: number;
    inbound_id: number;
}

export interface XuiInbound {
    id: number;
    remark: string;
    tag: string;
    protocol: string;
    port: number;
}

export interface XuiServerStatus {
    online: boolean;
}

export class ServersModule {
    constructor(
        private api: Api,
        private proxy: ProxyModule,
    ) {
    }

    list() {
        return this.api.request<Server[]>('/infrastructure/servers');
    }

    create(data: { name: string; address: string; xui_api_key: string }) {
        return this.api.request<{ id: number; uuid: string }>('/infrastructure/servers', {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    delete(id: number) {
        return this.api.request<void>(`/infrastructure/servers/${id}`, {
            method: 'DELETE',
        });
    }

    listInbounds(id: number) {
        return this.api.request<ServerInbound[]>(`/infrastructure/servers/${id}/inbounds`);
    }

    addInbound(id: number, inbound_id: number) {
        return this.api.request<{ id: number }>(`/infrastructure/servers/${id}/inbounds`, {
            method: 'POST',
            body: JSON.stringify({inbound_id}),
        });
    }

    removeInbound(id: number, inbound_id: number) {
        return this.api.request<void>(`/infrastructure/servers/${id}/inbounds/${inbound_id}`, {
            method: 'DELETE',
        });
    }

    async listXuiInbounds(id: number) {
        const res = await this.proxy.request<{ success: boolean; obj: XuiInbound[] }>(
            id, '/panel/api/inbounds/options'
        );
        console.log('proxy response:', res);
        return (res.body.obj ?? []).filter(i => i.protocol === 'wireguard');
    }

    async checkStatus(id: number): Promise<boolean> {
        try {
            const res = await this.proxy.request(id, '/panel/api/server/status');
            return res.status >= 200 && res.status < 300;
        } catch {
            return false;
        }
    }
}