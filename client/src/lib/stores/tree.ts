import type { Data } from '$lib/dataSchema';
import { writable, type Readable } from 'svelte/store';
import treeData from '$lib/assets/tree.json';
import { prevent_default } from 'svelte/internal';

type TreeState = { selectedNode: Data | null; nodes: Data[]; showLeaves: boolean };

export type TreeStore = Readable<TreeState> & {
	addLeaf: () => void;
	addExtension: (angle: number, length: number) => void;
	setSelectedNode: (node: Data | null) => void;
	toggleLeaves: (show: boolean) => void;
};

export const createTreeStore = (): TreeStore => {
	const state = writable<TreeState>({
		selectedNode: null,
		showLeaves: true,
		nodes: []
	});

	// load tree after 1 ms, giving it a starting animation
	setTimeout(() => {
		state.update((prev) => ({ ...prev, nodes: treeData as any }));
	}, 1);

	const addLeaf = () => {
		const item: Data = {
			type: 'leaf',
			children: [],
			taskId: '123981628362',
			time: Date.now()
		};

		state.update((prev) => {
			if (!prev.selectedNode) return prev;
			prev.selectedNode?.children.push(item);
			return prev;
		});
	};

	const addExtension = (angle: number, length: number) => {
		const item: Data = {
			type: 'extension',
			length,
			angle,
			children: [],
			taskId: '123981628362',
			time: Date.now()
		};

		state.update((prev) => {
			if (!prev.selectedNode) return prev;
			prev.selectedNode?.children.push(item);
			prev.selectedNode = item; // select newly created item
			return prev;
		});
	};

	const setSelectedNode = (node: Data | null) => {
		state.update((prev) => {
			prev.selectedNode = node;
			return prev;
		});
	};

	const toggleLeaves = (show: boolean) => {
		state.update((prev) => {
			prev.showLeaves = show;
			return prev;
		});
	};

	return {
		subscribe: state.subscribe,
		addLeaf,
		addExtension,
		setSelectedNode,
		toggleLeaves
	};
};
