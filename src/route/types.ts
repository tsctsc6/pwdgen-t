export type RouteState = {
    path: string;
    props?: Record<string, unknown>;
};

export interface IMemoryRouter {
    stack: import("svelte/store").Writable<RouteState[]>;
    current: import("svelte/store").Writable<RouteState>;
    push: (path: string, props?: Record<string, unknown>) => void;
    replace: (path: string, props?: Record<string, unknown>) => void;
    back: () => void;
    clear: (path: string) => void;
}

export const SingletonKey = "MEMORY_ROUTER";
