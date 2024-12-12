import './PokerTable.css';
import React, { useState } from 'react';
import Actions from './Actions';

type Prop = {
  numImages: number;
  pot: number;
};

const PokerTable: React.FC<Prop> = ({ numImages, pot }) => {
  function getImageURL(name: string) {
    return new URL(`../assets/${name}`, import.meta.url).href;
  }
  return (
    <div className="poker-table">
      <div className="table-div">
        <div className="table-components">

          {numImages >= 1 ? <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" /> : null}
          {numImages >= 2 ? <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" /> : null}
          {numImages >= 3 ? <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" /> : null}
          {numImages >= 4 ? <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" /> : null}
          {numImages >= 5 ? <img src={getImageURL('PNG-cards-1.3/2_of_clubs.png')} alt="2 of Clubs" /> : null}
        </div>
        <div className="pot">
          <p>Pot: ${pot}</p>
        </div>
        <div className="buttons">
          <Actions call_check={"Call"} />
        </div>
      </div>
    </div>
  );
};

export default PokerTable;