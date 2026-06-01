// src/core/api/modules/infrastructure/mikrotiks.ts
import type {Api} from '../../client.ts';
import {API_BASE} from '@/core/config.ts';

export interface Mikrotik {
    id: number;
    name: string;
    server_id: number;
    inbound_id: number;
    public_key: string | null;
    assigned_ip: string | null;
    uuid: string;
    status: 'pending_key' | 'active';
    created_at: string;
}

export interface CreateMikrotikRequest {
    name: string;
    server_id: number;
    inbound_id: number;
}

export interface CreateMikrotikResponse {
    id: number;
    uuid: string;
}

export interface SetKeyResponse {
    assigned_ip: string;
}

export class MikrotiksModule {
    constructor(private api: Api) {
    }

    list(): Promise<Mikrotik[]> {
        return this.api.request('/infrastructure/mikrotiks');
    }

    create(body: CreateMikrotikRequest): Promise<CreateMikrotikResponse> {
        return this.api.request('/infrastructure/mikrotiks', {
            method: 'POST',
            body: JSON.stringify(body),
        });
    }

    delete(id: number): Promise<void> {
        return this.api.request(`/infrastructure/mikrotiks/${id}`, {
            method: 'DELETE',
        });
    }

    setKey(id: number, public_key: string): Promise<SetKeyResponse> {
        return this.api.request(`/infrastructure/mikrotiks/${id}/key`, {
            method: 'PATCH',
            body: JSON.stringify({public_key}),
        });
    }

    scriptUrl(id: number): string {
        return `${API_BASE}/api/infrastructure/mikrotiks/${id}/script`;
    }
}