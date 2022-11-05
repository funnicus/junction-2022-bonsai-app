import { writable } from "svelte/store";

export const createMenuStore = () => {
    const menuState = writable({
        state: -1,
    })

    return menuState
};