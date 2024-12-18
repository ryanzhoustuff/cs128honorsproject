import React from 'react';
import './Game.css';
import Header from './components/Header';
import PokerTable from './components/PokerTable';

import PlayerInfoRefactor from './components/PlayerInfo';

const App: React.FC = () => {
  function getImageURL(name: string) {
    return new URL(`./assets/${name}`, import.meta.url).href;
  }
  const namesInput = ["Home", "Play"];
  const hrefsInput = ["/", "/game"];
  return (
    <div className="app">
      <Header names={namesInput} hrefs={hrefsInput} />
      <div className="content">
        <div className="sidebar" >
          <img src={getImageURL('sample_profile.jpg')} alt="Profile Picture" className="leftImage" />
          <PlayerInfoRefactor userName="Ryan Z" userBalance={2000} currentBet={100} />
          <div className="leftcards">
            <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" className="leftcard1" />
            <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" className="leftcard2" />
          </div>

        </div>
        <PokerTable numImages={4} pot={500} />
        <div className="sidebar" >
          <img src={getImageURL('sample_profile.jpg')} alt="Profile Picture" className="rightImage" />
          <div className="margin-adjuster">
            <PlayerInfoRefactor userName="Dabo Swinney" userBalance={2000} currentBet={100} />
          </div>


          <div className="rightcards">
            <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" className="rightcard1" />
            <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" className="rightcard2" />
          </div>
        </div>
      </div>
      <footer>© 2024 Honors Poker. All rights reserved.</footer>
    </div>
  );
};

export default App;