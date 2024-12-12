Contributors: Ryan Zhou(ryanz5), Eric Wang(ericw9), Vishnu Aravind (vaa4), Amit Gulati (amitabh6)
Poker Group

1. Introduction
- This project aims to develop a comprehensive full-stack poker bot. The bot will provide a practice environment for poker enthusiasts, allowing them to play against an opponent with advanced strategy optimization capabilities. The backend will be developed in Rust for performance and scalability, and the frontend will use React to offer a user-friendly interface. 

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

Project Checkpoint 1
- Start on the poker bot logic
- Start on the website

Project Checkpoint 2
- Finish the poker bot logic
- Finish the website
- Start on connecting the frontend to the backend

3. Project Challenges
- Learning a new language (React)
- Connecting the frontend to the backend
