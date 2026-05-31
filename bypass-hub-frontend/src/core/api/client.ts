// src/core/api/client.ts
import {AuthModule} from "./modules/auth.ts";
import {ScopedApi} from "./modules/scoped.ts";
import {API_BASE} from "@/core/config.ts";

export class ApiError extends Error {
    constructor(
        public status: number,
        public data: any,
        message?: string,
    ) {
        const detailedMessage = message || ApiError.extractMessage(data) || `HTTP ${status}`;
        super(detailedMessage);
        this.name = 'ApiError';
    }

    private static extractMessage(data: any): string | null {
        if (!data) return null;
        if (typeof data === 'string') return data;
        if (typeof data === 'object') return data.message || data.error || JSON.stringify(data);
        return null;
    }
}

export class Api {
    public auth: AuthModule;
    public xui: ScopedApi;
    public mikrotik: ScopedApi;

    private readonly API_PREFIX = '/api';

    constructor() {
        this.auth = new AuthModule(this);
        this.xui = new ScopedApi(this, 'xui');
        this.mikrotik = new ScopedApi(this, 'mikrotik');
    }

    async request<T>(path: string, options?: RequestInit): Promise<T> {
        const url = `${API_BASE}${this.API_PREFIX}${path}`;

        const headers = new Headers(options?.headers);
        if (!(options?.body instanceof FormData) && !headers.has('Content-Type')) {
            headers.set('Content-Type', 'application/json');
        }

        const response = await fetch(url, {
            ...options,
            headers,
            credentials: 'include',
        });

        if (!response.ok) {
            const text = await response.text();
            throw new ApiError(response.status, {message: text});
        }

        const text = await response.text();
        if (!text) return undefined as T;

        try {
            return JSON.parse(text) as T;
        } catch {
            return text as T;
        }
    }
}

export const api = new Api();