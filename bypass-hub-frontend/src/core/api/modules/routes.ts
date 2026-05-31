// src/core/api/modules/routes.ts
import type {Api} from '../client.ts';

export interface Route {
    id: number;
    value: string;
    type: 'domain' | 'ip';
}

export interface GroupedRoutes {
    group: { id: number; name: string } | null;
    routes: Route[];
}

export class RoutesModule {
    constructor(
        private api: Api,
        private scope: 'xui' | 'mikrotik',
    ) {
    }

    private p(path: string) {
        return `/${this.scope}/routes${path}`;
    }

    list() {
        return this.api.request<Route[]>(this.p(''));
    }

    listGrouped() {
        return this.api.request<GroupedRoutes[]>(this.p('/grouped'));
    }

    create(data: { value: string; type: 'domain' | 'ip'; group_ids?: number[] }) {
        return this.api.request<{ id: number }>(this.p(''), {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    bulkCreate(data: { routes: { value: string; type: 'domain' | 'ip' }[]; group_ids?: number[] }) {
        return this.api.request<{ inserted: number }>(this.p('/bulk'), {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    delete(id: number) {
        return this.api.request<void>(this.p(`/${id}`), {
            method: 'DELETE',
        });
    }
}