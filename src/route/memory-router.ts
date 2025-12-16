import {writable, get, type Writable} from "svelte/store";
import type {MemoryRouter, RouteState} from "./types";

export function createMemoryRouter(): MemoryRouter {
    const initPath = "/init";

    const stack: Writable<RouteState[]> = writable([
        {path: initPath, props: undefined}
    ]);

    const current: Writable<RouteState> = writable({
        path: initPath,
        props: undefined
    });

    function sync() {
        const s = get(stack);
        current.set(s[s.length - 1]);
    }

    function push(path: string, props?: Record<string, unknown>) {
        stack.update(s => [...s, {path, props}]);
        sync();
    }

    function replace(path: string, props?: Record<string, unknown>) {
        stack.update(s => {
            const copy = [...s];
            copy[copy.length - 1] = {path, props};
            return copy;
        });
        sync();
    }

    function back() {
        stack.update(s => {
            if (s.length > 1) {
                return s.slice(0, -1);
            }
            return s;
        });
        sync();
    }

    function clear(path = "/") {
        stack.set([{path, props: undefined}]);
        sync();
    }

    return {
        stack,
        current,
        push,
        replace,
        back,
        clear
    };
}
// Note: callers should create their own router instance via `createMemoryRouter()`
