import React, { useState } from "react";
import axios from "axios";

const API_URL = "http://127.0.0.1:5000";

function Author() {
  const [author, setAuthor] = useState("");
  const [data, setData] = useState(null);

  const handleClick = async () => {
    const encodedAuthor = encodeURIComponent(author);
    const result = await axios.get(
      `${API_URL}/api/getAuthorArticles/${encodedAuthor}`
    );
    setData(result.data);
  };

  return (
    <div>
      <input value={author} onChange={(e) => setAuthor(e.target.value)} />
      <button onClick={handleClick}>Search</button>
      {data && <pre>{JSON.stringify(data, null, 2)}</pre>}
    </div>
  );
}

export default Author;
