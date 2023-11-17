// import cookiesDrop from '@/scripts/cookieDrop';
import Link from 'next/link';
import React from 'react';

function Header() {
  // const onLogout = async () => {
  //     cookiesDrop()
  // }

  return (
    <nav className="navbar navbar-expand-lg navbar-light bg-light">
      <a className="navbar-brand" href="/" style={{ paddingLeft: "10px" }}>NEXTJobs</a>
      <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarNavAltMarkup" aria-controls="navbarNavAltMarkup" aria-expanded="false" aria-label="Toggle navigation">
        <span className="navbar-toggler-icon"></span>
      </button>
      <div className="collapse navbar-collapse" id="navbarNavAltMarkup">
        <div className="navbar-nav">
          <Link className="nav-item nav-link" href="/login">Login</Link>
          <Link className="nav-item nav-link" href="/register">Register</Link>
        </div>
      </div>
    </nav>
  );
}

export default Header;
