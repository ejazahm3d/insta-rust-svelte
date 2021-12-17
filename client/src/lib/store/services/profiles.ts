import type { Follower, Post, Profile } from '$lib/types';
import { baseApi } from './baseApi';

export const profileApi = baseApi.injectEndpoints({
	endpoints: (builder) => ({
		profileDetails: builder.query<Profile, string>({
			query: (userId) => ({
				url: `/profiles/${userId}`,
				method: 'GET'
			}),
			providesTags: (_res, _err, arg) => [{ type: 'ProfileDetails', id: arg }]
		}),
		postsByProfile: builder.query<Post[], string>({
			query: (userId) => ({
				url: `/profiles/${userId}/posts`,
				method: 'GET'
			}),
			providesTags: (_res, _err, arg) => [{ type: 'PostsByProfile', id: arg }]
		}),

		followersByLeaderId: builder.query<Follower[], string>({
			query: (leaderId) => ({
				url: `/followers/${leaderId}`,
				method: 'GET'
			}),
			providesTags: (_res, _err, arg) => [{ type: 'Followers', id: arg }]
		}),

		followersCount: builder.query<number, string>({
			query: (leaderId) => ({
				url: `/followers/${leaderId}/count`,
				method: 'GET'
			}),
			providesTags: (_res, _err, arg) => [{ type: 'Followers', id: arg }]
		}),

		leadersByFollowerId: builder.query<Follower[], string>({
			query: (followerId) => ({
				url: `/leaders/${followerId}`,
				method: 'GET'
			}),
			providesTags: (_res, _err, arg) => [{ type: 'Leaders', id: arg }]
		}),
		leadersCount: builder.query<number, string>({
			query: (followerId) => ({
				url: `/leaders/${followerId}/count`,
				method: 'GET'
			}),
			providesTags: (_res, _err, arg) => [{ type: 'Leaders', id: arg }]
		}),

		isFollowing: builder.query<boolean, string>({
			query: (followerId) => ({
				url: `/leaders/${followerId}/isFollowing`,
				method: 'GET'
			}),
			providesTags: (_res, _err, arg) => [{ type: 'Leaders', id: arg }]
		}),

		followOrUnfollow: builder.mutation<Follower, string>({
			query: (leaderId) => ({
				url: `/leaders/${leaderId}`,
				method: 'POST'
			}),
			invalidatesTags: (_res, _err, arg) => [
				{ type: 'Followers', id: arg },
				{ type: 'Leaders', id: arg }
			]
		})
	}),
	overrideExisting: false
});

export const {
	profileDetails,
	postsByProfile,
	followOrUnfollow,
	followersByLeaderId,
	followersCount,
	isFollowing,
	leadersByFollowerId,
	leadersCount
} = profileApi.endpoints;
