import type { Data } from '$lib/dataSchema';
import type { Writable } from 'svelte/store';
import { writable } from 'svelte-local-storage-store';

type TreeState = {
	selectedNode: Data | null;
	nodes: Data[];
	showLeaves: boolean;
	previewAngle: number;
	previewLength: number;
};

export type TreeStore = Writable<TreeState> & {
	addLeaf: () => void;
	addExtension: (angle: number, length: number) => void;
	setSelectedNode: (node: Data | null) => void;
	toggleLeaves: (show: boolean) => void;
	removeNode: (node: Data) => void;
	setNodes: (nodes: Data[]) => void;
};

export const createTreeStore = (): TreeStore => {
	const state = writable<TreeState>('tree', {
		previewAngle: 0,
		previewLength: 20,
		selectedNode: null,
		showLeaves: true,
		nodes: [
			{
				type: 'extension',
				time: Date.now(),
				taskId: '',
				length: 30,
				angle: 4,
				children: []
			}
		]
	});

	// // load tree after 1 ms, giving it a starting animation
	// setTimeout(() => {
	// 	state.update((prev) => ({ ...prev, nodes: treeData as any }));
	// }, 1);

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

	const setPreviewData = (angle: number, length: number) => {
		state.update((prev) => ({ ...prev, previewAngle: angle, previewLength: length }));
	};

	const recursiveDelete = (nodes: Data[], nodeToDelete: Data): boolean => {
		for (const node of nodes) {
			if (node === nodeToDelete) {
				nodes.splice(nodes.indexOf(node), 1);
				return true;
			}

			const found = recursiveDelete(node.children, nodeToDelete);
			if (found) return found;
		}

		return false;
	};

	const removeNode = (nodeToDelete: Data) => {
		state.update((prev) => {
			recursiveDelete(prev.nodes, nodeToDelete);
			return prev;
		});
	};

	const setNodes = (nodes: Data[]) => {
		state.update((prev) => {
			prev.nodes[0].children = nodes;
			return prev;
		});
	};

	return {
		subscribe: state.subscribe,
		set: state.set,
		update: state.update,
		addLeaf,
		addExtension,
		setSelectedNode,
		toggleLeaves,
		removeNode,
		setNodes
	};
};
