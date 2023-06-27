import React from "react";
import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import axios from "axios";
import TopHeadlines from "./TopHeadlines";
import Author from "./Author";
import Article from "./Article";

const API_URL = "http://localhost:3000";

function App() {
  return (
    <Router>
      <div>
        <nav>
          <ul>
            <li>
              <Link to="/topheadlines">Top Headlines</Link>
            </li>
            <li>
              <Link to="/author">Author</Link>
            </li>
            <li>
              <Link to="/article">Article</Link>
            </li>
          </ul>
        </nav>

        <Routes>
          <Route path="/topheadlines" element={<TopHeadlines />} />
          <Route path="/author" element={<Author />} />
          <Route path="/article" element={<Article />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
