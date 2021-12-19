# Instagram Clone written with Rust and Svelte

Instagram clone written in rust and svelte. It implments the following instagram features.

1. Authentication and authorization.
2. Uploading a photo with description.
3. Liking and disliking photos.
4. Creating and deleting comments.
5. Liking and disliking comments.
6. User profiles.
7. Following People.

## Technologies Used

It uses following technologies.

### Backend

1. Actix web for async API
2. PostgreSQL as Database
3. sqlx for querying
4. Prisma for migrations

### Frontend

1. SvelteKit
2. Tailwind for UI
3. Redux Toolkit Query for querying, caching and revalidating.

### Devops

1. Docker for spinning up postgres database.

## Instructions

### Prerequisites

1. At least version 16 of `node.js` with `npm`.
2. Rust toolchain with `cargo`.
3. `docker` and `docker-compose`.

### Steps

1. Clone the repo.
2. cd into `client` and run `npm i` to install all dependencies.
3. cd into `server` and run `cp .env.example .env`
4. Run `docker-compose up -d` in `server` folder.
5. Run `npx prisma migrate deploy` in `server` directory.
6. Run `npm run dev` in `client` directory and then run `cargo run` in `server` directory.
7. Frontend should be running on `localhost:3000` and backend should be on `localhost:5000`

## Feedback

Feel free to leave any feedback.
