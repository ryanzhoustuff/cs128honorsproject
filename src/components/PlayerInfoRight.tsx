import './PlayerInfoRight.css';

const PlayerInfoRight: React.FC = () => {
  return (
    <div className="player-info-right">
      <h2>[FirstName LastName]</h2>
      <p>Money: [PlaceHolder]</p>
      <p>Current Bet: [PlaceHolder]</p>
    </div>
  );
};

export default PlayerInfoRight;