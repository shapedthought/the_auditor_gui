export interface AuthItem {
	type: string;
	id: string;
	user: User | null;
	group: Group | null;
}

export interface User {
	displayName: string;
	id: string;
	locationType: string;
	name: string;
}

export interface Group {
	displayName: string;
	id: string;
	locationType: string;
	name: string;
	onPremisesSid: string;
}
