// src/core/api/modules/infrastructure/proxy.ts
import type {Api} from '../../client.ts';

export interface ProxyResponse<T = unknown> {
    status: number;
    body: T;
}

export class ProxyModule {
    constructor(private api: Api) {
    }

    request<T = unknown>(serverId: number, path: string, method = 'GET', body?: unknown) {
        return this.api.request<ProxyResponse<T>>(
            `/infrastructure/servers/${serverId}/proxy`,
            {
                method: 'POST',
                body: JSON.stringify({path, method, body}),
            }
        );
    }
}