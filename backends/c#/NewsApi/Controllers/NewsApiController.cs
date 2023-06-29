using Microsoft.AspNetCore.Cors;
using Microsoft.AspNetCore.Mvc;
using Newtonsoft.Json.Linq;

namespace NewsAPI.Controllers
{
    [Route("api/")]
    [ApiController]
    [EnableCors("MyPolicy")]
    public class NewsController : ControllerBase
    {
        private readonly HttpClient _httpClient;
        private readonly ILogger _logger;
        private const string ApiKey = "8dade0feadcf43e285e215cc7271de9c";
        private const string BaseUrl = "https://newsapi.org/v2/";

        public NewsController(ILogger<NewsController> logger)
        {
            _httpClient = new HttpClient();
            _httpClient.DefaultRequestHeaders.UserAgent.ParseAdd("Micro News");
            _logger = logger;
        }

        private async Task<IActionResult> GetNewsAsync(string url)
        {
            var response = await _httpClient.GetAsync(url);
            AddCorsHeaders();

            if (response.IsSuccessStatusCode)
            {
                var content = await response.Content.ReadAsStringAsync();
                return Content(content, "application/json");
            }

            var errorContent = await response.Content.ReadAsStringAsync();
            _logger.LogWarning("Error retrieving news. Response: {Response}. Content: {Content}", response, errorContent);
            return BadRequest("Error retrieving news");
        }

        private void AddCorsHeaders()
        {
            Response.Headers.Add("Access-Control-Allow-Origin", "*");
            Response.Headers.Add("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE");
            Response.Headers.Add("Access-Control-Allow-Headers", "Content-Type");
        }

        [HttpGet("getArticle/{title}")]
        public async Task<IActionResult> GetArticle(string title)
        {
            return await GetNewsAsync($"{BaseUrl}everything?q={title}&apiKey={ApiKey}");
        }

        [HttpGet("getAuthorArticles/{author}")]
        public async Task<IActionResult> GetAuthorArticles(string author)
        {
            return await GetNewsAsync($"{BaseUrl}everything?q={author}&apiKey={ApiKey}");
        }

        [HttpGet("getTopHeadlines")]
        public async Task<IActionResult> GetTopHeadlines()
        {
            return await GetNewsAsync($"{BaseUrl}top-headlines?country=us&apiKey={ApiKey}");
        }
    }
}
