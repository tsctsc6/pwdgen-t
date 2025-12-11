export type RouteState<T = unknown> = {
    path: string;
    state?: T;
};

export interface MemoryRouter<T = unknown> {
    stack: import("svelte/store").Writable<RouteState<T>[]>;
    current: import("svelte/store").Writable<RouteState<T>>;
    push: (path: string, state?: T) => void;
    replace: (path: string, state?: T) => void;
    back: () => void;
    clear: (path: string) => void;
}
