import {Api} from '../client.ts';

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
    constructor(private api: Api) {
    }

    list() {
        return this.api.request<Route[]>('/routes');
    }

    listGrouped() {
        return this.api.request<GroupedRoutes[]>('/routes/grouped');
    }

    create(data: { value: string; type: 'domain' | 'ip'; group_ids?: number[] }) {
        return this.api.request<{ id: number }>('/routes', {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    bulkCreate(data: { routes: { value: string; type: 'domain' | 'ip' }[]; group_ids?: number[] }) {
        return this.api.request<{ inserted: number }>('/routes/bulk', {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    delete(id: number) {
        return this.api.request<void>(`/routes/${id}`, {
            method: 'DELETE',
        });
    }
}