import {Api} from '../client.ts';

export class AuthModule {
    constructor(private api: Api) {
    }

    login(credentials: { username: string; password: string }) {
        return this.api.request<void>('/auth/login', {
            method: 'POST',
            body: JSON.stringify(credentials),
        });
    }

    logout() {
        return this.api.request<void>('/auth/logout', {
            method: 'POST',
        });
    }

    me() {
        return this.api.request<void>('/auth/me');
    }
}