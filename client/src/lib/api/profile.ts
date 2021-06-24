import { requests } from './agent';

export interface Profile {
	id: string;
}
export const Profiles = {
	details: (): Promise<Profile> => requests.get(`/profiles/me`)
};
