import './PokerTable.css';
import React, { useState } from 'react';
import Actions from './Actions';

type Prop = {
  numImages: number;
  pot: number;
};

const PokerTable: React.FC<Prop> = ({numImages, pot}) => {
  
  return (
    <div className="poker-table">
      <div className="table-div">
      <div className="table-components">
        
        {numImages >= 1 ? <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png'/> : null}
        {numImages >= 2 ? <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png'/> : null}
        {numImages >= 3 ? <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png'/> : null}
        {numImages >= 4 ? <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png'/> : null}
        {numImages >= 5 ? <img src='./src/assets/PNG-cards-1.3/2_of_clubs.png'/> : null}
      </div>
      <div className="pot">
        <p>Pot: ${pot}</p>
      </div>
      <div className="buttons">
        <Actions call_check={"Call"}/>
      </div>
      </div>
    </div>
  );
};

export default PokerTable;