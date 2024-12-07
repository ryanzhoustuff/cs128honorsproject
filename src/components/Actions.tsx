import './Actions.css';
import { useState } from 'react';

type Prop ={
  call_check: string;
}


/*<button>{betButtonText}</button>*/
const Actions = ({call_check}:Prop) => {
  const [bet, setBet] = useState(100);
  const [betClicked, setBetClicked] = useState(false);
  const betClick = () => {
    /*you might wanna check that its their turn before you allow this button to be clicked*/
    /*alert("This is a popup.");*/
    setBetClicked(true);
  }
  const submitClick = () => {
    setBetClicked(false);
    /*use api here*/
  }
  const change = event => {
    const newvalue = event.target.value;
    setBet(newvalue);
  }

  return (
    <div className="actions">
      {betClicked==false ? <div className="buttonWrapper"><button>{call_check}</button></div>: null}
      {betClicked==false ?<div className="buttonWrapper"><button onClick = {betClick}>Bet</button></div>: null}
      {betClicked==true ?<div><input onChange = {change} value={bet}></input></div>: null}
      {betClicked==false ?<div className="buttonWrapper"><button>Fold</button></div>: null}
      {betClicked==true ?<div className="buttonWrapper"><button onClick = {submitClick}>Submit</button></div>: null}
    </div>
  );
};

export default Actions;