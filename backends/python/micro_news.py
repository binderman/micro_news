from flask import Flask, jsonify
import requests

app = Flask(__name__)

API_KEY = '8dade0feadcf43e285e215cc7271de9c'
URL = f'https://newsapi.org/v2/top-headlines?country=us&apiKey={API_KEY}'

@app.route('/api/getArticle/<title>', methods=['GET'])
def get_article(title):
    response = requests.get(URL)
    data = response.json()
    article = next((a for a in data['articles'] if a['title'] == title), None)
    return jsonify(article)

@app.route('/api/getAuthorArticles/<author>', methods=['GET'])
def get_author_articles(author):
    response = requests.get(URL)
    data = response.json()
    articles = [a for a in data['articles'] if a['author'] == author]
    return jsonify(articles)

@app.route('/getTopHeadlines', methods=['GET'])
def get_top_headlines():
    response = requests.get(f'https://newsapi.org/v2/top-headlines?country=us&apiKey={API_KEY}')
    return jsonify(response.json())

if __name__ == '__main__':
    app.run(port=5000)