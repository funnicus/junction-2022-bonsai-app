

export type Data = {
	type: 'extension' | 'leaf';
	time: number;
	children: Data[];
	taskId: string;
	angle?: number;
	length?: number;
};
