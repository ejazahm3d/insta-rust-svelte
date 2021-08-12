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

	followersCount: (leaderId: string): Promise<number> =>
		requests.get(`/followers/${leaderId}/count`)
};

export const Leaders = {
	leadersByFollowerId: (followerId: string): Promise<Follower[]> =>
		requests.get(`/leaders/${followerId}`),
	leadersCount: (followerId: string): Promise<number> =>
		requests.get(`/leaders/${followerId}/count`),

	isFollowing: (followerId: string): Promise<boolean> =>
		requests.get(`/leaders/${followerId}/isFollowing`),
	followOrUnfollow: (leaderId: string): Promise<Follower> =>
		requests.post(`/leaders/${leaderId}`, {})
};
