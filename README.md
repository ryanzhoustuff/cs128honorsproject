1. Introduction
This project aims to develop a comprehensive full-stack poker bot and solver with strategic optimization using Counterfactual Regret Minimization (CFR). The bot will provide a practice environment for poker enthusiasts, allowing them to play against an AI opponent with advanced strategy optimization capabilities. The backend will be developed in Rust for performance and scalability, and the frontend will use React to offer a user-friendly interface. The goal is to leverage CFR to continually optimize the bot’s decision-making strategies across simulated games.

2. Objectives
- Create a robust poker game engine with support for Texas Hold'Em, including mechanics such as betting rounds, hand evaluations, and game flows.
- Optimize gameplay strategies using Counterfactual Regret Minimization to improve the bot’s decision-making and simulate competitive poker scenarios.
- Implement a seamless full-stack architecture using Rust for backend logic and React for the frontend interface.
- Integrate MongoDB to store player profiles, game histories, hand evaluations, and bot performance metrics for analysis and future optimizations.

3. Project Architecture and Technologies

Frontend: React
- User-friendly interface for creating accounts, starting games, and visualizing game statistics.
- Responsive design to ensure a smooth experience on desktops and mobile devices.

Backend: Rust with Axum Framework
- Use Axum to build HTTP APIs for handling game mechanics, player actions, and communication between frontend and backend.
- Ensure high performance and low latency for real-time game interactions.

Data Serialization: Serde
- Efficient serialization and deserialization of JSON objects between the frontend and backend.
- Smooth data exchange, minimizing overhead during gameplay.

Database: MongoDB
- Store player profiles, game histories, and hand evaluation data.
- Enable detailed analysis of game sessions and user progress over time.

Algorithm: Counterfactual Regret Minimization (CFR)
- Implement CFR in Rust to model optimal poker strategies by minimizing regret over repeated game simulations.
- Optimize decision-making processes for the bot to continuously improve its strategy in dynamic environments.

How to use:
npm run dev
