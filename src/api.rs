use serde_json::{json, Value};

use crate::client::TraktClient;
use crate::pagination::Pagination;

/// Trakt API wrapper with all endpoint implementations.
pub struct Api;

impl Api {
    // ============ Movies ============
    
    /// Get box office movies.
    pub async fn get_movies_boxoffice(
        client: &TraktClient,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/movies/boxoffice{}", query_string), None).await
    }
    
    /// Get most played movies.
    pub async fn get_movies_played(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/movies/played{}", query_string), None).await
    }
    
    /// Get most watched movies.
    pub async fn get_movies_watched(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/movies/watched{}", query_string), None).await
    }
    
    /// Get most collected movies.
    pub async fn get_movies_collected(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/movies/collected{}", query_string), None).await
    }
    
    /// Get most anticipated movies.
    pub async fn get_movies_anticipated(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/movies/anticipated{}", query_string), None).await
    }
    
    /// Get most favorited movies.
    pub async fn get_movies_favorited(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/movies/favorited{}", query_string), None).await
    }
    
    /// Get DVD releases.
    pub async fn get_movies_dvd_releases(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/movies/dvd_releases{}", query_string), None).await
    }
    
    /// Get streaming releases.
    pub async fn get_movies_streaming_releases(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/movies/streaming_releases{}", query_string), None).await
    }
    
    /// Get movie studios.
    pub async fn get_movie_studios(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/studios", id);
        client.get(&endpoint, None).await
    }
    
    /// Get users watching movie right now.
    pub async fn get_movie_watching(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/movies/{}/watching{}", id, query_string), None).await
    }
    
    /// Get movie videos.
    pub async fn get_movie_videos(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/videos", id);
        client.get(&endpoint, None).await
    }
    
    /// Refresh movie metadata (VIP only).
    pub async fn refresh_movie_metadata(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/metadata", id);
        client.post(&endpoint, None).await
    }
    
    /// Get popular movies.
    pub async fn get_popular_movies(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let mut query_params: Vec<String> = Vec::new();
        if let Some(ext) = extended {
            query_params.push(format!("extended={}", ext));
        }
        if let Some(ref pg) = pagination {
            if let Some(page) = pg.page {
                query_params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                query_params.push(format!("limit={}", limit));
            }
        }
        
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        client.get(&format!("/movies/popular{}", query_string), None).await
    }
    
    /// Get trending movies.
    pub async fn get_trending_movies(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let mut query_params: Vec<String> = Vec::new();
        if let Some(ext) = extended {
            query_params.push(format!("extended={}", ext));
        }
        if let Some(ref pg) = pagination {
            if let Some(page) = pg.page {
                query_params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                query_params.push(format!("limit={}", limit));
            }
        }
        
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        client.get(&format!("/movies/trending{}", query_string), None).await
    }
    
    /// Get movie by ID.
    pub async fn get_movie(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/movies/{}{}", id, query_string), None).await
    }
    
    /// Get movie aliases.
    pub async fn get_movie_aliases(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/aliases", id);
        client.get(&endpoint, None).await
    }
    
    /// Get movie releases.
    pub async fn get_movie_releases(
        client: &TraktClient,
        id: &str,
        country: Option<&str>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/releases{}", 
            id, 
            country.map(|c| format!("?country={}", c)).unwrap_or_default()
        );
        client.get(&endpoint, None).await
    }
    
    /// Get movie translations.
    pub async fn get_movie_translations(
        client: &TraktClient,
        id: &str,
        language: Option<&str>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/translations{}", 
            id, 
            language.map(|l| format!("?language={}", l)).unwrap_or_default()
        );
        client.get(&endpoint, None).await
    }
    
    /// Get movie comments.
    pub async fn get_movie_comments(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/comments{}", 
            id, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get movie lists.
    pub async fn get_movie_lists(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/lists{}", 
            id, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get movie ratings.
    pub async fn get_movie_ratings(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/ratings", id);
        client.get(&endpoint, None).await
    }
    
    /// Get movie related.
    pub async fn get_movie_related(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/related{}", 
            id, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get movie stats.
    pub async fn get_movie_stats(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/movies/{}/stats", id);
        client.get(&endpoint, None).await
    }
    
    // ============ Shows ============
    
    /// Get popular shows.
    pub async fn get_popular_shows(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let mut query_params: Vec<String> = Vec::new();
        if let Some(ext) = extended {
            query_params.push(format!("extended={}", ext));
        }
        if let Some(ref pg) = pagination {
            if let Some(page) = pg.page {
                query_params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                query_params.push(format!("limit={}", limit));
            }
        }
        
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        client.get(&format!("/shows/popular{}", query_string), None).await
    }
    
    /// Get trending shows.
    pub async fn get_trending_shows(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let mut query_params: Vec<String> = Vec::new();
        if let Some(ext) = extended {
            query_params.push(format!("extended={}", ext));
        }
        if let Some(ref pg) = pagination {
            if let Some(page) = pg.page {
                query_params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                query_params.push(format!("limit={}", limit));
            }
        }
        
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };
        
        client.get(&format!("/shows/trending{}", query_string), None).await
    }
    
    /// Get show by ID.
    pub async fn get_show(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}{}", id, query_string), None).await
    }
    
    /// Get show aliases.
    pub async fn get_show_aliases(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/aliases", id);
        client.get(&endpoint, None).await
    }
    
    /// Get show translations.
    pub async fn get_show_translations(
        client: &TraktClient,
        id: &str,
        language: Option<&str>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/translations{}", 
            id, 
            language.map(|l| format!("?language={}", l)).unwrap_or_default()
        );
        client.get(&endpoint, None).await
    }
    
    /// Get show comments.
    pub async fn get_show_comments(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/comments{}", 
            id, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get show lists.
    pub async fn get_show_lists(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/lists{}", 
            id, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get show ratings.
    pub async fn get_show_ratings(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/ratings", id);
        client.get(&endpoint, None).await
    }
    
    /// Get show related.
    pub async fn get_show_related(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/related{}", 
            id, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get show stats.
    pub async fn get_show_stats(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/stats", id);
        client.get(&endpoint, None).await
    }
    
    /// Get show seasons.
    pub async fn get_show_seasons(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons{}", id, query_string), None).await
    }
    
    /// Get season by ID.
    pub async fn get_season(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons/{}{}", show_id, season_id, query_string), None).await
    }
    
    /// Get season episodes.
    pub async fn get_season_episodes(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons/{}/episodes{}", show_id, season_id, query_string), None).await
    }
    
    /// Get episode.
    pub async fn get_episode(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        episode_id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons/{}/episodes/{}{}", show_id, season_id, episode_id, query_string), None).await
    }
    
    // ============ People ============
    
    /// Get person.
    pub async fn get_person(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/people/{}{}", id, query_string), None).await
    }
    
    /// Get person movies.
    pub async fn get_person_movies(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/people/{}/movies", id);
        client.get(&endpoint, None).await
    }
    
    /// Get person shows.
    pub async fn get_person_shows(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/people/{}/shows", id);
        client.get(&endpoint, None).await
    }
    
    // ============ Users ============
    
    /// Get user settings.
    pub async fn get_user_settings(client: &TraktClient) -> crate::Result<Value> {
        client.get("/users/settings", None).await
    }
    
    /// Get user profile.
    pub async fn get_user_profile(client: &TraktClient, username: &str) -> crate::Result<Value> {
        let endpoint = format!("/users/{}", username);
        client.get(&endpoint, None).await
    }
    
    /// Get user collection (movies).
    pub async fn get_user_collection_movies(
        client: &TraktClient,
        username: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/users/{}/collection/movies{}", username, query_string), None).await
    }
    
    /// Get user collection (shows).
    pub async fn get_user_collection_shows(
        client: &TraktClient,
        username: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/users/{}/collection/shows{}", username, query_string), None).await
    }
    
    /// Get user watchlist (movies).
    pub async fn get_user_watchlist_movies(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/watchlist/movies{}", 
            username, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get user watchlist (shows).
    pub async fn get_user_watchlist_shows(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/watchlist/shows{}", 
            username, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get user history (movies).
    pub async fn get_user_history_movies(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/history/movies{}", 
            username, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get user history (shows).
    pub async fn get_user_history_shows(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/history/shows{}", 
            username, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get user history (episodes).
    pub async fn get_user_history_episodes(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/history/episodes{}", 
            username, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get user ratings.
    pub async fn get_user_ratings(
        client: &TraktClient,
        username: &str,
        type_: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = type_.map(|t| format!("?type={}", t)).unwrap_or_default();
        let endpoint = format!("/users/{}/ratings{}", username, query_string);
        client.get(&endpoint, None).await
    }
    
    /// Get user lists.
    pub async fn get_user_lists(
        client: &TraktClient,
        username: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists", username);
        client.get(&endpoint, None).await
    }
    
    // ============ Search ============
    
    /// Search for movies, shows, episodes, people, or lists.
    pub async fn search(
        client: &TraktClient,
        query: &str,
        type_: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let mut query_params = format!("query={}&type={}", urlencoding::encode(query), type_);
        if let Some(ref pg) = pagination {
            if let Some(page) = pg.page {
                query_params.push_str(&format!("&page={}", page));
            }
            if let Some(limit) = pg.limit {
                query_params.push_str(&format!("&limit={}", limit));
            }
        }
        
        client.get(&format!("/search?{}", query_params), None).await
    }
    
    /// Search by ID (IMDB, TMDB, TVDB, etc.).
    pub async fn search_id(
        client: &TraktClient,
        id_type: &str,
        id: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/search/{}:{}", id_type, id);
        client.get(&endpoint, None).await
    }
    
    // ============ Calendars ============
    
    /// Get calendar (my movies).
    pub async fn get_calendar_movies(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/my/movies{}{}", date_part, query_params), None).await
    }
    
    /// Get calendar (my shows).
    pub async fn get_calendar_shows(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/my/shows{}{}", date_part, query_params), None).await
    }
    
    /// Get all movies calendar.
    pub async fn get_all_movies_calendar(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/all/movies{}{}", date_part, query_params), None).await
    }
    
    /// Get all shows calendar.
    pub async fn get_all_shows_calendar(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/all/shows{}{}", date_part, query_params), None).await
    }
    
    /// Get user's show premieres calendar.
    pub async fn get_calendar_shows_premieres(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/my/shows/premieres{}{}", date_part, query_params), None).await
    }
    
    /// Get new shows calendar.
    pub async fn get_calendar_new_shows(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/my/shows/new{}{}", date_part, query_params), None).await
    }
    
    /// Get user's show finales calendar.
    pub async fn get_calendar_shows_finales(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/my/shows/finales{}{}", date_part, query_params), None).await
    }
    
    /// Get user's streaming releases calendar.
    pub async fn get_calendar_streaming(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/my/streaming{}{}", date_part, query_params), None).await
    }
    
    /// Get user's DVD releases calendar.
    pub async fn get_calendar_dvd(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/my/dvd{}{}", date_part, query_params), None).await
    }
    
    /// Get all new shows calendar.
    pub async fn get_all_new_shows_calendar(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/all/shows/new{}{}", date_part, query_params), None).await
    }
    
    /// Get all season premieres calendar.
    pub async fn get_all_shows_premieres_calendar(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/all/shows/premieres{}{}", date_part, query_params), None).await
    }
    
    /// Get all finales calendar.
    pub async fn get_all_shows_finales_calendar(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/all/shows/finales{}{}", date_part, query_params), None).await
    }
    
    /// Get all streaming releases calendar.
    pub async fn get_all_streaming_calendar(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/all/streaming{}{}", date_part, query_params), None).await
    }
    
    /// Get all DVD releases calendar.
    pub async fn get_all_dvd_calendar(
        client: &TraktClient,
        start_date: Option<&str>,
        days: Option<u32>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let mut query_params = String::new();
        if let Some(d) = days {
            query_params = format!("?days={}", d);
        }
        
        client.get(&format!("/calendars/all/dvd{}{}", date_part, query_params), None).await
    }
    
    // ============ Genres ============
    
    /// Get movie genres.
    pub async fn get_movie_genres(client: &TraktClient) -> crate::Result<Value> {
        client.get("/genres/movies", None).await
    }
    
    /// Get show genres.
    pub async fn get_show_genres(client: &TraktClient) -> crate::Result<Value> {
        client.get("/genres/shows", None).await
    }
    
    // ============ Lists ============
    
    /// Get list.
    pub async fn get_list(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}{}", id, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get list items.
    pub async fn get_list_items(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/items{}", id, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    // ============ Comments ============
    
    /// Get comment.
    pub async fn get_comment(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}", id);
        client.get(&endpoint, None).await
    }
    
    /// Get comment replies.
    pub async fn get_comment_replies(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}/replies{}", 
            id, 
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Update a comment.
    pub async fn update_comment(
        client: &TraktClient,
        id: &str,
        comment: &str,
        spoiler: Option<bool>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}", id);
        let mut data = json!({
            "comment": comment,
        });
        if let Some(s) = spoiler {
            data["spoiler"] = json!(s);
        }
        client.put(&endpoint, Some(&data)).await
    }
    
    /// Delete a comment.
    pub async fn delete_comment(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}", id);
        client.delete(&endpoint).await
    }
    
    /// Get comment item.
    pub async fn get_comment_item(
        client: &TraktClient,
        id: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}/item", id);
        client.get(&endpoint, None).await
    }
    
    /// Get comment likes.
    pub async fn get_comment_likes(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}/likes{}", id, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Like a comment.
    pub async fn like_comment(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}/like", id);
        client.post(&endpoint, None).await
    }
    
    /// Unlike a comment.
    pub async fn unlike_comment(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}/like", id);
        client.delete(&endpoint).await
    }
    
    /// Get trending comments.
    pub async fn get_trending_comments(
        client: &TraktClient,
        comment_type: Option<&str>,
        type_: Option<&str>,
        include_replies: Option<bool>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let mut query_parts = Vec::new();
        if let Some(ct) = comment_type {
            query_parts.push(format!("comment_type={}", ct));
        }
        if let Some(t) = type_ {
            query_parts.push(format!("type={}", t));
        }
        if let Some(ir) = include_replies {
            query_parts.push(format!("include_replies={}", ir));
        }
        let query_string = Self::build_query_string(pagination.as_ref());
        let full_query = if !query_parts.is_empty() && !query_string.is_empty() {
            format!("{}?{}", query_string, query_parts.join("&"))
        } else if !query_parts.is_empty() {
            format!("?{}", query_parts.join("&"))
        } else {
            query_string
        };
        let endpoint = format!("/comments/trending{}", full_query);
        client.get(&endpoint, None).await
    }
    
    // ============ Updates ============
    
    /// Get movies updated.
    pub async fn get_updated_movies(
        client: &TraktClient,
        start_date: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let query_part = Self::build_query_string(pagination.as_ref());
        client.get(&format!("/updates/movies{}{}", date_part, query_part), None).await
    }
    
    /// Get shows updated.
    pub async fn get_updated_shows(
        client: &TraktClient,
        start_date: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let query_part = Self::build_query_string(pagination.as_ref());
        client.get(&format!("/updates/shows{}{}", date_part, query_part), None).await
    }
    
    /// Get episodes updated.
    pub async fn get_updated_episodes(
        client: &TraktClient,
        start_date: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let query_part = Self::build_query_string(pagination.as_ref());
        client.get(&format!("/updates/episodes{}{}", date_part, query_part), None).await
    }
    
    // ============ Shows ============
    
    /// Get most played shows.
    pub async fn get_shows_played(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/shows/played{}", query_string), None).await
    }
    
    /// Get most watched shows.
    pub async fn get_shows_watched(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/shows/watched{}", query_string), None).await
    }
    
    /// Get most collected shows.
    pub async fn get_shows_collected(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/shows/collected{}", query_string), None).await
    }
    
    /// Get most anticipated shows.
    pub async fn get_shows_anticipated(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/shows/anticipated{}", query_string), None).await
    }
    
    /// Get most favorited shows.
    pub async fn get_shows_favorited(
        client: &TraktClient,
        pagination: Option<Pagination>,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = Self::build_full_query_string(pagination.as_ref(), extended);
        client.get(&format!("/shows/favorited{}", query_string), None).await
    }
    
    /// Get show certifications.
    pub async fn get_show_certifications(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/certifications", id);
        client.get(&endpoint, None).await
    }
    
    /// Get show collection progress.
    pub async fn get_show_collection_progress(
        client: &TraktClient,
        id: &str,
        hidden: Option<bool>,
        specials: Option<bool>,
    ) -> crate::Result<Value> {
        let mut params = Vec::new();
        if let Some(h) = hidden {
            params.push(format!("hidden={}", h));
        }
        if let Some(s) = specials {
            params.push(format!("specials={}", s));
        }
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        client.get(&format!("/shows/{}/collection/progress{}", id, query_string), None).await
    }
    
    /// Get show watched progress.
    pub async fn get_show_watched_progress(
        client: &TraktClient,
        id: &str,
        hidden: Option<bool>,
        specials: Option<bool>,
    ) -> crate::Result<Value> {
        let mut params = Vec::new();
        if let Some(h) = hidden {
            params.push(format!("hidden={}", h));
        }
        if let Some(s) = specials {
            params.push(format!("specials={}", s));
        }
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        
        client.get(&format!("/shows/{}/watched/progress{}", id, query_string), None).await
    }
    
    /// Reset show progress.
    pub async fn reset_show_progress(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/progress/reset", id);
        client.post(&endpoint, None).await
    }
    
    /// Undo reset show progress (VIP only).
    pub async fn undo_reset_show_progress(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/progress/reset/undo", id);
        client.delete(&endpoint).await
    }
    
    /// Get show studios.
    pub async fn get_show_studios(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/studios", id);
        client.get(&endpoint, None).await
    }
    
    /// Get users watching show right now.
    pub async fn get_show_watching(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/watching{}", id, query_string), None).await
    }
    
    /// Get next episode for a show.
    pub async fn get_show_next_episode(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/next_episode{}", id, query_string), None).await
    }
    
    /// Get last episode for a show.
    pub async fn get_show_last_episode(
        client: &TraktClient,
        id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/last_episode{}", id, query_string), None).await
    }
    
    /// Get show videos.
    pub async fn get_show_videos(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/videos", id);
        client.get(&endpoint, None).await
    }
    
    /// Refresh show metadata (VIP only).
    pub async fn refresh_show_metadata(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/metadata", id);
        client.post(&endpoint, None).await
    }
    
    // ============ Seasons ============
    
    /// Get season translations.
    pub async fn get_season_translations(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        language: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = language.map(|l| format!("?language={}", l)).unwrap_or_default();
        client.get(&format!("/shows/{}/seasons/{}/translations{}", show_id, season_id, query_string), None).await
    }
    
    /// Get season comments.
    pub async fn get_season_comments(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/comments{}", 
            show_id, 
            season_id,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get lists containing season.
    pub async fn get_season_lists(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/lists{}", 
            show_id, 
            season_id,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get people for a season.
    pub async fn get_season_people(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons/{}/people{}", show_id, season_id, query_string), None).await
    }
    
    /// Get season ratings.
    pub async fn get_season_ratings(client: &TraktClient, show_id: &str, season_id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/ratings", show_id, season_id);
        client.get(&endpoint, None).await
    }
    
    /// Get season stats.
    pub async fn get_season_stats(client: &TraktClient, show_id: &str, season_id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/stats", show_id, season_id);
        client.get(&endpoint, None).await
    }
    
    /// Get users watching season right now.
    pub async fn get_season_watching(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons/{}/watching{}", show_id, season_id, query_string), None).await
    }
    
    /// Get season videos.
    pub async fn get_season_videos(client: &TraktClient, show_id: &str, season_id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/videos", show_id, season_id);
        client.get(&endpoint, None).await
    }
    
    // ============ Episodes ============
    
    /// Get episode translations.
    pub async fn get_episode_translations(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        episode_id: &str,
        language: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = language.map(|l| format!("?language={}", l)).unwrap_or_default();
        client.get(&format!("/shows/{}/seasons/{}/episodes/{}/translations{}", show_id, season_id, episode_id, query_string), None).await
    }
    
    /// Get episode comments.
    pub async fn get_episode_comments(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        episode_id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/episodes/{}/comments{}", 
            show_id, 
            season_id,
            episode_id,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get lists containing episode.
    pub async fn get_episode_lists(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        episode_id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/episodes/{}/lists{}", 
            show_id, 
            season_id,
            episode_id,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    /// Get people for an episode.
    pub async fn get_episode_people(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        episode_id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons/{}/episodes/{}/people{}", show_id, season_id, episode_id, query_string), None).await
    }
    
    /// Get episode ratings.
    pub async fn get_episode_ratings(client: &TraktClient, show_id: &str, season_id: &str, episode_id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/episodes/{}/ratings", show_id, season_id, episode_id);
        client.get(&endpoint, None).await
    }
    
    /// Get episode stats.
    pub async fn get_episode_stats(client: &TraktClient, show_id: &str, season_id: &str, episode_id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/episodes/{}/stats", show_id, season_id, episode_id);
        client.get(&endpoint, None).await
    }
    
    /// Get users watching episode right now.
    pub async fn get_episode_watching(
        client: &TraktClient,
        show_id: &str,
        season_id: &str,
        episode_id: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/shows/{}/seasons/{}/episodes/{}/watching{}", show_id, season_id, episode_id, query_string), None).await
    }
    
    /// Get episode videos.
    pub async fn get_episode_videos(client: &TraktClient, show_id: &str, season_id: &str, episode_id: &str) -> crate::Result<Value> {
        let endpoint = format!("/shows/{}/seasons/{}/episodes/{}/videos", show_id, season_id, episode_id);
        client.get(&endpoint, None).await
    }
    
    // ============ People ============
    
    /// Get lists containing person.
    pub async fn get_person_lists(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/people/{}/lists{}", id, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Refresh person metadata (VIP only).
    pub async fn refresh_person_metadata(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/people/{}/metadata", id);
        client.post(&endpoint, None).await
    }
    
    /// Get recently updated people.
    pub async fn get_updated_people(
        client: &TraktClient,
        start_date: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let query_part = Self::build_query_string(pagination.as_ref());
        client.get(&format!("/updates/people{}{}", date_part, query_part), None).await
    }
    
    /// Get recently updated people Trakt IDs.
    pub async fn get_updated_people_ids(
        client: &TraktClient,
        start_date: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let date_part = start_date.map(|d| format!("/{}", d)).unwrap_or_default();
        let query_part = Self::build_query_string(pagination.as_ref());
        client.get(&format!("/updates/people_ids{}{}", date_part, query_part), None).await
    }
    
    // ============ Users (OAuth required) ============
    
    /// Get user likes.
    pub async fn get_user_likes(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/likes{}", username, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get user followers.
    pub async fn get_user_followers(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/followers{}", username, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get user following.
    pub async fn get_user_following(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/following{}", username, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get user friends.
    pub async fn get_user_friends(
        client: &TraktClient,
        username: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/friends{}", username, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get user watching now.
    pub async fn get_user_watching(
        client: &TraktClient,
        username: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/users/{}/watching{}", username, query_string), None).await
    }
    
    /// Get user watched (movies or shows).
    pub async fn get_user_watched(
        client: &TraktClient,
        username: &str,
        type_: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/users/{}/watched/{}{}", username, type_, query_string), None).await
    }
    
    /// Get user stats.
    pub async fn get_user_stats(client: &TraktClient, username: &str) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/stats", username);
        client.get(&endpoint, None).await
    }
    
    /// Get user hidden items.
    pub async fn get_user_hidden(
        client: &TraktClient,
        username: &str,
        type_: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let type_part = type_.map(|t| format!("/{}", t)).unwrap_or_default();
        let query_part = Self::build_query_string(pagination.as_ref());
        client.get(&format!("/users/{}/hidden{}{}", username, type_part, query_part), None).await
    }
    
    /// Add hidden items (OAuth required).
    pub async fn add_hidden_items(
        client: &TraktClient,
        username: &str,
        type_: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/hidden/{}", username, type_);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Remove hidden items (OAuth required).
    pub async fn remove_hidden_items(
        client: &TraktClient,
        username: &str,
        type_: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/hidden/{}/remove", username, type_);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Get pending following requests (OAuth required).
    pub async fn get_pending_requests(
        client: &TraktClient,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/requests{}", Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get follow requests (OAuth required).
    pub async fn get_follow_requests(
        client: &TraktClient,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/users/requests/following{}", query_string), None).await
    }
    
    /// Approve follow request (OAuth required).
    pub async fn approve_follow_request(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/users/requests/{}", id);
        client.post(&endpoint, None).await
    }
    
    /// Deny follow request (OAuth required).
    pub async fn deny_follow_request(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/users/requests/{}", id);
        client.delete(&endpoint).await
    }
    
    // ============ Sync (OAuth required) ============
    
    /// Get sync collection (movies).
    pub async fn get_sync_collection_movies(
        client: &TraktClient,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/sync/collection/movies{}", query_string), None).await
    }
    
    /// Get sync collection (shows).
    pub async fn getSync_collection_shows(
        client: &TraktClient,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/sync/collection/shows{}", query_string), None).await
    }
    
    /// Add to collection (OAuth required).
    pub async fn add_to_collection(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/collection", Some(&data)).await
    }
    
    /// Remove from collection (OAuth required).
    pub async fn remove_from_collection(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/collection/remove", Some(&data)).await
    }
    
    /// Get sync watched (movies or shows).
    pub async fn get_sync_watched(
        client: &TraktClient,
        type_: &str,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/sync/watched/{}{}", type_, query_string), None).await
    }
    
    /// Get sync history.
    pub async fn get_sync_history(
        client: &TraktClient,
        type_: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let type_part = type_.map(|t| format!("/{}", t)).unwrap_or_default();
        let endpoint = format!("/sync/history{}{}", type_part, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Add to history (OAuth required).
    pub async fn add_to_history(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/history", Some(&data)).await
    }
    
    /// Remove from history (OAuth required).
    pub async fn remove_from_history(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/history/remove", Some(&data)).await
    }
    
    /// Get sync ratings.
    pub async fn get_sync_ratings(
        client: &TraktClient,
        type_: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let type_part = type_.map(|t| format!("?type={}", t)).unwrap_or_default();
        let pagination_str = Self::build_query_string(pagination.as_ref());
        let separator = if type_part.is_empty() && pagination_str.is_empty() {
            String::new()
        } else if type_part.is_empty() {
            pagination_str
        } else if pagination_str.is_empty() {
            type_part
        } else {
            format!("{}&{}", type_part, pagination_str.trim_start_matches('?'))
        };
        
        let query_string = if separator.is_empty() {
            String::new()
        } else {
            format!("?{}", separator.trim_start_matches('?'))
        };
        
        client.get(&format!("/sync/ratings{}", query_string), None).await
    }
    
    /// Add ratings (OAuth required).
    pub async fn add_ratings(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/ratings", Some(&data)).await
    }
    
    /// Remove ratings (OAuth required).
    pub async fn remove_ratings(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/ratings/remove", Some(&data)).await
    }
    
    /// Get sync watchlist.
    pub async fn get_sync_watchlist(
        client: &TraktClient,
        type_: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let type_part = type_.map(|t| format!("?type={}", t)).unwrap_or_default();
        let pagination_str = Self::build_query_string(pagination.as_ref());
        
        let query_params: Vec<String> = Vec::new();
        let mut final_params: Vec<String> = Vec::new();
        
        if let Some(t) = type_ {
            final_params.push(format!("type={}", t));
        }
        if let Some(ref pg) = pagination {
            if let Some(page) = pg.page {
                final_params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                final_params.push(format!("limit={}", limit));
            }
        }
        
        let query_string = if final_params.is_empty() {
            String::new()
        } else {
            format!("?{}", final_params.join("&"))
        };
        
        client.get(&format!("/sync/watchlist{}", query_string), None).await
    }
    
    /// Add to watchlist (OAuth required).
    pub async fn add_to_watchlist(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/watchlist", Some(&data)).await
    }
    
    /// Remove from watchlist (OAuth required).
    pub async fn remove_from_watchlist(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/watchlist/remove", Some(&data)).await
    }
    
    /// Reorder watchlist (OAuth required).
    
    /// Get sync favorites.
    pub async fn get_sync_favorites(
        client: &TraktClient,
        type_: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let type_part = type_.map(|t| format!("?type={}", t)).unwrap_or_default();
        let mut final_params: Vec<String> = Vec::new();
        
        if let Some(t) = type_ {
            final_params.push(format!("type={}", t));
        }
        if let Some(ref pg) = pagination {
            if let Some(page) = pg.page {
                final_params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                final_params.push(format!("limit={}", limit));
            }
        }
        
        let query_string = if final_params.is_empty() {
            String::new()
        } else {
            format!("?{}", final_params.join("&"))
        };
        
        client.get(&format!("/sync/favorites{}", query_string), None).await
    }
    
    /// Add to favorites (OAuth required).
    pub async fn add_to_favorites(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/favorites", Some(&data)).await
    }
    
    /// Remove from favorites (OAuth required).
    pub async fn remove_from_favorites(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/sync/favorites/remove", Some(&data)).await
    }
    
    /// Reorder favorites (OAuth required).
    
    // ============ Scrobble / Checkin (OAuth required) ============
    
    /// Start scrobbling.
    pub async fn scrobble_start(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/scrobble/start", Some(&data)).await
    }
    
    /// Pause scrobbling.
    pub async fn scrobble_pause(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/scrobble/pause", Some(&data)).await
    }
    
    /// Stop scrobbling.
    pub async fn scrobble_stop(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/scrobble/stop", Some(&data)).await
    }
    
    /// Check into an item.
    pub async fn checkin(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/checkin", Some(&data)).await
    }
    
    /// Delete checkin.
    pub async fn delete_checkin(client: &TraktClient) -> crate::Result<Value> {
        client.delete("/checkin").await
    }
    
    /// Start watching (media center).
    pub async fn start_watching(
        client: &TraktClient,
        data: Value,
    ) -> crate::Result<Value> {
        client.post("/watching", Some(&data)).await
    }
    
    /// Stop watching (media center).
    pub async fn stop_watching(client: &TraktClient) -> crate::Result<Value> {
        client.post("/watching", None).await
    }
    
    /// Get playback progress.
    pub async fn get_playback_progress(
        client: &TraktClient,
        type_: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let type_part = type_.map(|t| format!("/{}", t)).unwrap_or_default();
        let endpoint = format!("/playback{}{}", type_part, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Remove playback item.
    pub async fn remove_playback_item(
        client: &TraktClient,
        id: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/playback/{}", id);
        client.delete(&endpoint).await
    }
    
    // ============ Comments ============
    
    /// Get recently created comments.
    pub async fn get_recently_created_comments(
        client: &TraktClient,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/comments/recently_created{}", Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get recently updated comments.
    pub async fn get_recently_updated_comments(
        client: &TraktClient,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/comments/recently_updated{}", Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Post a comment.
    pub async fn post_comment(
        client: &TraktClient,
        type_: &str,
        id: &str,
        comment: &str,
        spoiler: Option<bool>,
    ) -> crate::Result<Value> {
        let data = serde_json::json!({
            "comment": comment,
            "spoiler": spoiler.unwrap_or(false)
        });
        
        let endpoint = format!("/{}/{}", type_, id);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Get users who liked a comment.
    pub async fn get_comment_likers(
        client: &TraktClient,
        id: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/comments/{}/likers", id);
        client.get(&endpoint, None).await
    }
    
    // ============ Lists ============
    
    /// Get trending lists.
    pub async fn get_trending_lists(
        client: &TraktClient,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/trending{}", Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get popular lists.
    pub async fn get_popular_lists(
        client: &TraktClient,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/popular{}", Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Like a list.
    pub async fn like_list(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/like", id);
        client.post(&endpoint, None).await
    }
    
    /// Unlike a list.
    pub async fn unlike_list(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/like", id);
        client.delete(&endpoint).await
    }
    
    /// Get list likers.
    pub async fn get_list_likers(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/likes{}", id, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Get list comments.
    pub async fn get_list_comments(
        client: &TraktClient,
        id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/comments{}", id, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Create a list.
    pub async fn create_list(
        client: &TraktClient,
        name: &str,
        description: Option<&str>,
        privacy: Option<&str>,
    ) -> crate::Result<Value> {
        let mut data = serde_json::json!({
            "name": name
        });
        
        if let Some(desc) = description {
            data["description"] = serde_json::json!(desc);
        }
        if let Some(p) = privacy {
            data["privacy"] = serde_json::json!(p);
        }
        
        client.post("/users/me/lists", Some(&data)).await
    }
    
    /// Update a list.
    pub async fn update_list(
        client: &TraktClient,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        privacy: Option<&str>,
    ) -> crate::Result<Value> {
        let mut data = serde_json::json!({});
        
        if let Some(n) = name {
            data["name"] = serde_json::json!(n);
        }
        if let Some(desc) = description {
            data["description"] = serde_json::json!(desc);
        }
        if let Some(p) = privacy {
            data["privacy"] = serde_json::json!(p);
        }
        
        let endpoint = format!("/lists/{}", id);
        client.put(&endpoint, Some(&data)).await
    }
    
    /// Delete a list.
    pub async fn delete_list(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}", id);
        client.delete(&endpoint).await
    }
    
    /// Add items to list.
    pub async fn add_list_items(
        client: &TraktClient,
        id: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/items", id);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Remove items from list.
    pub async fn remove_list_items(
        client: &TraktClient,
        id: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/items/remove", id);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Reorder list items.
    pub async fn reorder_list_items(
        client: &TraktClient,
        id: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/lists/{}/items/reorder", id);
        client.post(&endpoint, Some(&data)).await
    }
    
    // ============ User Lists ============
    
    /// Get all lists a user can collaborate on.
    pub async fn get_user_collaborations(
        client: &TraktClient,
        username: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/collaborations", username);
        client.get(&endpoint, None).await
    }
    
    /// Create personal list.
    pub async fn create_personal_list(
        client: &TraktClient,
        name: &str,
        description: Option<&str>,
        privacy: Option<&str>,
    ) -> crate::Result<Value> {
        let mut data = serde_json::json!({
            "name": name
        });
        
        if let Some(desc) = description {
            data["description"] = serde_json::json!(desc);
        }
        if let Some(p) = privacy {
            data["privacy"] = serde_json::json!(p);
        }
        
        client.post("/users/me/lists", Some(&data)).await
    }
    
    /// Update personal list.
    pub async fn update_personal_list(
        client: &TraktClient,
        username: &str,
        list_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        privacy: Option<&str>,
    ) -> crate::Result<Value> {
        let mut data = serde_json::json!({});
        
        if let Some(n) = name {
            data["name"] = serde_json::json!(n);
        }
        if let Some(desc) = description {
            data["description"] = serde_json::json!(desc);
        }
        if let Some(p) = privacy {
            data["privacy"] = serde_json::json!(p);
        }
        
        let endpoint = format!("/users/{}/lists/{}", username, list_id);
        client.put(&endpoint, Some(&data)).await
    }
    
    /// Delete personal list.
    pub async fn delete_personal_list(
        client: &TraktClient,
        username: &str,
        list_id: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists/{}", username, list_id);
        client.delete(&endpoint).await
    }
    
    /// Like a user's list.
    pub async fn like_user_list(
        client: &TraktClient,
        username: &str,
        list_id: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists/{}/like", username, list_id);
        client.post(&endpoint, None).await
    }
    
    /// Unlike a user's list.
    pub async fn unlike_user_list(
        client: &TraktClient,
        username: &str,
        list_id: &str,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists/{}/like", username, list_id);
        client.delete(&endpoint).await
    }
    
    /// Get user list items (with VIP enhanced).
    pub async fn get_user_list_items(
        client: &TraktClient,
        username: &str,
        list_id: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists/{}/items{}", username, list_id, Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }
    
    /// Add items to personal list.
    pub async fn add_personal_list_items(
        client: &TraktClient,
        username: &str,
        list_id: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists/{}/items", username, list_id);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Remove items from personal list.
    pub async fn remove_personal_list_items(
        client: &TraktClient,
        username: &str,
        list_id: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists/{}/items/remove", username, list_id);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Reorder items on a list.
    pub async fn reorder_personal_list_items(
        client: &TraktClient,
        username: &str,
        list_id: &str,
        data: Value,
    ) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/lists/{}/items/reorder", username, list_id);
        client.post(&endpoint, Some(&data)).await
    }
    
    /// Follow a user.
    pub async fn follow_user(client: &TraktClient, username: &str) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/follow", username);
        client.post(&endpoint, None).await
    }
    
    /// Unfollow a user.
    pub async fn unfollow_user(client: &TraktClient, username: &str) -> crate::Result<Value> {
        let endpoint = format!("/users/{}/follow", username);
        client.delete(&endpoint).await
    }
    
    /// Get user recommendations (movies).
    pub async fn get_movie_recommendations(
        client: &TraktClient,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/recommendations/movies{}", query_string), None).await
    }
    
    /// Hide movie recommendation.
    pub async fn hide_movie_recommendation(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/recommendations/movies/{}", id);
        client.delete(&endpoint).await
    }
    
    /// Get user recommendations (shows).
    pub async fn get_show_recommendations(
        client: &TraktClient,
        extended: Option<&str>,
    ) -> crate::Result<Value> {
        let query_string = if let Some(ext) = extended {
            format!("?extended={}", ext)
        } else {
            String::new()
        };
        
        client.get(&format!("/recommendations/shows{}", query_string), None).await
    }
    
    /// Hide show recommendation.
    pub async fn hide_show_recommendation(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/recommendations/shows/{}", id);
        client.delete(&endpoint).await
    }
    
    // ============ Notes (VIP Enhanced) ============
    
    /// Add note (VIP Enhanced).
    pub async fn add_note(
        client: &TraktClient,
        noteable_type: &str,
        noteable_id: &str,
        note: &str,
    ) -> crate::Result<Value> {
        let data = serde_json::json!({
            "noteable_type": noteable_type,
            "noteable_id": noteable_id,
            "note": note
        });
        client.post("/notes", Some(&data)).await
    }
    
    /// Get note (VIP Enhanced).
    pub async fn get_note(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/notes/{}", id);
        client.get(&endpoint, None).await
    }
    
    /// Update note (VIP Enhanced).
    pub async fn update_note(client: &TraktClient, id: &str, note: &str) -> crate::Result<Value> {
        let data = serde_json::json!({
            "note": note
        });
        let endpoint = format!("/notes/{}", id);
        client.put(&endpoint, Some(&data)).await
    }
    
    /// Delete note (VIP Enhanced).
    pub async fn delete_note(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/notes/{}", id);
        client.delete(&endpoint).await
    }
    
    /// Get note attached item (VIP Enhanced).
    pub async fn get_note_item(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let endpoint = format!("/notes/{}/item", id);
        client.get(&endpoint, None).await
    }
    
    // ============ Saved Filters (VIP Only) ============
    
    /// Get saved filters (VIP Only).
    pub async fn get_saved_filters(
        client: &TraktClient,
        section: &str,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let endpoint = format!(
            "/users/saved_filters/{}{}",
            section,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    // ============ User Watchlist Item (VIP Enhanced) ============
    
    /// Update watchlist item (VIP Enhanced).
    pub async fn update_watchlist_item(
        client: &TraktClient,
        list_item_id: &str,
        note: Option<&str>,
    ) -> crate::Result<Value> {
        let data = serde_json::json!({
            "note": note.unwrap_or("")
        });
        let endpoint = format!("/sync/watchlist/{}", list_item_id);
        client.put(&endpoint, Some(&data)).await
    }
    
    /// Update favorite item (VIP Enhanced).
    pub async fn update_favorite_item(
        client: &TraktClient,
        list_item_id: &str,
        note: Option<&str>,
    ) -> crate::Result<Value> {
        let data = serde_json::json!({
            "note": note.unwrap_or("")
        });
        let endpoint = format!("/sync/favorites/{}", list_item_id);
        client.put(&endpoint, Some(&data)).await
    }
    
    // ============ User Comments ============
    
    /// Get user comments.
    pub async fn get_user_comments(
        client: &TraktClient,
        username: &str,
        comment_type: Option<&str>,
        type_: Option<&str>,
        include_replies: Option<bool>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let mut query_parts = Vec::new();
        if let Some(ct) = comment_type {
            query_parts.push(ct.to_string());
        } else {
            query_parts.push("all".to_string());
        }
        if let Some(t) = type_ {
            query_parts.push(t.to_string());
        } else {
            query_parts.push("all".to_string());
        }
        let comment_path = query_parts.join("/");
        
        let mut query_params = Vec::new();
        if let Some(ir) = include_replies {
            query_params.push(format!("include_replies={}", ir));
        }
        
        let query_string = Self::build_query_string(pagination.as_ref());
        let full_query = if !query_params.is_empty() && !query_string.is_empty() {
            format!("{}?{}", query_string, query_params.join("&"))
        } else if !query_params.is_empty() {
            format!("?{}", query_params.join("&"))
        } else {
            query_string
        };
        
        let endpoint = format!(
            "/users/{}/comments/{}{}",
            username,
            comment_path,
            full_query
        );
        client.get(&endpoint, None).await
    }
    
    // ============ User Notes ============
    
    /// Get user notes (VIP Enhanced).
    pub async fn get_user_notes(
        client: &TraktClient,
        username: &str,
        type_: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let type_part = type_.map(|t| format!("/{}", t)).unwrap_or_default();
        let endpoint = format!(
            "/users/{}/notes{}{}",
            username,
            type_part,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    // ============ Watchlist Comments ============
    
    /// Get watchlist comments.
    pub async fn get_watchlist_comments(
        client: &TraktClient,
        username: &str,
        sort: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let sort_part = sort.map(|s| format!("/{}", s)).unwrap_or_default();
        let endpoint = format!(
            "/users/{}/watchlist/comments{}{}",
            username,
            sort_part,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    // ============ Favorites Comments ============
    
    /// Get favorites comments.
    pub async fn get_favorites_comments(
        client: &TraktClient,
        username: &str,
        sort: Option<&str>,
        pagination: Option<Pagination>,
    ) -> crate::Result<Value> {
        let sort_part = sort.map(|s| format!("/{}", s)).unwrap_or_default();
        let endpoint = format!(
            "/users/{}/favorites/comments{}{}",
            username,
            sort_part,
            Self::build_query_string(pagination.as_ref())
        );
        client.get(&endpoint, None).await
    }
    
    // ============ Reference Data ============
    
    /// Get countries (movies).
    pub async fn get_countries_movies(client: &TraktClient) -> crate::Result<Value> {
        client.get("/countries/movies", None).await
    }
    
    /// Get countries (shows).
    pub async fn get_countries_shows(client: &TraktClient) -> crate::Result<Value> {
        client.get("/countries/shows", None).await
    }
    
    /// Get languages.
    pub async fn get_languages(client: &TraktClient) -> crate::Result<Value> {
        client.get("/languages", None).await
    }
    
    /// Get networks.
    pub async fn get_networks(client: &TraktClient) -> crate::Result<Value> {
        client.get("/networks", None).await
    }
    
    /// Get certifications (movies).
    pub async fn get_certifications_movies(client: &TraktClient) -> crate::Result<Value> {
        client.get("/certifications/movies", None).await
    }
    
    /// Get certifications (shows).
    pub async fn get_certifications_shows(client: &TraktClient) -> crate::Result<Value> {
        client.get("/certifications/shows", None).await
    }
    
    // ============ Last Activity ============
    
    /// Get last activity.
    pub async fn get_last_activity(client: &TraktClient) -> crate::Result<Value> {
        client.get("/last_activity", None).await
    }
    
    // ============ Helper Functions ============
    
    /// Build query string with pagination and extended info.
    fn build_full_query_string(pagination: Option<&Pagination>, extended: Option<&str>) -> String {
        let mut params: Vec<String> = Vec::new();
        
        if let Some(ext) = extended {
            params.push(format!("extended={}", ext));
        }
        
        if let Some(pg) = pagination {
            if let Some(page) = pg.page {
                params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                params.push(format!("limit={}", limit));
            }
        }
        
        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
    
    // Helper function to build query string
    fn build_query_string(pagination: Option<&Pagination>) -> String {
        if let Some(pg) = pagination {
            let mut params = Vec::new();
            if let Some(page) = pg.page {
                params.push(format!("page={}", page));
            }
            if let Some(limit) = pg.limit {
                params.push(format!("limit={}", limit));
            }
            if params.is_empty() {
                String::new()
            } else {
                format!("?{}", params.join("&"))
            }
        } else {
            String::new()
        }
    
    }

    pub async fn get_genres(client: &TraktClient, type_: &str, extended: Option<&str>) -> crate::Result<Value> {
        let mut endpoint = format!("/genres/{}", type_);
        if let Some(ext) = extended {
            endpoint.push_str(&format!("?extended={}", ext));
        }
        client.get(&endpoint, None).await
    }

    pub async fn get_movie_people_extended(client: &TraktClient, id: &str, extended: Option<&str>) -> crate::Result<Value> {
        let mut endpoint = format!("/movies/{}/people", id);
        if let Some(ext) = extended {
            endpoint.push_str(&format!("?extended={}", ext));
        }
        client.get(&endpoint, None).await
    }

    pub async fn get_show_people_extended(client: &TraktClient, id: &str, extended: Option<&str>) -> crate::Result<Value> {
        let mut endpoint = format!("/shows/{}/people", id);
        if let Some(ext) = extended {
            endpoint.push_str(&format!("?extended={}", ext));
        }
        client.get(&endpoint, None).await
    }

    pub async fn add_notes(client: &TraktClient, data: Value) -> crate::Result<Value> {
        client.post("/notes", Some(&data)).await
    }

    pub async fn create_user_list(client: &TraktClient, id: &str, data: Value) -> crate::Result<Value> {
        client.post(&format!("/users/{}/lists", id), Some(&data)).await
    }
    
    pub async fn update_user_list(client: &TraktClient, id: &str, list_id: &str, data: Value) -> crate::Result<Value> {
        client.put(&format!("/users/{}/lists/{}", id, list_id), Some(&data)).await
    }
    
    pub async fn delete_user_list(client: &TraktClient, id: &str, list_id: &str) -> crate::Result<Value> {
        client.delete(&format!("/users/{}/lists/{}", id, list_id)).await
    }

    pub async fn add_user_list_items(client: &TraktClient, id: &str, list_id: &str, data: Value) -> crate::Result<Value> {
        client.post(&format!("/users/{}/lists/{}/items", id, list_id), Some(&data)).await
    }
    
    pub async fn remove_user_list_items(client: &TraktClient, id: &str, list_id: &str, data: Value) -> crate::Result<Value> {
        client.post(&format!("/users/{}/lists/{}/items/remove", id, list_id), Some(&data)).await
    }
    
    pub async fn reorder_user_list_items(client: &TraktClient, id: &str, list_id: &str, data: Value) -> crate::Result<Value> {
        client.post(&format!("/users/{}/lists/{}/items/reorder", id, list_id), Some(&data)).await
    }
    
    pub async fn update_user_list_item(client: &TraktClient, id: &str, list_id: &str, list_item_id: &str, data: Value) -> crate::Result<Value> {
        client.put(&format!("/users/{}/lists/{}/items/{}", id, list_id, list_item_id), Some(&data)).await
    }

    pub async fn get_user_requests_following(client: &TraktClient) -> crate::Result<Value> {
        client.get("/users/requests/following", None).await
    }
    
    pub async fn get_user_requests(client: &TraktClient) -> crate::Result<Value> {
        client.get("/users/requests", None).await
    }
    
    pub async fn approve_user_request(client: &TraktClient, id: &str) -> crate::Result<Value> {
        let empty = serde_json::json!({});
        client.post(&format!("/users/requests/{}", id), Some(&empty)).await
    }
    
    pub async fn deny_user_request(client: &TraktClient, id: &str) -> crate::Result<Value> {
        client.delete(&format!("/users/requests/{}", id)).await
    }

    pub async fn get_hidden_items(client: &TraktClient, section: &str, type_: Option<&str>) -> crate::Result<Value> {
        let mut endpoint = format!("/users/hidden/{}", section);
        if let Some(t) = type_ {
            endpoint.push_str(&format!("?type={}", t));
        }
        client.get(&endpoint, None).await
    }

    pub async fn get_user_list(client: &TraktClient, id: &str, list_id: &str) -> crate::Result<Value> {
        client.get(&format!("/users/{}/lists/{}", id, list_id), None).await
    }

    
    
    pub async fn reorder_lists(client: &TraktClient, id: &str, data: Value) -> crate::Result<Value> {
        client.post(&format!("/users/{}/lists/reorder", id), Some(&data)).await
    }

    pub async fn get_user_favorites(client: &TraktClient, id: &str, type_: Option<&str>, pagination: Option<Pagination>) -> crate::Result<serde_json::Value> {
        let mut endpoint = format!("/users/{}/favorites", id);
        if let Some(t) = type_ {
            endpoint.push_str(&format!("/{}", t));
        }
        endpoint.push_str(&Self::build_query_string(pagination.as_ref()));
        client.get(&endpoint, None).await
    }

    pub async fn reorder_watchlist(client: &TraktClient, data: serde_json::Value) -> crate::Result<serde_json::Value> {
        client.post("/sync/watchlist/reorder", Some(&data)).await
    }
    
    pub async fn reorder_favorites(client: &TraktClient, data: serde_json::Value) -> crate::Result<serde_json::Value> {
        client.post("/sync/favorites/reorder", Some(&data)).await
    }
}
