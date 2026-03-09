use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(name = "trakt-cli")]
#[command(author = "")]
#[command(version = "0.1.0")]
#[command(about = "Command-line interface for the Trakt API", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long, global = true)]
    pub json: bool,

    #[arg(short, long, global = true)]
    pub quiet: bool,

    #[arg(long, global = true)]
    pub extended: Option<String>,

    #[arg(long, global = true)]
    pub page: Option<u32>,

    #[arg(long, global = true)]
    pub limit: Option<u32>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Configure {
        #[arg(long)]
        client_id: String,
        #[arg(long)]
        client_secret: String,
    },
    Auth,
    Me,

    #[command(subcommand)]
    Calendars(CalendarsCommands),
    #[command(subcommand)]
    Checkin(CheckinCommands),
    #[command(subcommand)]
    Certifications(CertificationsCommands),
    #[command(subcommand)]
    Comments(CommentsCommands),
    #[command(subcommand)]
    Countries(CountriesCommands),
    #[command(subcommand)]
    Genres(GenresCommands),
    #[command(subcommand)]
    Languages(LanguagesCommands),
    #[command(subcommand)]
    Lists(ListsCommands),
    #[command(subcommand)]
    Movies(MoviesCommands),
    #[command(subcommand)]
    Networks(NetworksCommands),
    #[command(subcommand)]
    Notes(NotesCommands),
    #[command(subcommand)]
    People(PeopleCommands),
    #[command(subcommand)]
    Recommendations(RecommendationsCommands),
    #[command(subcommand)]
    Scrobble(ScrobbleCommands),
    #[command(subcommand)]
    Search(SearchCommands),
    #[command(subcommand)]
    Shows(ShowsCommands),
    #[command(subcommand)]
    Seasons(SeasonsCommands),
    #[command(subcommand)]
    Episodes(EpisodesCommands),
    #[command(subcommand)]
    Sync(SyncCommands),
    #[command(subcommand)]
    Users(UsersCommands),
}

#[derive(Subcommand, Clone)]
pub enum CalendarsCommands {
    MyShows { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    MyNewShows { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    MySeasonPremieres { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    MyFinales { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    MyMovies { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    MyStreaming { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    MyDvd { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    AllShows { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    AllNewShows { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    AllSeasonPremieres { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    AllFinales { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    AllMovies { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    AllStreaming { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
    AllDvd { #[arg(long)] start_date: Option<String>, #[arg(long)] days: Option<u32> },
}

#[derive(Subcommand, Clone)]
pub enum CheckinCommands {
    Create {
        #[arg(long)] item: String,
        #[arg(long)] sharing: Option<String>,
        #[arg(long)] message: Option<String>,
    },
    Delete,
}

#[derive(Subcommand, Clone)]
pub enum CertificationsCommands {
    List { type_: String },
}

#[derive(Subcommand, Clone)]
pub enum CommentsCommands {
    Create {
        #[arg(long)] item: String,
        #[arg(long)] comment: String,
        #[arg(long)] spoiler: bool,
        #[arg(long)] sharing: Option<String>,
    },
    Get { id: String },
    Update {
        id: String,
        #[arg(long)] comment: String,
        #[arg(long)] spoiler: bool,
    },
    Delete { id: String },
    Replies { id: String },
    Reply {
        id: String,
        #[arg(long)] comment: String,
        #[arg(long)] spoiler: bool,
    },
    Item { id: String },
    Likes { id: String },
    Like { id: String },
    Unlike { id: String },
    Trending {
        #[arg(long)] comment_type: Option<String>,
        #[arg(long)] type_: Option<String>,
        #[arg(long)] include_replies: bool,
    },
    Recent {
        #[arg(long)] comment_type: Option<String>,
        #[arg(long)] type_: Option<String>,
        #[arg(long)] include_replies: bool,
    },
    Updates {
        #[arg(long)] comment_type: Option<String>,
        #[arg(long)] type_: Option<String>,
        #[arg(long)] include_replies: bool,
    },
}

#[derive(Subcommand, Clone)]
pub enum CountriesCommands {
    List { type_: String },
}

#[derive(Subcommand, Clone)]
pub enum GenresCommands {
    List {
        type_: String,
        #[arg(long)] extended: Option<String>,
    },
}

#[derive(Subcommand, Clone)]
pub enum LanguagesCommands {
    List { type_: String },
}

#[derive(Subcommand, Clone)]
pub enum ListsCommands {
    Trending { #[arg(long)] type_: Option<String> },
    Popular { #[arg(long)] type_: Option<String> },
    Get { id: String },
    Likes { id: String },
    Like { id: String },
    Unlike { id: String },
    Items {
        id: String,
        #[arg(long)] type_: Option<String>,
        #[arg(long)] sort_by: Option<String>,
        #[arg(long)] sort_how: Option<String>,
    },
    Comments { id: String, #[arg(long)] sort: Option<String> },
}

#[derive(Subcommand, Clone)]
pub enum MoviesCommands {
    Trending,
    Popular,
    Favorited { #[arg(long)] period: Option<String> },
    Played { #[arg(long)] period: Option<String> },
    Watched { #[arg(long)] period: Option<String> },
    Collected { #[arg(long)] period: Option<String> },
    Anticipated,
    Boxoffice,
    Updates { #[arg(long)] start_date: Option<String> },
    UpdatedIds { #[arg(long)] start_date: Option<String> },
    Get { id: String, #[arg(long)] extended: Option<String> },
    Aliases { id: String },
    Releases { id: String, #[arg(long)] country: Option<String> },
    Translations { id: String, #[arg(long)] language: Option<String> },
    Comments { id: String, #[arg(long)] sort: Option<String> },
    Lists { id: String, #[arg(long)] type_: Option<String>, #[arg(long)] sort: Option<String> },
    People { id: String },
    Ratings { id: String },
    Related { id: String },
    Stats { id: String },
    Studios { id: String },
    Watching { id: String },
    Videos { id: String },
    Refresh { id: String },
}

#[derive(Subcommand, Clone)]
pub enum NetworksCommands {
    List,
}

#[derive(Subcommand, Clone)]
pub enum NotesCommands {
    Create {
        #[arg(long)] item: String,
        #[arg(long)] notes: String,
        #[arg(long)] spoiler: bool,
        #[arg(long)] privacy: Option<String>,
    },
    Get { id: String },
    Update {
        id: String,
        #[arg(long)] notes: String,
        #[arg(long)] spoiler: bool,
        #[arg(long)] privacy: Option<String>,
    },
    Delete { id: String },
    Item { id: String },
}

#[derive(Subcommand, Clone)]
pub enum PeopleCommands {
    Updates { #[arg(long)] start_date: Option<String> },
    UpdatedIds { #[arg(long)] start_date: Option<String> },
    Get { id: String, #[arg(long)] extended: Option<String> },
    Movies { id: String },
    Shows { id: String },
    Lists { id: String, #[arg(long)] type_: Option<String>, #[arg(long)] sort: Option<String> },
    Refresh { id: String },
}

#[derive(Subcommand, Clone)]
pub enum RecommendationsCommands {
    Movies { #[arg(long)] ignore_collected: bool, #[arg(long)] ignore_watchlisted: bool },
    HideMovie { id: String },
    Shows { #[arg(long)] ignore_collected: bool, #[arg(long)] ignore_watchlisted: bool },
    HideShow { id: String },
}

#[derive(Subcommand, Clone)]
pub enum ScrobbleCommands {
    Start { #[arg(long)] item: String, #[arg(long)] progress: f64 },
    Stop { #[arg(long)] item: String, #[arg(long)] progress: f64 },
}

#[derive(Subcommand, Clone)]
pub enum SearchCommands {
    Text { type_: String, query: String, #[arg(long)] fields: Option<String> },
    Id { id_type: String, id: String, #[arg(long)] type_: Option<String> },
}

#[derive(Subcommand, Clone)]
pub enum ShowsCommands {
    Trending,
    Popular,
    Favorited { #[arg(long)] period: Option<String> },
    Played { #[arg(long)] period: Option<String> },
    Watched { #[arg(long)] period: Option<String> },
    Collected { #[arg(long)] period: Option<String> },
    Anticipated,
    Updates { #[arg(long)] start_date: Option<String> },
    UpdatedIds { #[arg(long)] start_date: Option<String> },
    Get { id: String, #[arg(long)] extended: Option<String> },
    Aliases { id: String },
    Certifications { id: String },
    Translations { id: String, #[arg(long)] language: Option<String> },
    Comments { id: String, #[arg(long)] sort: Option<String> },
    Lists { id: String, #[arg(long)] type_: Option<String>, #[arg(long)] sort: Option<String> },
    ProgressCollection { id: String, #[arg(long)] hidden: bool, #[arg(long)] specials: bool, #[arg(long)] count_specials: bool, #[arg(long)] last_activity: Option<String> },
    ProgressWatched { id: String, #[arg(long)] hidden: bool, #[arg(long)] specials: bool, #[arg(long)] count_specials: bool, #[arg(long)] last_activity: Option<String> },
    ProgressWatchedReset { id: String, #[arg(long)] reset_at: Option<String> },
    ProgressWatchedUndoReset { id: String },
    People { id: String },
    Ratings { id: String },
    Related { id: String },
    Stats { id: String },
    Studios { id: String },
    Watching { id: String },
    NextEpisode { id: String },
    LastEpisode { id: String },
    Videos { id: String },
    Refresh { id: String },
}

#[derive(Subcommand, Clone)]
pub enum SeasonsCommands {
    List { id: String, #[arg(long)] extended: Option<String> },
    Get { id: String, season: String },
    Episodes { id: String, season: String, #[arg(long)] translations: Option<String> },
    Translations { id: String, season: String, #[arg(long)] language: Option<String> },
    Comments { id: String, season: String, #[arg(long)] sort: Option<String> },
    Lists { id: String, season: String, #[arg(long)] type_: Option<String>, #[arg(long)] sort: Option<String> },
    People { id: String, season: String },
    Ratings { id: String, season: String },
    Stats { id: String, season: String },
    Watching { id: String, season: String },
    Videos { id: String, season: String },
}

#[derive(Subcommand, Clone)]
pub enum EpisodesCommands {
    Get { id: String, season: String, episode: String },
    Translations { id: String, season: String, episode: String, #[arg(long)] language: Option<String> },
    Comments { id: String, season: String, episode: String, #[arg(long)] sort: Option<String> },
    Lists { id: String, season: String, episode: String, #[arg(long)] type_: Option<String>, #[arg(long)] sort: Option<String> },
    People { id: String, season: String, episode: String },
    Ratings { id: String, season: String, episode: String },
    Stats { id: String, season: String, episode: String },
    Watching { id: String, season: String, episode: String },
    Videos { id: String, season: String, episode: String },
}

#[derive(Subcommand, Clone)]
pub enum SyncCommands {
    LastActivities,
    Playback { #[arg(long)] type_: Option<String>, #[arg(long)] start_at: Option<String>, #[arg(long)] end_at: Option<String> },
    RemovePlayback { id: String },
    Collection { type_: String },
    AddCollection { #[arg(long)] items: String },
    RemoveCollection { #[arg(long)] items: String },
    Watched { type_: String },
    History { #[arg(long)] type_: Option<String>, #[arg(long)] id: Option<String>, #[arg(long)] start_at: Option<String>, #[arg(long)] end_at: Option<String> },
    AddHistory { #[arg(long)] items: String },
    RemoveHistory { #[arg(long)] items: String },
    Ratings { #[arg(long)] type_: Option<String>, #[arg(long)] rating: Option<String> },
    AddRatings { #[arg(long)] items: String },
    RemoveRatings { #[arg(long)] items: String },
    Watchlist { #[arg(long)] type_: Option<String>, #[arg(long)] sort_by: Option<String>, #[arg(long)] sort_how: Option<String> },
    UpdateWatchlist { #[arg(long)] description: Option<String>, #[arg(long)] sort_by: Option<String>, #[arg(long)] sort_how: Option<String> },
    AddWatchlist { #[arg(long)] items: String },
    RemoveWatchlist { #[arg(long)] items: String },
    ReorderWatchlist { #[arg(long)] rank: String },
    UpdateWatchlistItem { list_item_id: String, #[arg(long)] notes: String },
    Favorites { #[arg(long)] type_: Option<String>, #[arg(long)] sort_by: Option<String>, #[arg(long)] sort_how: Option<String> },
    UpdateFavorites { #[arg(long)] description: Option<String>, #[arg(long)] sort_by: Option<String>, #[arg(long)] sort_how: Option<String> },
    AddFavorites { #[arg(long)] items: String },
    RemoveFavorites { #[arg(long)] items: String },
    ReorderFavorites { #[arg(long)] rank: String },
    UpdateFavoriteItem { list_item_id: String, #[arg(long)] notes: String },
}

#[derive(Subcommand, Clone)]
pub enum UsersCommands {
    Settings,
    RequestsFollowing,
    Requests,
    ApproveRequest { id: String },
    DenyRequest { id: String },
    SavedFilters { #[arg(long)] section: Option<String> },
    Hidden { section: String, #[arg(long)] type_: Option<String> },
    AddHidden { section: String, #[arg(long)] items: String },
    RemoveHidden { section: String, #[arg(long)] items: String },
    Profile { id: String },
    Likes { id: String, #[arg(long)] type_: Option<String> },
    Collection { id: String, type_: String },
    Comments { id: String, #[arg(long)] comment_type: Option<String>, #[arg(long)] type_: Option<String>, #[arg(long)] include_replies: Option<bool> },
    Notes { id: String, #[arg(long)] type_: Option<String> },
    Lists { id: String },
    CreateList {
        id: String,
        #[arg(long)] name: String,
        #[arg(long)] description: Option<String>,
        #[arg(long)] privacy: Option<String>,
        #[arg(long)] display_numbers: Option<bool>,
        #[arg(long)] allow_comments: Option<bool>,
        #[arg(long)] sort_by: Option<String>,
        #[arg(long)] sort_how: Option<String>,
    },
    ReorderLists { id: String, #[arg(long)] rank: String },
    Collaborations { id: String },
    GetList { id: String, list_id: String },
    UpdateList {
        id: String,
        list_id: String,
        #[arg(long)] name: Option<String>,
        #[arg(long)] description: Option<String>,
        #[arg(long)] privacy: Option<String>,
        #[arg(long)] display_numbers: Option<bool>,
        #[arg(long)] allow_comments: Option<bool>,
        #[arg(long)] sort_by: Option<String>,
        #[arg(long)] sort_how: Option<String>,
    },
    DeleteList { id: String, list_id: String },
    ListLikes { id: String, list_id: String },
    LikeList { id: String, list_id: String },
    UnlikeList { id: String, list_id: String },
    ListItems {
        id: String,
        list_id: String,
        #[arg(long)] type_: Option<String>,
        #[arg(long)] sort_by: Option<String>,
        #[arg(long)] sort_how: Option<String>,
    },
    AddListItems { id: String, list_id: String, #[arg(long)] items: String },
    RemoveListItems { id: String, list_id: String, #[arg(long)] items: String },
    ReorderListItems { id: String, list_id: String, #[arg(long)] rank: String },
    UpdateListItem { id: String, list_id: String, list_item_id: String, #[arg(long)] notes: String },
    ListComments { id: String, list_id: String, #[arg(long)] sort: Option<String> },
    Follow { id: String },
    Unfollow { id: String },
    Followers { id: String },
    Following { id: String },
    Friends { id: String },
    History { id: String, #[arg(long)] type_: Option<String>, #[arg(long)] item_id: Option<String>, #[arg(long)] start_at: Option<String>, #[arg(long)] end_at: Option<String> },
    Ratings { id: String, #[arg(long)] type_: Option<String>, #[arg(long)] rating: Option<String> },
    Watchlist { id: String, #[arg(long)] type_: Option<String>, #[arg(long)] sort_by: Option<String>, #[arg(long)] sort_how: Option<String> },
    WatchlistComments { id: String, #[arg(long)] sort: Option<String> },
    Favorites { id: String, #[arg(long)] type_: Option<String>, #[arg(long)] sort_by: Option<String>, #[arg(long)] sort_how: Option<String> },
    FavoritesComments { id: String, #[arg(long)] sort: Option<String> },
    Watching { id: String },
    Watched { id: String, type_: String },
    Stats { id: String },
}
