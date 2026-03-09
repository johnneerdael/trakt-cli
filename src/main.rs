use clap::{Parser, Subcommand};
use anyhow::Result;
use serde_json::Value;

mod config;
mod auth;
mod client;
mod pagination;
mod api;

// Exit codes for AI agent control flow
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;
pub const EXIT_USAGE: i32 = 2;
pub const EXIT_NOT_FOUND: i32 = 3;
pub const EXIT_AUTH_ERROR: i32 = 4;
pub const EXIT_CONFLICT: i32 = 5;

/// Simple Trakt CLI - optimized for AI agents.
#[derive(Parser, Clone)]
#[command(name = "trakt-cli")]
#[command(author = "")]
#[command(version = "0.1.0")]
#[command(about = "Command-line interface for the Trakt API", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Output as JSON (recommended for AI agents)
    #[arg(long, global = true)]
    json: bool,
    
    /// Quiet mode: output bare values only, one per line
    #[arg(short, long, global = true)]
    quiet: bool,
    
    /// Extended info level (none, images, full, full,images)
    #[arg(long, global = true)]
    extended: Option<String>,
    
    /// Page number
    #[arg(long, global = true)]
    page: Option<u32>,
    
    /// Number of items per page (default: 10)
    #[arg(long, global = true)]
    limit: Option<u32>,
}

#[derive(Subcommand, Clone)]
enum Commands {
    /// Configure the CLI with your Trakt API credentials
    /// 
    /// First create an app at https://trakt.tv/oauth/applications/new
    /// Then use: trakt-cli configure --client-id YOUR_ID --client-secret YOUR_SECRET
    Configure {
        /// Trakt API client ID (required)
        #[arg(long)]
        client_id: String,
        
        /// Trakt API client secret (required)
        #[arg(long)]
        client_secret: String,
    },
    
    /// Perform device authentication flow and store tokens
    /// 
    /// This opens a browser for you to authorize the app.
    /// Example: trakt-cli auth
    Auth,
    
    /// Get authenticated user settings
    /// 
    /// Requires authentication. Returns user profile, VIP status, etc.
    /// Example: trakt-cli me --json
    Me,
    
    // ============ Movies ============
    
    /// Get popular movies
    /// 
    /// Example: trakt-cli movies-popular --limit 20 --json
    MoviesPopular,
    
    /// Get trending movies
    /// 
    /// Example: trakt-cli movies-trending --limit 20 --json
    MoviesTrending,
    
    /// Get movie by ID
    /// 
    /// ID can be: trakt slug, trakt ID, IMDB ID (tt0123456), or TMDB ID (12345)
    /// Example: trakt-cli movie "breaking-bad" --extended full --json
    Movie {
        /// Movie ID (slug, trakt ID, IMDB, or TMDB)
        id: String,
    },
    
    // ============ Shows ============
    
    /// Get popular shows
    /// 
    /// Example: trakt-cli shows-popular --limit 20 --json
    ShowsPopular,
    
    /// Get trending shows
    /// 
    /// Example: trakt-cli shows-trending --limit 20 --json
    ShowsTrending,
    
    /// Get show by ID
    /// 
    /// Example: trakt-cli show "breaking-bad" --extended full --json
    Show {
        /// Show ID (slug, trakt ID, IMDB, or TMDB)
        id: String,
    },
    
    /// Get show seasons
    /// 
    /// Example: trakt-cli show-seasons "breaking-bad" --extended full --json
    ShowSeasons {
        /// Show ID
        id: String,
    },
    
    /// Get season episodes
    /// 
    /// Example: trakt-cli season-episodes "breaking-bad" 1 --json
    SeasonEpisodes {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
    },
    
    /// Get episode
    /// 
    /// Example: trakt-cli episode "breaking-bad" 1 1 --json
    Episode {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
        
        /// Episode number
        episode_id: String,
    },
    
    // ============ People ============
    
    /// Get person by ID
    /// 
    /// Example: trakt-cli person "bryan-cranston" --json
    Person {
        /// Person ID (slug or trakt ID)
        id: String,
    },
    
    // ============ Search ============
    
    /// Search for movies, shows, episodes, people, or lists
    /// 
    /// Example: trakt-cli search "Breaking Bad" --type show --json
    Search {
        /// Search query
        query: String,
        
        /// Type: movie, show, episode, person, or list
        #[arg(long)]
        type_: String,
    },
    
    /// Search by external ID
    /// 
    /// Example: trakt-cli search-id imdb tt0903747 --json
    SearchId {
        /// ID type: imdb, tmdb, tvdb, trakt, slug
        id_type: String,
        
        /// ID value
        id: String,
    },
    
    // ============ Users ============
    
    /// Get user profile
    /// 
    /// Example: trakt-cli user sean --json
    User {
        /// Username
        username: String,
    },
    
    /// Get user collection
    /// 
    /// Example: trakt-cli user-collection sean --type movies --json
    UserCollection {
        /// Username
        username: String,
        
        /// Type: movies or shows
        #[arg(long)]
        type_: Option<String>,
    },
    
    /// Get user watchlist
    /// 
    /// Example: trakt-cli user-watchlist sean --type movies --json
    UserWatchlist {
        /// Username
        username: String,
        
        /// Type: movies or shows
        #[arg(long)]
        type_: Option<String>,
    },
    
    /// Get user history
    /// 
    /// Example: trakt-cli user-history sean --type movies --json
    UserHistory {
        /// Username
        username: String,
        
        /// Type: movies, shows, or episodes
        #[arg(long)]
        type_: Option<String>,
    },
    
    /// Get user ratings
    /// 
    /// Example: trakt-cli user-ratings sean --type movies --json
    UserRatings {
        /// Username
        username: String,
        
        /// Type: movies, shows, episodes, or all
        #[arg(long)]
        type_: Option<String>,
    },
    
    // ============ Calendars ============
    
    /// Get user's movie calendar
    /// 
    /// Example: trakt-cli calendar-movies --days 7 --json
    CalendarMovies {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get user's show calendar
    /// 
    /// Example: trakt-cli calendar-shows --days 7 --json
    CalendarShows {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get user's show premieres calendar
    /// 
    /// Example: trakt-cli calendar-shows-premieres --days 7 --json
    CalendarShowsPremieres {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get new shows calendar (shows user hasn't seen yet)
    /// 
    /// Example: trakt-cli calendar-new-shows --days 7 --json
    CalendarNewShows {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get user's show finales calendar (OAuth required)
    /// 
    /// Example: trakt-cli calendar-shows-finales --days 7 --json
    CalendarShowsFinales {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get user's streaming releases calendar (OAuth required)
    /// 
    /// Example: trakt-cli calendar-streaming --days 7 --json
    CalendarStreaming {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get user's DVD releases calendar (OAuth required)
    /// 
    /// Example: trakt-cli calendar-dvd --days 7 --json
    CalendarDvd {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    // ============ All Calendars (public) ============
    
    /// Get all shows calendar
    /// 
    /// Example: trakt-cli all-calendar-shows --days 7 --json
    AllCalendarShows {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get all new shows calendar
    /// 
    /// Example: trakt-cli all-calendar-new-shows --days 7 --json
    AllCalendarNewShows {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get all season premieres calendar
    /// 
    /// Example: trakt-cli all-calendar-premieres --days 7 --json
    AllCalendarPremieres {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get all season finales calendar
    /// 
    /// Example: trakt-cli all-calendar-finales --days 7 --json
    AllCalendarFinales {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get all movies calendar
    /// 
    /// Example: trakt-cli all-calendar-movies --days 7 --json
    AllCalendarMovies {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get all streaming releases calendar
    /// 
    /// Example: trakt-cli all-calendar-streaming --days 7 --json
    AllCalendarStreaming {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    /// Get all DVD releases calendar
    /// 
    /// Example: trakt-cli all-calendar-dvd --days 7 --json
    AllCalendarDvd {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Number of days
        #[arg(long)]
        days: Option<u32>,
    },
    
    // ============ Genres ============
    
    /// Get movie genres
    /// 
    /// Example: trakt-cli genres-movies --json
    GenresMovies,
    
    /// Get show genres
    /// 
    /// Example: trakt-cli genres-shows --json
    GenresShows,
    
    // ============ Lists ============
    
    /// Get list details
    /// 
    /// Example: trakt-cli list 12345 --json
    List {
        /// List ID
        id: String,
    },
    
    /// Get list items
    /// 
    /// Example: trakt-cli list-items 12345 --json
    ListItems {
        /// List ID
        id: String,
    },
    
    // ============ Comments ============
    
    /// Get comment
    /// 
    /// Example: trakt-cli comment 12345 --json
    Comment {
        /// Comment ID
        id: String,
    },
    
    // ============ Updates ============
    
    /// Get recently updated movies
    /// 
    /// Example: trakt-cli updated-movies --json
    UpdatedMovies {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
    },
    
    /// Get recently updated shows
    /// 
    /// Example: trakt-cli updated-shows --json
    UpdatedShows {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
    },
    
    // ============ Movies (additional) ============
    
    /// Get box office movies
    /// 
    /// Example: trakt-cli movies-boxoffice --json
    MoviesBoxoffice,
    
    /// Get most played movies
    /// 
    /// Example: trakt-cli movies-played --limit 20 --json
    MoviesPlayed,
    
    /// Get most watched movies
    /// 
    /// Example: trakt-cli movies-watched --limit 20 --json
    MoviesWatched,
    
    /// Get most collected movies
    /// 
    /// Example: trakt-cli movies-collected --limit 20 --json
    MoviesCollected,
    
    /// Get most anticipated movies
    /// 
    /// Example: trakt-cli movies-anticipated --limit 20 --json
    MoviesAnticipated,
    
    /// Get most favorited movies
    /// 
    /// Example: trakt-cli movies-favorited --limit 20 --json
    MoviesFavorited,
    
    /// Get DVD releases
    /// 
    /// Example: trakt-cli movies-dvd-releases --json
    MoviesDvdReleases,
    
    /// Get streaming releases
    /// 
    /// Example: trakt-cli movies-streaming-releases --json
    MoviesStreamingReleases,
    
    /// Get movie studios
    /// 
    /// Example: trakt-cli movie-studios "batman-begins-2005" --json
    MovieStudios {
        /// Movie ID
        id: String,
    },
    
    /// Get users watching movie
    /// 
    /// Example: trakt-cli movie-watching "batman-begins-2005" --json
    MovieWatching {
        /// Movie ID
        id: String,
    },
    
    /// Get movie videos
    /// 
    /// Example: trakt-cli movie-videos "batman-begins-2005" --json
    MovieVideos {
        /// Movie ID
        id: String,
    },
    
    // ============ Shows (additional) ============
    
    /// Get most played shows
    /// 
    /// Example: trakt-cli shows-played --limit 20 --json
    ShowsPlayed,
    
    /// Get most watched shows
    /// 
    /// Example: trakt-cli shows-watched --limit 20 --json
    ShowsWatched,
    
    /// Get most collected shows
    /// 
    /// Example: trakt-cli shows-collected --limit 20 --json
    ShowsCollected,
    
    /// Get most anticipated shows
    /// 
    /// Example: trakt-cli shows-anticipated --limit 20 --json
    ShowsAnticipated,
    
    /// Get most favorited shows
    /// 
    /// Example: trakt-cli shows-favorited --limit 20 --json
    ShowsFavorited,
    
    /// Get show certifications
    /// 
    /// Example: trakt-cli show-certifications "breaking-bad" --json
    ShowCertifications {
        /// Show ID
        id: String,
    },
    
    /// Get show collection progress
    /// 
    /// Example: trakt-cli show-collection-progress "breaking-bad" --json
    ShowCollectionProgress {
        /// Show ID
        id: String,
    },
    
    /// Get show watched progress
    /// 
    /// Example: trakt-cli show-watched-progress "breaking-bad" --json
    ShowWatchedProgress {
        /// Show ID
        id: String,
    },
    
    /// Reset show progress (OAuth required)
    /// 
    /// Example: trakt-cli show-progress-reset "breaking-bad" --json
    ShowProgressReset {
        /// Show ID
        id: String,
    },
    
    /// Get show studios
    /// 
    /// Example: trakt-cli show-studios "breaking-bad" --json
    ShowStudios {
        /// Show ID
        id: String,
    },
    
    /// Get users watching show
    /// 
    /// Example: trakt-cli show-watching "breaking-bad" --json
    ShowWatching {
        /// Show ID
        id: String,
    },
    
    /// Get show next episode
    /// 
    /// Example: trakt-cli show-next-episode "breaking-bad" --json
    ShowNextEpisode {
        /// Show ID
        id: String,
    },
    
    /// Get show last episode
    /// 
    /// Example: trakt-cli show-last-episode "breaking-bad" --json
    ShowLastEpisode {
        /// Show ID
        id: String,
    },
    
    /// Get show videos
    /// 
    /// Example: trakt-cli show-videos "breaking-bad" --json
    ShowVideos {
        /// Show ID
        id: String,
    },
    
    // ============ Seasons ============
    
    /// Get season details
    /// 
    /// Example: trakt-cli season "breaking-bad" 1 --extended full --json
    Season {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
    },
    
    /// Get season translations
    /// 
    /// Example: trakt-cli season-translations "breaking-bad" 1 --json
    SeasonTranslations {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
    },
    
    /// Get season comments
    /// 
    /// Example: trakt-cli season-comments "breaking-bad" 1 --json
    SeasonComments {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
    },
    
    /// Get season people
    /// 
    /// Example: trakt-cli season-people "breaking-bad" 1 --json
    SeasonPeople {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
    },
    
    /// Get season ratings
    /// 
    /// Example: trakt-cli season-ratings "breaking-bad" 1 --json
    SeasonRatings {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
    },
    
    /// Get season stats
    /// 
    /// Example: trakt-cli season-stats "breaking-bad" 1 --json
    SeasonStats {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
    },
    
    // ============ Episodes (additional) ============
    
    /// Get episode translations
    /// 
    /// Example: trakt-cli episode-translations "breaking-bad" 1 1 --json
    EpisodeTranslations {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
        
        /// Episode number
        episode_id: String,
    },
    
    /// Get episode comments
    /// 
    /// Example: trakt-cli episode-comments "breaking-bad" 1 1 --json
    EpisodeComments {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
        
        /// Episode number
        episode_id: String,
    },
    
    /// Get episode people
    /// 
    /// Example: trakt-cli episode-people "breaking-bad" 1 1 --json
    EpisodePeople {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
        
        /// Episode number
        episode_id: String,
    },
    
    /// Get episode ratings
    /// 
    /// Example: trakt-cli episode-ratings "breaking-bad" 1 1 --json
    EpisodeRatings {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
        
        /// Episode number
        episode_id: String,
    },
    
    /// Get episode stats
    /// 
    /// Example: trakt-cli episode-stats "breaking-bad" 1 1 --json
    EpisodeStats {
        /// Show ID
        show_id: String,
        
        /// Season number
        season_id: String,
        
        /// Episode number
        episode_id: String,
    },
    
    // ============ People (additional) ============
    
    /// Get person movie credits
    /// 
    /// Example: trakt-cli person-movies "bryan-cranston" --json
    PersonMovies {
        /// Person ID
        id: String,
    },
    
    /// Get person show credits
    /// 
    /// Example: trakt-cli person-shows "bryan-cranston" --json
    PersonShows {
        /// Person ID
        id: String,
    },
    
    /// Get person lists
    /// 
    /// Example: trakt-cli person-lists "bryan-cranston" --json
    PersonLists {
        /// Person ID
        id: String,
    },
    
    // ============ Users (additional) =========
    
    /// Get user likes
    /// 
    /// Example: trakt-cli user-likes sean --json
    UserLikes {
        /// Username
        username: String,
    },
    
    /// Get user followers
    /// 
    /// Example: trakt-cli user-followers sean --json
    UserFollowers {
        /// Username
        username: String,
    },
    
    /// Get user following
    /// 
    /// Example: trakt-cli user-following sean --json
    UserFollowing {
        /// Username
        username: String,
    },
    
    /// Get user friends
    /// 
    /// Example: trakt-cli user-friends sean --json
    UserFriends {
        /// Username
        username: String,
    },
    
    /// Get user watching now
    /// 
    /// Example: trakt-cli user-watching sean --json
    UserWatchingNow {
        /// Username
        username: String,
    },
    
    /// Get user watched
    /// 
    /// Example: trakt-cli user-watched sean movies --json
    UserWatched {
        /// Username
        username: String,
        
        /// Type: movies or shows
        #[arg(long)]
        type_: String,
    },
    
    /// Get user stats
    /// 
    /// Example: trakt-cli user-stats sean --json
    UserStats {
        /// Username
        username: String,
    },
    
    /// Get user hidden items
    /// 
    /// Example: trakt-cli user-hidden sean --json
    UserHidden {
        /// Username
        username: String,
        
        /// Type: movies or shows
        #[arg(long)]
        type_: Option<String>,
    },
    
    // ============ Sync (OAuth required) ============
    
    /// Get sync collection
    /// 
    /// Example: trakt-cli sync-collection --type movies --json
    SyncCollection {
        /// Type: movies or shows
        #[arg(long)]
        type_: Option<String>,
    },
    
    /// Get sync watched
    /// 
    /// Example: trakt-cli sync-watched movies --json
    SyncWatched {
        /// Type: movies or shows
        type_: String,
    },
    
    /// Get sync history
    /// 
    /// Example: trakt-cli sync-history --type movies --json
    SyncHistory {
        /// Type: movies, shows, or episodes
        #[arg(long)]
        type_: Option<String>,
    },
    
    /// Get sync ratings
    /// 
    /// Example: trakt-cli sync-ratings --type movies --json
    SyncRatings {
        /// Type: movies, shows, episodes, or all
        #[arg(long)]
        type_: Option<String>,
    },
    
    /// Get sync watchlist
    /// 
    /// Example: trakt-cli sync-watchlist --type movies --json
    SyncWatchlist {
        /// Type: movies or shows
        #[arg(long)]
        type_: Option<String>,
    },
    
    /// Get sync favorites
    /// 
    /// Example: trakt-cli sync-favorites --type movies --json
    SyncFavorites {
        /// Type: movies or shows
        #[arg(long)]
        type_: Option<String>,
    },
    
    // ============ Scrobble / Checkin ============
    
    /// Start scrobbling
    /// 
    /// Example: trakt-cli scrobble-start --movie-id 1 --progress 50 --json
    ScrobbleStart {
        /// Movie or show ID
        #[arg(long)]
        movie_id: Option<String>,
        
        /// Show ID (for episodes)
        #[arg(long)]
        show_id: Option<String>,
        
        /// Season number (for episodes)
        #[arg(long)]
        season: Option<String>,
        
        /// Episode number (for episodes)
        #[arg(long)]
        episode: Option<String>,
        
        /// Progress (0-100)
        #[arg(long)]
        progress: f64,
    },
    
    /// Stop scrobbling
    /// 
    /// Example: trakt-cli scrobble-stop --movie-id 1 --progress 100 --json
    ScrobbleStop {
        /// Movie or show ID
        #[arg(long)]
        movie_id: Option<String>,
        
        /// Show ID (for episodes)
        #[arg(long)]
        show_id: Option<String>,
        
        /// Season number (for episodes)
        #[arg(long)]
        season: Option<String>,
        
        /// Episode number (for episodes)
        #[arg(long)]
        episode: Option<String>,
        
        /// Progress (0-100)
        #[arg(long)]
        progress: f64,
    },
    
    /// Check into an item
    /// 
    /// Example: trakt-cli checkin --show-id 1 --season 1 --episode 1 --json
    Checkin {
        /// Movie ID
        #[arg(long)]
        movie_id: Option<String>,
        
        /// Show ID (for episodes)
        #[arg(long)]
        show_id: Option<String>,
        
        /// Season number (for episodes)
        #[arg(long)]
        season: Option<String>,
        
        /// Episode number (for episodes)
        #[arg(long)]
        episode: Option<String>,
    },
    
    /// Delete checkin
    /// 
    /// Example: trakt-cli checkin-delete --json
    CheckinDelete,
    
    /// Get playback progress
    /// 
    /// Example: trakt-cli playback-progress --json
    PlaybackProgress {
        /// Type: movies or shows
        #[arg(long)]
        type_: Option<String>,
    },
    
    // ============ Comments ============
    
    /// Get trending comments
    /// 
    /// Example: trakt-cli comments-trending --json
    CommentsTrending {
        /// Comment type: all, reviews, or shouts
        #[arg(long)]
        comment_type: Option<String>,
        /// Type: all, movies, shows, seasons, episodes, or lists
        #[arg(long)]
        type_: Option<String>,
        /// Include comment replies
        #[arg(long)]
        include_replies: Option<bool>,
        /// Page number
        #[arg(long)]
        page: Option<u32>,
        /// Results per page
        #[arg(long)]
        limit: Option<u32>,
    },
    
    /// Get recently created comments
    /// 
    /// Example: trakt-cli comments-recent --json
    CommentsRecent,
    
    /// Get recently updated comments
    /// 
    /// Example: trakt-cli comments-updated --json
    CommentsUpdated,
    
    /// Post a comment
    /// 
    /// Example: trakt-cli comment-post --type movies --id 1 --comment "Great movie!" --json
    CommentPost {
        /// Type: movies, shows, seasons, episodes, or lists
        #[arg(long)]
        type_: String,
        
        /// ID
        #[arg(long)]
        id: String,
        
        /// Comment text
        #[arg(long)]
        comment: String,
        
        /// Spoiler
        #[arg(long)]
        spoiler: Option<bool>,
    },
    
    /// Like a comment
    /// 
    /// Example: trakt-cli comment-like 12345 --json
    CommentLike {
        /// Comment ID
        id: String,
    },
    
    /// Unlike a comment
    /// 
    /// Example: trakt-cli comment-unlike 12345 --json
    CommentUnlike {
        /// Comment ID
        id: String,
    },
    
    // ============ Lists ============
    
    /// Get trending lists
    /// 
    /// Example: trakt-cli lists-trending --json
    ListsTrending,
    
    /// Get popular lists
    /// 
    /// Example: trakt-cli lists-popular --json
    ListsPopular,
    
    /// Like a list
    /// 
    /// Example: trakt-cli list-like 12345 --json
    ListLike {
        /// List ID
        id: String,
    },
    
    /// Unlike a list
    /// 
    /// Example: trakt-cli list-unlike 12345 --json
    ListUnlike {
        /// List ID
        id: String,
    },
    
    /// Create a list
    /// 
    /// Example: trakt-cli list-create --name "My List" --json
    ListCreate {
        /// List name
        #[arg(long)]
        name: String,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
        
        /// Privacy: public, private, or friends
        #[arg(long)]
        privacy: Option<String>,
    },
    
    /// Update a list
    /// 
    /// Example: trakt-cli list-update 12345 --name "New Name" --json
    ListUpdate {
        /// List ID
        id: String,
        
        /// New name
        #[arg(long)]
        name: Option<String>,
        
        /// New description
        #[arg(long)]
        description: Option<String>,
        
        /// Privacy: public, private, or friends
        #[arg(long)]
        privacy: Option<String>,
    },
    
    /// Delete a list
    /// 
    /// Example: trakt-cli list-delete 12345 --json
    ListDelete {
        /// List ID
        id: String,
    },
    
    /// Add items to list
    /// 
    /// Example: trakt-cli list-add-items 12345 --items '[{"type": "movie", "id": 1}]' --json
    ListAddItems {
        /// List ID
        id: String,
        
        /// JSON items array
        #[arg(long)]
        items: String,
    },
    
    /// Remove items from list
    /// 
    /// Example: trakt-cli list-remove-items 12345 --items '[{"type": "movie", "id": 1}]' --json
    ListRemoveItems {
        /// List ID
        id: String,
        
        /// JSON items array
        #[arg(long)]
        items: String,
    },
    
    // ============ Reference Data ============
    
    /// Get countries (movies)
    /// 
    /// Example: trakt-cli countries-movies --json
    CountriesMovies,
    
    /// Get countries (shows)
    /// 
    /// Example: trakt-cli countries-shows --json
    CountriesShows,
    
    /// Get languages
    /// 
    /// Example: trakt-cli languages --json
    Languages,
    
    /// Get networks
    /// 
    /// Example: trakt-cli networks --json
    Networks,
    
    /// Get certifications (movies)
    /// 
    /// Example: trakt-cli certifications-movies --json
    CertificationsMovies,
    
    /// Get certifications (shows)
    /// 
    /// Example: trakt-cli certifications-shows --json
    CertificationsShows,
    
    // ============ Last Activity ============
    
    /// Get last activity
    /// 
    /// Example: trakt-cli last-activity --json
    LastActivity,
    
    // ============ Recommendations ============
    
    /// Get movie recommendations
    /// 
    /// Example: trakt-cli recommendations-movies --json
    RecommendationsMovies,
    
    /// Get show recommendations
    /// 
    /// Example: trakt-cli recommendations-shows --json
    RecommendationsShows,
    
    // ============ Updates (additional) ============
    
    /// Get recently updated people
    /// 
    /// Example: trakt-cli updated-people --json
    UpdatedPeople,
    
    /// Get recently updated people IDs
    /// 
    /// Example: trakt-cli updated-people-ids --json
    UpdatedPeopleIds,
}

fn output_result(result: Result<Value>, json: bool, quiet: bool) -> i32 {
    match result {
        Ok(value) => {
            if json {
                // JSON to stdout
                println!("{}", serde_json::to_string(&value).unwrap_or_default());
            } else if quiet {
                // Quiet mode: just output values
                if let Some(arr) = value.as_array() {
                    for item in arr {
                        if let Some(obj) = item.as_object() {
                            // Try to get common display fields
                            if let Some(title) = obj.get("title").and_then(|v| v.as_str()) {
                                println!("{}", title);
                            } else if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
                                println!("{}", name);
                            } else if let Some(id) = obj.get("ids").and_then(|v| v.as_object()) {
                                if let Some(trakt) = id.get("trakt") {
                                    println!("{}", trakt);
                                } else if let Some(slug) = id.get("slug") {
                                    println!("{}", slug);
                                }
                            } else {
                                println!("{}", item);
                            }
                        } else {
                            println!("{}", item);
                        }
                    }
                } else {
                    println!("{}", value);
                }
            } else {
                // Pretty print to stdout
                println!("{}", serde_json::to_string_pretty(&value).unwrap_or_default());
            }
            EXIT_SUCCESS
        }
        Err(e) => {
            if json {
                let error_json = serde_json::json!({
                    "error": "failure",
                    "message": e.to_string()
                });
                eprintln!("{}", serde_json::to_string(&error_json).unwrap_or_default());
            } else {
                eprintln!("Error: {}", e);
            }
            EXIT_FAILURE
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    let exit_code = match run_cli(cli.clone()).await {
        Ok(code) => code,
        Err(e) => {
            if cli.json {
                let error_json = serde_json::json!({
                    "error": "failure",
                    "message": e.to_string()
                });
                eprintln!("{}", serde_json::to_string(&error_json).unwrap_or_default());
            } else {
                eprintln!("Error: {}", e);
            }
            EXIT_FAILURE
        }
    };
    
    std::process::exit(exit_code);
}

async fn run_cli(cli: Cli) -> Result<i32> {
    match cli.command {
        Commands::Configure { client_id, client_secret } => {
            let mut cfg = config::Config::load()?;
            cfg.client_id = client_id;
            cfg.client_secret = client_secret;
            cfg.save()?;
            
            if cli.json {
                println!("{}", serde_json::json!({"status": "success", "message": "Configuration saved"}).to_string());
            } else if cli.quiet {
                println!("saved");
            } else {
                println!("Configuration saved successfully!");
            }
            Ok(EXIT_SUCCESS)
        }
        
        Commands::Auth => {
            let mut cfg = config::Config::load()?;
            auth::device_auth(&mut cfg).await?;
            
            if cli.json {
                println!("{}", serde_json::json!({"status": "success", "message": "Authentication successful"}).to_string());
            } else if cli.quiet {
                println!("authenticated");
            } else {
                println!("Authentication successful!");
            }
            Ok(EXIT_SUCCESS)
        }
        
        Commands::Me => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                if cli.json {
                    eprintln!("{}", serde_json::json!({"error": "auth_required", "message": "Not authenticated. Run 'trakt-cli auth' first."}).to_string());
                } else {
                    eprintln!("Error: Not authenticated. Run 'trakt-cli auth' first.");
                }
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_user_settings(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Movies
        Commands::MoviesPopular => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_popular_movies(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesTrending => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_trending_movies(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::Movie { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_movie(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Shows
        Commands::ShowsPopular => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_popular_shows(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowsTrending => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_trending_shows(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::Show { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowSeasons { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_seasons(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SeasonEpisodes { show_id, season_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_season_episodes(&client, &show_id, &season_id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::Episode { show_id, season_id, episode_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_episode(&client, &show_id, &season_id, &episode_id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // People
        Commands::Person { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_person(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Search
        Commands::Search { query, type_ } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::search(&client, &query, &type_, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SearchId { id_type, id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::search_id(&client, &id_type, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Users
        Commands::User { username } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_user_profile(&client, &username).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserCollection { username, type_ } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let type_str = type_.unwrap_or_else(|| "movies".to_string());
            let result = if type_str == "movies" {
                api::Api::get_user_collection_movies(&client, &username, cli.extended.as_deref()).await?
            } else {
                api::Api::get_user_collection_shows(&client, &username, cli.extended.as_deref()).await?
            };
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserWatchlist { username, type_ } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let type_str = type_.unwrap_or_else(|| "movies".to_string());
            let result = if type_str == "movies" {
                api::Api::get_user_watchlist_movies(&client, &username, Some(pagination.clone())).await?
            } else {
                api::Api::get_user_watchlist_shows(&client, &username, Some(pagination)).await?
            };
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserHistory { username, type_ } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let type_str = type_.unwrap_or_else(|| "movies".to_string());
            let result = match type_str.as_str() {
                "movies" => api::Api::get_user_history_movies(&client, &username, Some(pagination.clone())).await?,
                "shows" => api::Api::get_user_history_shows(&client, &username, Some(pagination.clone())).await?,
                _ => api::Api::get_user_history_episodes(&client, &username, Some(pagination)).await?,
            };
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserRatings { username, type_ } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_user_ratings(&client, &username, type_.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Calendars
        Commands::CalendarMovies { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_calendar_movies(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CalendarShows { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_calendar_shows(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CalendarShowsPremieres { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_calendar_shows_premieres(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CalendarNewShows { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_calendar_new_shows(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CalendarShowsFinales { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_calendar_shows_finales(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CalendarStreaming { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_calendar_streaming(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CalendarDvd { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_calendar_dvd(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // All Calendars (public)
        Commands::AllCalendarShows { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_all_shows_calendar(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::AllCalendarNewShows { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_all_new_shows_calendar(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::AllCalendarPremieres { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_all_shows_premieres_calendar(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::AllCalendarFinales { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_all_shows_finales_calendar(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::AllCalendarMovies { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_all_movies_calendar(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::AllCalendarStreaming { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_all_streaming_calendar(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::AllCalendarDvd { start_date, days } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_all_dvd_calendar(&client, start_date.as_deref(), days).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Genres
        Commands::GenresMovies => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_movie_genres(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::GenresShows => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_genres(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Lists
        Commands::List { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_list(&client, &id, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListItems { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_list_items(&client, &id, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Comments
        Commands::Comment { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_comment(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Updates
        Commands::UpdatedMovies { start_date } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(100));
            let result = api::Api::get_updated_movies(&client, start_date.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UpdatedShows { start_date } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(100));
            let result = api::Api::get_updated_shows(&client, start_date.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Movies (additional)
        Commands::MoviesBoxoffice => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_movies_boxoffice(&client, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesPlayed => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_movies_played(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesWatched => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_movies_watched(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesCollected => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_movies_collected(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesAnticipated => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_movies_anticipated(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesFavorited => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_movies_favorited(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesDvdReleases => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_movies_dvd_releases(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MoviesStreamingReleases => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_movies_streaming_releases(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MovieStudios { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_movie_studios(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MovieWatching { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_movie_watching(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::MovieVideos { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_movie_videos(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Shows (additional)
        Commands::ShowsPlayed => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_shows_played(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowsWatched => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_shows_watched(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowsCollected => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_shows_collected(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowsAnticipated => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_shows_anticipated(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowsFavorited => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_shows_favorited(&client, Some(pagination), cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowCertifications { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_certifications(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowCollectionProgress { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_collection_progress(&client, &id, None, None).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowWatchedProgress { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_watched_progress(&client, &id, None, None).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowProgressReset { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::reset_show_progress(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowStudios { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_studios(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowWatching { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_watching(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowNextEpisode { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_next_episode(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowLastEpisode { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_last_episode(&client, &id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ShowVideos { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_videos(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Seasons
        Commands::Season { show_id, season_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_season(&client, &show_id, &season_id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SeasonTranslations { show_id, season_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_season_translations(&client, &show_id, &season_id, None).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SeasonComments { show_id, season_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_season_comments(&client, &show_id, &season_id, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SeasonPeople { show_id, season_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_season_people(&client, &show_id, &season_id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SeasonRatings { show_id, season_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_season_ratings(&client, &show_id, &season_id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SeasonStats { show_id, season_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_season_stats(&client, &show_id, &season_id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Episodes (additional)
        Commands::EpisodeTranslations { show_id, season_id, episode_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_episode_translations(&client, &show_id, &season_id, &episode_id, None).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::EpisodeComments { show_id, season_id, episode_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_episode_comments(&client, &show_id, &season_id, &episode_id, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::EpisodePeople { show_id, season_id, episode_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_episode_people(&client, &show_id, &season_id, &episode_id, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::EpisodeRatings { show_id, season_id, episode_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_episode_ratings(&client, &show_id, &season_id, &episode_id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::EpisodeStats { show_id, season_id, episode_id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_episode_stats(&client, &show_id, &season_id, &episode_id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // People (additional)
        Commands::PersonMovies { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_person_movies(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::PersonShows { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_person_shows(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::PersonLists { id } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_person_lists(&client, &id, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Users (additional)
        Commands::UserLikes { username } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_user_likes(&client, &username, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserFollowers { username } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_user_followers(&client, &username, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserFollowing { username } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_user_following(&client, &username, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserFriends { username } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_user_friends(&client, &username, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserWatchingNow { username } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_user_watching(&client, &username, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserWatched { username, type_ } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_user_watched(&client, &username, &type_, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserStats { username } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_user_stats(&client, &username).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UserHidden { username, type_ } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_user_hidden(&client, &username, type_.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Sync
        Commands::SyncCollection { type_ } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = if type_.as_deref() == Some(&"shows".to_string()) {
                api::Api::getSync_collection_shows(&client, cli.extended.as_deref()).await?
            } else {
                api::Api::get_sync_collection_movies(&client, cli.extended.as_deref()).await?
            };
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SyncWatched { type_ } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_sync_watched(&client, &type_, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SyncHistory { type_ } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_sync_history(&client, type_.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SyncRatings { type_ } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_sync_ratings(&client, type_.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SyncWatchlist { type_ } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_sync_watchlist(&client, type_.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::SyncFavorites { type_ } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_sync_favorites(&client, type_.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Scrobble / Checkin
        Commands::ScrobbleStart { movie_id, show_id, season, episode, progress } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let mut data = serde_json::json!({ "progress": progress });
            if let Some(mid) = movie_id {
                data["movie"] = serde_json::json!({ "ids": { "trakt": mid.parse::<u64>().unwrap_or(0) }});
            } else if let (Some(sid), Some(s), Some(e)) = (&show_id, &season, &episode) {
                data["show"] = serde_json::json!({ "ids": { "trakt": sid.parse::<u64>().unwrap_or(0) }});
                data["episode"] = serde_json::json!({ "season": s.parse::<u32>().unwrap_or(0), "number": e.parse::<u32>().unwrap_or(0) });
            }
            let result = api::Api::scrobble_start(&client, data).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ScrobbleStop { movie_id, show_id, season, episode, progress } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let mut data = serde_json::json!({ "progress": progress });
            if let Some(mid) = movie_id {
                data["movie"] = serde_json::json!({ "ids": { "trakt": mid.parse::<u64>().unwrap_or(0) }});
            } else if let (Some(sid), Some(s), Some(e)) = (&show_id, &season, &episode) {
                data["show"] = serde_json::json!({ "ids": { "trakt": sid.parse::<u64>().unwrap_or(0) }});
                data["episode"] = serde_json::json!({ "season": s.parse::<u32>().unwrap_or(0), "number": e.parse::<u32>().unwrap_or(0) });
            }
            let result = api::Api::scrobble_stop(&client, data).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::Checkin { movie_id, show_id, season, episode } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let mut data = serde_json::json!({});
            if let Some(mid) = movie_id {
                data["movie"] = serde_json::json!({ "ids": { "trakt": mid.parse::<u64>().unwrap_or(0) }});
            } else if let (Some(sid), Some(s), Some(e)) = (&show_id, &season, &episode) {
                data["show"] = serde_json::json!({ "ids": { "trakt": sid.parse::<u64>().unwrap_or(0) }});
                data["episode"] = serde_json::json!({ "season": s.parse::<u32>().unwrap_or(0), "number": e.parse::<u32>().unwrap_or(0) });
            }
            let result = api::Api::checkin(&client, data).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CheckinDelete => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::delete_checkin(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::PlaybackProgress { type_ } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_playback_progress(&client, type_.as_deref(), Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Comments
        Commands::CommentsTrending { comment_type, type_, include_replies, page, limit } => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(page.unwrap_or(cli.page.unwrap_or(1)))
                .with_limit(limit.unwrap_or(cli.limit.unwrap_or(10)));
            let result = api::Api::get_trending_comments(
                &client,
                comment_type.as_deref(),
                type_.as_deref(),
                include_replies,
                Some(pagination)
            ).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CommentsRecent => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_recently_created_comments(&client, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CommentsUpdated => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_recently_updated_comments(&client, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CommentPost { type_, id, comment, spoiler } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::post_comment(&client, &type_, &id, &comment, spoiler).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CommentLike { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::like_comment(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CommentUnlike { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::unlike_comment(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Lists
        Commands::ListsTrending => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_trending_lists(&client, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListsPopular => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(10));
            let result = api::Api::get_popular_lists(&client, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListLike { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::like_list(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListUnlike { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::unlike_list(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListCreate { name, description, privacy } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::create_list(&client, &name, description.as_deref(), privacy.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListUpdate { id, name, description, privacy } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::update_list(&client, &id, name.as_deref(), description.as_deref(), privacy.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListDelete { id } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::delete_list(&client, &id).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListAddItems { id, items } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let items_data: Value = serde_json::from_str(&items).unwrap_or(serde_json::json!([]));
            let result = api::Api::add_list_items(&client, &id, items_data).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::ListRemoveItems { id, items } => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let items_data: Value = serde_json::from_str(&items).unwrap_or(serde_json::json!([]));
            let result = api::Api::remove_list_items(&client, &id, items_data).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Reference Data
        Commands::CountriesMovies => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_countries_movies(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CountriesShows => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_countries_shows(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::Languages => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_languages(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::Networks => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_networks(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CertificationsMovies => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_certifications_movies(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::CertificationsShows => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_certifications_shows(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Last Activity
        Commands::LastActivity => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_last_activity(&client).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Recommendations
        Commands::RecommendationsMovies => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_movie_recommendations(&client, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::RecommendationsShows => {
            let cfg = config::Config::load()?;
            if cfg.access_token.is_none() {
                return Ok(EXIT_AUTH_ERROR);
            }
            let client = client::TraktClient::new(cfg).await?;
            let result = api::Api::get_show_recommendations(&client, cli.extended.as_deref()).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        // Updates (additional)
        Commands::UpdatedPeople => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(100));
            let result = api::Api::get_updated_people(&client, None, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
        
        Commands::UpdatedPeopleIds => {
            let cfg = config::Config::load()?;
            let client = client::TraktClient::new(cfg).await?;
            let pagination = pagination::Pagination::new()
                .with_page(cli.page.unwrap_or(1))
                .with_limit(cli.limit.unwrap_or(100));
            let result = api::Api::get_updated_people_ids(&client, None, Some(pagination)).await?;
            Ok(output_result(Ok(result), cli.json, cli.quiet))
        }
    }
}
