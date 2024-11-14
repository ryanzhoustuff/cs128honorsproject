import './Actions.css';

type Prop ={
  call_check: string;
}

const Actions = ({call_check}:Prop) => {
  return (
    <div className="actions">
      <div className="buttonWrapper">
        <button>{call_check}</button>
      </div>
      <div className="buttonWrapper">
        <button>Bet</button>
      </div>
      <div className="buttonWrapper">
        <button>Fold</button>
      </div>
    </div>
  );
};

export default Actions;