export type Workspace = {
	id: number;
	name: string;
};
export type InitWorkspace = Omit<Workspace, "id">;
