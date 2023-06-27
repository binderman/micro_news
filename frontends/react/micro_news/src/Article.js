import React, { useState } from "react";
import axios from "axios";

const API_URL = "http://127.0.0.1:5000";

function Article() {
  const [title, setTitle] = useState("");
  const [data, setData] = useState(null);

  const handleClick = async () => {
    const encodedTitle = encodeURIComponent(title);
    const result = await axios.get(`${API_URL}/api/getArticle/${encodedTitle}`);
    setData(result.data);
  };

  return (
    <div>
      <input value={title} onChange={(e) => setTitle(e.target.value)} />
      <button onClick={handleClick}>Search</button>
      {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
    </div>
  );
}

export default Article;
