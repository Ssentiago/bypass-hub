// src/core/api/modules/scoped.ts
import type {Api} from '../client.ts';
import {RoutesModule} from './routes.ts';
import {GroupsModule} from './groups.ts';

export class ScopedApi {
    public routes: RoutesModule;
    public groups: GroupsModule;

    constructor(api: Api, scope: 'xui' | 'mikrotik') {
        this.routes = new RoutesModule(api, scope);
        this.groups = new GroupsModule(api, scope);
    }
}