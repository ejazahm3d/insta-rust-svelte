import { requests } from './agent';

export interface Profile {
	id: string;
}
export const Profiles = {
	details: (userId: string): Promise<Profile> => requests.get(`/profiles/${userId}`)
};
