import './PlayerInfoLeft.css';

const PlayerInfoLeft: React.FC = () => {
  return (
    <div className="player-info-left">
      <h2>[FirstName LastName]</h2>
      <p>Money: [PlaceHolder]</p>
      <p>Current Bet: [PlaceHolder]</p>
    </div>
  );
};

export default PlayerInfoLeft;