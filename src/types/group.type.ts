export interface Group {
	limit: number;
	offset: number;
	setId: string;
	result: Result[];
}

export interface Result {
	displayName: string;
	id: string;
	locationType: string;
	name: string;
	onPremisesSid: string;
	typeField: string;
	managedBy: string;
}
