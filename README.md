# KuriLogin

## Overview

This authentication service is a Rust application using Actix-web. It provides OAuth-based
authentication, user information retrieval,
and JWT token management, ideal for integration into larger projects needing user
authentication capabilities.

## Project Structure
- modules/auth: Contains OAuth handling, JWT management, and the application service layer for authentication.
- modules/user: Manages user services including retrieval of user information from the repository.
- utils: Utility modules such as configuration handling and database interactions.
- error: Custom error types structured for response handling across the application.

## Prerequisites
-   Rust programming language and Cargo package manager.
-   Access to a PostgreSQL database.
-   Valid Google OAuth credentials.
-   Environment setup for variables like `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `DOMAIN`, `DATABASE_URL`, `JWT_SECRET`.

## Installation

1. Clone the repository:
  ```bash
git clone https://github.com/Abraxas-365/KuriLogin auth_service
cd auth_service
  ```

2. Environment Variables Setup:
  ```bash
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret
DOMAIN=your_domain 
DATABASE_URL=postgres://username:password@host:port/database
JWT_SECRET=your_jwt_secret
  ```

3. Install Dependencies:

Ensure your Cargo.toml has all required dependencies and run:
  ```bash
cargo build
  ```

4.	Running the Application:
  ```bash
cargo run
  ```

## Usage
- User Authentication: The service supports Google OAuth2 for user authentication.
- Endpoints:
- /auth/{provider_name}/login: Initiates the login process for specified OAuth providers (e.g., google). `/auth/google/login`
- /auth/{provider_name}/callback: Handles callbacks from OAuth providers and provides JWT upon successful authentication. `/auth/google/callback`
- /me/{token}: Retrieves user information using a valid JWT.
