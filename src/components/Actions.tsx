import './Actions.css';
import { useState } from 'react';

type Prop = {
  call_check: string;
}

const Actions = ({ call_check }: Prop) => {
  const [numButtons, setNumButtons] = useState(3);
  const [bet, setBet] = useState(100);
  const [betClicked, setBetClicked] = useState(false);
  const betClick = () => {
    setNumButtons(2);
    setBetClicked(true);
  }
  const submitClick = () => {
    setBetClicked(false);
    setNumButtons(3);
  }
  const change = (event: { target: { value: any; }; }) => {
    const newvalue = event.target.value;
    setBet(newvalue);
  }

  return (
    <div className="actions">
      {betClicked == false ? <div className="buttonWrapper" style={{ width: `calc(100% / ${numButtons})`, minWidth: "135px" }}><button>{call_check}</button></div> : null}
      {betClicked == false ? <div className="buttonWrapper" style={{ width: `calc(100% / ${numButtons})`, minWidth: "135px" }}><button onClick={betClick}>Bet</button></div> : null}
      {betClicked == true ? <div><input onChange={change} value={bet} style={{ width: `calc(100% / ${numButtons})`, minWidth: "135px" }}></input></div> : null}
      {betClicked == false ? <div className="buttonWrapper" style={{ width: `calc(100% / ${numButtons})`, minWidth: "135px" }}><button>Fold</button></div> : null}
      {betClicked == true ? <div className="buttonWrapper" style={{ width: `calc(100% / ${numButtons})`, minWidth: "135px" }}><button onClick={submitClick}>Submit</button></div> : null}
    </div>
  );
};

export default Actions;