import { writable } from 'svelte/store';

export const createMenuStore = () => {
	const menuState = writable({
		state: -1,
		isLeaf: 0
	});

	return menuState;
};
