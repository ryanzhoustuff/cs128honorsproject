import React from 'react';
import './Game.css';
import Header from './components/Header';
import PokerTable from './components/PokerTable';

import PlayerInfoRefactor from './components/PlayerInfoRefactor';

const App: React.FC = () => {
  const namesInput = ["Home", "Play", "third page"];
  const hrefsInput = ["/", "/game", "f"];
  return (
    <div className="app">
      <Header names={namesInput} hrefs={hrefsInput} />
      <div className="content">
        <div className="sidebar" >
          <img src='./src/assets/sample_profile.jpg' alt="Profile Picture" className="leftImage"/>
            <PlayerInfoRefactor />
          <div className="leftcards">
            <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png' className="leftcard1"/>
            <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png' className="leftcard2"/>
          </div>
          
        </div>
        <PokerTable numImages={4} pot={500}/> 
        <div className="sidebar" >
        <img src='./src/assets/sample_profile.jpg' alt="Profile Picture" className="rightImage"/>
        <div className="margin-adjuster">
          <PlayerInfoRefactor/>
        </div>

          
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