import { Link } from "react-router-dom";
import "./Navbar.css";

function Navbar() {
  return (
    <div className="nav-container">
      <div className="nav-logo">FinWise</div>

      <div className="nav-links">

        <Link to="/dashboard">Dashboard</Link>

      </div>
    </div>
  );
}

export default Navbar;
