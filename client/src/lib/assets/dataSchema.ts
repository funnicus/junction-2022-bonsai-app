export type Data = {
    type: string,
    time: number,
    children: Data[],
    taskId: string,
    angle?: number,
    length?: number
} 