import './App.css';
import { BrowserRouter as Router, Routes, Route} from 'react-router-dom';
import User from './pages/User';
import Login from './pages/Login';

function App() {
  return (
    <Router>
      <Routes>
        <Route path='/' exact element={<Login />} />
        <Route path='/user' element={<User />} />
      </Routes>
    </Router>
  );
}

export default App;
