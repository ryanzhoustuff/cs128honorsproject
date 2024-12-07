import React from 'react';
import './Game.css';
import Header from './components/Header';
import PlayerInfo from './components/PlayerInfo';
import Actions from './components/Actions';
import PokerTable from './components/PokerTable';

import PlayerInfoLeft from './components/PlayerInfoLeft';
import PlayerInfoRight from './components/PlayerInfoRight';

const App: React.FC = () => {
  const namesInput = ["Home", "Play", "third page"];
  const hrefsInput = ["/", "/game", "f"];
  return (
    <div className="app">
      <Header names={namesInput} hrefs={hrefsInput} />
      <div className="content">
        <div className="sidebar" >
          <img src='./src/assets/sample_profile.jpg' alt="Profile Picture" className="leftImage"/>
          <PlayerInfoLeft />
          <div className="leftcards">
            <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png' className="leftcard1"/>
            <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png' className="leftcard2"/>
          </div>
          
        </div>
        <PokerTable numImages={4} pot={500}/> 
        <div className="sidebar">
        <img src='./src/assets/sample_profile.jpg' alt="Profile Picture" className="rightImage"/>
          <PlayerInfoRight />
          <div className="rightcards">
            <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png' className="rightcard1"/>
            <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png' className="rightcard2"/>
          </div>
        </div>
      </div>
      <footer>© 2024 Honors Poker. All rights reserved.</footer>
    </div>
  );
};

export default App;