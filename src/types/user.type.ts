export interface User {
	offset: number;
	limit: number;
	setId: string;
	result: Result[];
}

export interface Result {
	id: string;
	displayName: string;
	name: string;
	typeField: string;
	locationType: string;
}
