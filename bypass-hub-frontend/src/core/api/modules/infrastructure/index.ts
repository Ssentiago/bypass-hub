// src/core/api/modules/infrastructure/index.ts
import type {Api} from '../../client.ts';
import {ServersModule} from './servers.ts';
import {ProxyModule} from './proxy.ts';

export class InfrastructureApi {
    public servers: ServersModule;
    public proxy: ProxyModule;

    constructor(api: Api) {
        this.proxy = new ProxyModule(api);
        this.servers = new ServersModule(api, this.proxy);
    }
}