package src.main.java;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.client.RestTemplate;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@SpringBootApplication
public class Server {

    private static final String API_KEY = "8dade0feadcf43e285e215cc7271de9c";
    private static final String BASE_URL = "https://newsapi.org/v2";

    public static void main(String[] args) {
        SpringApplication.run(Server.class, args);
    }

    @RestController
    public static class ApiController {

        private final RestTemplate restTemplate = new RestTemplate();

        @GetMapping("/api/getTopHeadlines")
        public String getTopHeadlines() {
            String url = BASE_URL + "/top-headlines?country=us&apiKey=" + API_KEY;
            return restTemplate.getForObject(url, String.class);
        }

        @GetMapping("/api/getAuthorArticles/{author}")
        public String getAuthorArticles(@PathVariable String author) {
            String url = BASE_URL + "/everything?language=en&apiKey=" + API_KEY + "&q=" + author;
            return restTemplate.getForObject(url, String.class);
        }

        @GetMapping("/api/getArticle/{title}")
        public String getArticle(@PathVariable String title) {
            String url = BASE_URL + "/everything?language=en&apiKey=" + API_KEY + "&qInTitle=" + title;
            return restTemplate.getForObject(url, String.class);
        }
    }

    @Bean
    public WebMvcConfigurer corsConfigurer() {
        return new WebMvcConfigurer() {
            @Override
            public void addCorsMappings(CorsRegistry registry) {
                registry.addMapping("/api/**")
                        .allowedOrigins("*")
                        .allowedMethods("GET");
            }
        };
    }
}
