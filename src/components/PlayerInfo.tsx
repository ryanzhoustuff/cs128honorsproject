import './PlayerInfo.css';

const PlayerInfo: React.FC = () => {
  return (
    <div className="player-info">
      <h2>[FirstName LastName]</h2>
      <p>Money: [PlaceHolder]</p>
      <p>Current Bet: [PlaceHolder]</p>
    </div>
  );
};

export default PlayerInfo;