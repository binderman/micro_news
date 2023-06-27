const express = require("express");
const axios = require("axios");

const app = express();
const port = 8080;

const API_KEY = "8dade0feadcf43e285e215cc7271de9c";
const BASE_URL = "https://newsapi.org/v2";

// Middleware para tratamento de CORS
app.use((req, res, next) => {
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader("Access-Control-Allow-Methods", "GET");
  next();
});

// Rota para obter os Top Headlines
app.get("/api/getTopHeadlines", async (req, res) => {
  try {
    const url = `${BASE_URL}/top-headlines?country=us&apiKey=${API_KEY}`;
    const response = await axios.get(url);
    res.json(response.data);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: "Failed to fetch top headlines" });
  }
});

// Rota para obter artigos de um autor
app.get("/api/getAuthorArticles/:author", async (req, res) => {
  try {
    const { author } = req.params;
    const url = `${BASE_URL}/everything?language=en&apiKey=${API_KEY}&q=${author}`;
    const response = await axios.get(url);
    res.json(response.data);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: "Failed to fetch author articles" });
  }
});

// Rota para obter um artigo pelo título
app.get("/api/getArticle/:title", async (req, res) => {
  try {
    const { title } = req.params;
    const url = `${BASE_URL}/everything?language=en&apiKey=${API_KEY}&qInTitle=${title}`;
    const response = await axios.get(url);
    res.json(response.data);
  } catch (error) {
    console.error(error);
    res.status(500).json({ error: "Failed to fetch article" });
  }
});

// Iniciar o servidor
app.listen(port, () => {
  console.log(`Server running on port ${port}`);
});
