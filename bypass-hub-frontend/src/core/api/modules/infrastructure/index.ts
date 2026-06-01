import {MikrotiksModule} from './mikrotiks.ts';
import {ServersModule} from "@/core/api/modules/infrastructure/servers.ts";
import {ProxyModule} from "./proxy.ts";
import type {Api} from "@/core/api/client.ts";

export class InfrastructureApi {
    public servers: ServersModule;
    public proxy: ProxyModule;
    public mikrotiks: MikrotiksModule;

    constructor(api: Api) {
        this.proxy = new ProxyModule(api);
        this.servers = new ServersModule(api, this.proxy);
        this.mikrotiks = new MikrotiksModule(api);
    }
}