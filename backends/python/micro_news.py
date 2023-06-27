from flask import Flask, jsonify
from flask_cors import CORS, cross_origin
import requests

app = Flask(__name__)
CORS(app)

API_KEY = '8dade0feadcf43e285e215cc7271de9c'
URL = f'https://newsapi.org/v2/top-headlines?country=us&apiKey={API_KEY}'

@app.route('/api/getArticle/<title>', methods=['GET'])
def get_article(title):
    response = requests.get(URL)
    data = response.json()
    article = next((a for a in data['articles'] if a['title'] == title), None)
    response = jsonify(article)
    response.headers.add('Access-Control-Allow-Origin', '*')
    response.headers.add('Access-Control-Allow-Methods', 'GET')
    response.headers.add('Access-Control-Allow-Headers', 'Content-Type')
    return response

@app.route('/api/getAuthorArticles/<author>', methods=['GET'])
def get_author_articles(author):
    response = requests.get(URL)
    data = response.json()
    articles = [a for a in data['articles'] if a['author'] == author]
    response = jsonify(articles)
    response.headers.add('Access-Control-Allow-Origin', '*')
    response.headers.add('Access-Control-Allow-Methods', 'GET')
    response.headers.add('Access-Control-Allow-Headers', 'Content-Type')
    return response

@app.route('/api/getTopHeadlines', methods=['GET'])
def get_top_headlines():        
    response = requests.get(f'https://newsapi.org/v2/top-headlines?country=us&apiKey={API_KEY}')
    response = jsonify(response.json())
    response.headers.add('Access-Control-Allow-Origin', '*')
    response.headers.add('Access-Control-Allow-Methods', 'GET')
    response.headers.add('Access-Control-Allow-Headers', 'Content-Type')
    return response

if __name__ == '__main__':
    app.run(port=8080)