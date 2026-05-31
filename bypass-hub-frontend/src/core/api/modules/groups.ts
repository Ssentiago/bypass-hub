// src/core/api/modules/groups.ts
import type {Api} from '../client.ts';
import type {Route} from './routes.ts';

export interface Group {
    id: number;
    name: string;
    description?: string;
}

export class GroupsModule {
    constructor(
        private api: Api,
        private scope: 'xui' | 'mikrotik',
    ) {
    }

    private p(path: string) {
        return `/${this.scope}/groups${path}`;
    }

    list() {
        return this.api.request<Group[]>(this.p(''));
    }

    create(data: { name: string; description?: string }) {
        return this.api.request<{ id: number }>(this.p(''), {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    delete(id: number) {
        return this.api.request<void>(this.p(`/${id}`), {
            method: 'DELETE',
        });
    }

    routes(groupId: number) {
        return this.api.request<Route[]>(this.p(`/${groupId}/routes`));
    }

    addRoute(groupId: number, routeId: number) {
        return this.api.request<void>(this.p(`/${groupId}/routes`), {
            method: 'POST',
            body: JSON.stringify({route_id: routeId}),
        });
    }

    removeRoute(groupId: number, routeId: number) {
        return this.api.request<void>(this.p(`/${groupId}/routes/${routeId}`), {
            method: 'DELETE',
        });
    }
}