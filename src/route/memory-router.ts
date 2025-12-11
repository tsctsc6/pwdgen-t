import {writable, get, type Writable} from "svelte/store";
import type {MemoryRouter, RouteState} from "./types";

function createMemoryRouter<T = unknown>(): MemoryRouter<T> {
    const initPath = "/init";

    const stack: Writable<RouteState<T>[]> = writable([
        {path: initPath, state: undefined}
    ]);

    const current: Writable<RouteState<T>> = writable({
        path: initPath,
        state: undefined
    });

    function sync() {
        const s = get(stack);
        current.set(s[s.length - 1]);
    }

    function push(path: string, state?: T) {
        stack.update(s => [...s, {path, state}]);
        sync();
    }

    function replace(path: string, state?: T) {
        stack.update(s => {
            const copy = [...s];
            copy[copy.length - 1] = {path, state};
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
        stack.set([{path, state: undefined}]);
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

export const router = createMemoryRouter();
