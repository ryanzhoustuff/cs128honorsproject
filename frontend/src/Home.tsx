import React from 'react';
import './Game.css';
import Header from './components/Header';
import HomeBody from './components/HomeBody';

const Home: React.FC = () => {
  const namesInput = ["Home", "Play"];
  const hrefsInput = ["/", "/game"];
  return (
    <div className="app">
      <Header names={namesInput} hrefs={hrefsInput} />
      <HomeBody />
      <footer>© 2024 Honors Poker. All rights reserved.</footer>
    </div>
  );
};

export default Home;