import {getContext} from "svelte";
import type {MemoryRouter} from "./types";

export function useRouter<T = unknown>(): MemoryRouter<T> {
    return getContext("MEMORY_ROUTER");
}
