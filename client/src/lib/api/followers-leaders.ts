import { requests } from './agent';

export interface Follower {
	id: string;
	createdAt: Date;
	leaderId: string;
	followerId: string;
}

export const Followers = {
	followersByLeaderId: (leaderId: string): Promise<Follower[]> =>
		requests.get(`/followers/${leaderId}`),
	followOrUnfollow: (leaderId: string): Promise<Follower> =>
		requests.post('/followers', { leaderId }),
	followersCount: (leaderId: string): Promise<Follower[]> =>
		requests.get(`/followers/${leaderId}/count`)
};

export const Leaders = {
	leadersByFollowerId: (followerId: string): Promise<Follower[]> =>
		requests.get(`/leaders/${followerId}`),
	leadersCount: (followerId: string): Promise<Follower[]> =>
		requests.get(`/leaders/${followerId}/count`)
};
