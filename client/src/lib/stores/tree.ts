import type { Data } from "$lib/dataSchema"
import { writable, type Readable } from "svelte/store"

type TreeState = {selectedNode: Data | null, nodes: Data[]}

export type TreeStore = Readable<TreeState> & {
  addLeaf: () => void
  addExtension: (angle: number, length: number) => void
  setSelectedNode: (node: Data | null) => void
}



export const createTreeStore = (): TreeStore => {
  const state = writable<TreeState>({
    selectedNode: null,
    nodes:[
    {
      type:'extension',
      length: 20,
      angle: 5,
      children: [],
      taskId: "",
      time: Date.now()
    }
  ]})

  const addLeaf = () => {
    const item: Data =  {
      type:'leaf',
      children: [],
      taskId: "123981628362",
      time: Date.now()
    }

    state.update(prev => {
      if (!prev.selectedNode) return prev
      prev.selectedNode?.children.push(item)
      prev.selectedNode = null // clear selection
      return prev
    })
  }

  const addExtension = (angle: number, length: number) => {
    const item: Data =  {
      type:'extension',
      length,
      angle,
      children: [],
      taskId: "123981628362",
      time: Date.now()
    }

    state.update(prev => {
      if (!prev.selectedNode) return prev
      prev.selectedNode?.children.push(item)
      prev.selectedNode = item // select newly created item
      return prev
    })
  }

  const setSelectedNode = (node: Data | null) => {
    state.update(prev => {
      prev.selectedNode = node;
      return prev
    })
  }

  return {
    subscribe: state.subscribe,
    addLeaf,
    addExtension,
    setSelectedNode,
  }
}