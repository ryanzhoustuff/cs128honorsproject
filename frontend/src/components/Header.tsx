import React from 'react';
import './Header.css'

type Prop = {
  names: string[];
  hrefs: string[];
};

const Header = ({ names, hrefs }: Prop) => {
  return (
    <div>
      <header>
        <h1>Honors Poker</h1>
      <nav>
        <ul className="header">
          {names.map((name, index) => 
            <li className="nav-item">
            <a className="nav-link active" aria-current="page" href={hrefs[index]}>
              {name}
            </a>
          </li>
          )}
        </ul>
      </nav>
      </header>
    </div>
  );
};

export default Header;
