import React from 'react';
import './PlayerInfo.css';

type Prop = {
  userName: string;
  userBalance: number;
  currentBet: number;
}

const PlayerInfo: React.FC<Prop> = ({userName, userBalance, currentBet}) => {
  return (
    <div className="player-info">
      <h2>{userName}</h2>
      <p>Money: ${userBalance}</p>
      <p>Current Bet: ${currentBet}</p>
    </div>
  );
};

export default PlayerInfo;