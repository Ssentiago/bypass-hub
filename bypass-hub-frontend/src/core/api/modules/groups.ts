import {Api} from '../client.ts';
import type {Route} from './routes';

export interface Group {
    id: number;
    name: string;
    description?: string;
}

export class GroupsModule {
    constructor(private api: Api) {
    }

    list() {
        return this.api.request<Group[]>('/groups');
    }

    create(data: { name: string; description?: string }) {
        return this.api.request<{ id: number }>('/groups', {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    delete(id: number) {
        return this.api.request<void>(`/groups/${id}`, {
            method: 'DELETE',
        });
    }

    routes(groupId: number) {
        return this.api.request<Route[]>(`/groups/${groupId}/routes`);
    }

    addRoute(groupId: number, routeId: number) {
        return this.api.request<void>(`/groups/${groupId}/routes`, {
            method: 'POST',
            body: JSON.stringify({route_id: routeId}),
        });
    }

    removeRoute(groupId: number, routeId: number) {
        return this.api.request<void>(`/groups/${groupId}/routes/${routeId}`, {
            method: 'DELETE',
        });
    }
}