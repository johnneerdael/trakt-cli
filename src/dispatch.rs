use crate::commands::*;
use crate::{api, auth, client, config, pagination};
use anyhow::Result;
use serde_json::Value;

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;
pub const EXIT_USAGE: i32 = 2;
pub const EXIT_NOT_FOUND: i32 = 3;
pub const EXIT_AUTH_ERROR: i32 = 4;
pub const EXIT_CONFLICT: i32 = 5;

pub fn output_result(result: Result<Value>, json: bool, quiet: bool) -> i32 {
    match result {
        Ok(value) => {
            if json {
                println!("{}", serde_json::to_string(&value).unwrap_or_default());
            } else if quiet {
                if let Some(arr) = value.as_array() {
                    for item in arr {
                        if let Some(obj) = item.as_object() {
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

pub async fn run_cli(cli: Cli) -> Result<i32> {
    match cli.command.clone() {
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

        Commands::Search(sub) => handle_search(&cli, sub).await,
        Commands::Calendars(sub) => handle_calendars(&cli, sub).await,
        Commands::Checkin(sub) => handle_checkin(&cli, sub).await,
        Commands::Certifications(sub) => handle_certifications(&cli, sub).await,
        Commands::Comments(sub) => handle_comments(&cli, sub).await,
        Commands::Countries(sub) => handle_countries(&cli, sub).await,
        Commands::Genres(sub) => handle_genres(&cli, sub).await,
        Commands::Languages(sub) => handle_languages(&cli, sub).await,
        Commands::Lists(sub) => handle_lists(&cli, sub).await,
        Commands::Movies(sub) => handle_movies(&cli, sub).await,
        Commands::Networks(sub) => handle_networks(&cli, sub).await,
        Commands::Notes(sub) => handle_notes(&cli, sub).await,
        Commands::People(sub) => handle_people(&cli, sub).await,
        Commands::Recommendations(sub) => handle_recommendations(&cli, sub).await,
        Commands::Scrobble(sub) => handle_scrobble(&cli, sub).await,
        Commands::Shows(sub) => handle_shows(&cli, sub).await,
        Commands::Seasons(sub) => handle_seasons(&cli, sub).await,
        Commands::Episodes(sub) => handle_episodes(&cli, sub).await,
        Commands::Sync(sub) => handle_sync(&cli, sub).await,
        Commands::Users(sub) => handle_users(&cli, sub).await,
    }
}

async fn get_client_and_pagination(cli: &Cli) -> Result<(client::TraktClient, Option<pagination::Pagination>)> {
    let cfg = config::Config::load()?;
    let client = client::TraktClient::new(cfg).await?;
    let pagination = pagination::Pagination::new()
        .with_page(cli.page.unwrap_or(1))
        .with_limit(cli.limit.unwrap_or(10));
    Ok((client, Some(pagination)))
}

async fn handle_search(cli: &Cli, sub: SearchCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let res = match sub {
        SearchCommands::Text { type_, query, fields } => api::Api::search(&client, &type_, &query, pagination).await,
        SearchCommands::Id { id_type, id, type_ } => api::Api::search_id(&client, &id_type, &id).await, 
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_calendars(cli: &Cli, sub: CalendarsCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        CalendarsCommands::MyShows { start_date, days } => api::Api::get_calendar_shows(&client, start_date.as_deref(), days).await,
        CalendarsCommands::MyNewShows { start_date, days } => api::Api::get_calendar_new_shows(&client, start_date.as_deref(), days).await,
        CalendarsCommands::MySeasonPremieres { start_date, days } => api::Api::get_calendar_shows_premieres(&client, start_date.as_deref(), days).await,
        CalendarsCommands::MyFinales { start_date, days } => api::Api::get_calendar_shows_finales(&client, start_date.as_deref(), days).await,
        CalendarsCommands::MyMovies { start_date, days } => api::Api::get_calendar_movies(&client, start_date.as_deref(), days).await,
        CalendarsCommands::MyStreaming { start_date, days } => api::Api::get_calendar_streaming(&client, start_date.as_deref(), days).await,
        CalendarsCommands::MyDvd { start_date, days } => api::Api::get_calendar_dvd(&client, start_date.as_deref(), days).await,
        CalendarsCommands::AllShows { start_date, days } => api::Api::get_all_shows_calendar(&client, start_date.as_deref(), days).await,
        CalendarsCommands::AllNewShows { start_date, days } => api::Api::get_all_new_shows_calendar(&client, start_date.as_deref(), days).await,
        CalendarsCommands::AllSeasonPremieres { start_date, days } => api::Api::get_all_shows_premieres_calendar(&client, start_date.as_deref(), days).await,
        CalendarsCommands::AllFinales { start_date, days } => api::Api::get_all_shows_finales_calendar(&client, start_date.as_deref(), days).await,
        CalendarsCommands::AllMovies { start_date, days } => api::Api::get_all_movies_calendar(&client, start_date.as_deref(), days).await,
        CalendarsCommands::AllStreaming { start_date, days } => api::Api::get_all_streaming_calendar(&client, start_date.as_deref(), days).await,
        CalendarsCommands::AllDvd { start_date, days } => api::Api::get_all_dvd_calendar(&client, start_date.as_deref(), days).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_checkin(cli: &Cli, sub: CheckinCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        CheckinCommands::Create { item, sharing: _, message } => {
            let data: Value = serde_json::from_str(&item)?;
            api::Api::checkin(&client, data).await
        },
        CheckinCommands::Delete => api::Api::delete_checkin(&client).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_certifications(cli: &Cli, sub: CertificationsCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        CertificationsCommands::List { type_ } => {
            if type_ == "movies" { api::Api::get_certifications_movies(&client).await } else { api::Api::get_certifications_shows(&client).await }
        },
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_comments(cli: &Cli, sub: CommentsCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let res = match sub {
        CommentsCommands::Create { item, comment, spoiler, sharing: _ } => {
            let item_val: Value = serde_json::from_str(&item)?;
            let item_type = item_val.as_object().unwrap().keys().next().unwrap();
            let id = item_val[item_type]["ids"]["trakt"].as_str().unwrap();
            api::Api::post_comment(&client, item_type, id, &comment, Some(spoiler)).await
        },
        CommentsCommands::Get { id } => api::Api::get_comment(&client, &id).await,
        CommentsCommands::Update { id, comment, spoiler: _ } => api::Api::update_comment(&client, &id, &comment, None).await,
        CommentsCommands::Delete { id } => api::Api::delete_comment(&client, &id).await,
        CommentsCommands::Replies { id } => api::Api::get_comment_replies(&client, &id, pagination).await,
        CommentsCommands::Reply { id, comment, spoiler: _ } => {
            api::Api::post_comment(&client, "comment", &id, &comment, None).await
        },
        CommentsCommands::Item { id } => api::Api::get_comment_item(&client, &id).await,
        CommentsCommands::Likes { id } => api::Api::get_comment_likers(&client, &id).await,
        CommentsCommands::Like { id } => api::Api::like_comment(&client, &id).await,
        CommentsCommands::Unlike { id } => api::Api::unlike_comment(&client, &id).await,
        CommentsCommands::Trending { comment_type, type_, include_replies: _ } => api::Api::get_trending_comments(&client, comment_type.as_deref(), type_.as_deref(), None, pagination).await,
        CommentsCommands::Recent { comment_type: _, type_: _, include_replies: _ } => api::Api::get_recently_created_comments(&client, pagination).await,
        CommentsCommands::Updates { comment_type: _, type_: _, include_replies: _ } => api::Api::get_recently_updated_comments(&client, pagination).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_countries(cli: &Cli, sub: CountriesCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        CountriesCommands::List { type_ } => {
            if type_ == "movies" { api::Api::get_countries_movies(&client).await } else { api::Api::get_countries_shows(&client).await }
        },
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_genres(cli: &Cli, sub: GenresCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        GenresCommands::List { type_, extended } => api::Api::get_genres(&client, &type_, extended.as_deref()).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_languages(cli: &Cli, sub: LanguagesCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        LanguagesCommands::List { type_ } => api::Api::get_languages(&client).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_lists(cli: &Cli, sub: ListsCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let res = match sub {
        ListsCommands::Trending { type_ } => api::Api::get_trending_lists(&client, pagination).await,
        ListsCommands::Popular { type_ } => api::Api::get_popular_lists(&client, pagination).await,
        ListsCommands::Get { id } => api::Api::get_list(&client, &id, pagination).await,
        ListsCommands::Likes { id } => api::Api::get_list_likers(&client, &id, pagination).await,
        ListsCommands::Like { id } => api::Api::like_list(&client, &id).await,
        ListsCommands::Unlike { id } => api::Api::unlike_list(&client, &id).await,
        ListsCommands::Items { id, type_, sort_by, sort_how } => api::Api::get_list_items(&client, &id, pagination).await,
        ListsCommands::Comments { id, sort } => api::Api::get_list_comments(&client, &id, pagination).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_movies(cli: &Cli, sub: MoviesCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let extended = cli.extended.as_deref();
    let res = match sub {
        MoviesCommands::Trending => api::Api::get_trending_movies(&client, pagination, extended).await,
        MoviesCommands::Popular => api::Api::get_popular_movies(&client, pagination, extended).await,
        MoviesCommands::Favorited { period } => api::Api::get_movies_favorited(&client, pagination, extended).await,
        MoviesCommands::Played { period } => api::Api::get_movies_played(&client, pagination, extended).await,
        MoviesCommands::Watched { period } => api::Api::get_movies_watched(&client, pagination, extended).await,
        MoviesCommands::Collected { period } => api::Api::get_movies_collected(&client, pagination, extended).await,
        MoviesCommands::Anticipated => api::Api::get_movies_anticipated(&client, pagination, extended).await,
        MoviesCommands::Boxoffice => api::Api::get_movies_boxoffice(&client, extended).await,
        MoviesCommands::Updates { start_date } => api::Api::get_updated_movies(&client, start_date.as_deref(), pagination).await,
        MoviesCommands::UpdatedIds { start_date } => api::Api::get_updated_movies(&client, start_date.as_deref(), pagination).await,
        MoviesCommands::Get { id, extended } => api::Api::get_movie(&client, &id, extended.as_deref()).await,
        MoviesCommands::Aliases { id } => api::Api::get_movie_aliases(&client, &id).await,
        MoviesCommands::Releases { id, country } => api::Api::get_movie_releases(&client, &id, country.as_deref()).await,
        MoviesCommands::Translations { id, language } => api::Api::get_movie_translations(&client, &id, language.as_deref()).await,
        MoviesCommands::Comments { id, sort } => api::Api::get_movie_comments(&client, &id, pagination).await,
        MoviesCommands::Lists { id, type_, sort } => api::Api::get_movie_lists(&client, &id, pagination).await,
        MoviesCommands::People { id } => api::Api::get_movie_people_extended(&client, &id, cli.extended.as_deref()).await,
        MoviesCommands::Ratings { id } => api::Api::get_movie_ratings(&client, &id).await,
        MoviesCommands::Related { id } => api::Api::get_movie_related(&client, &id, pagination).await,
        MoviesCommands::Stats { id } => api::Api::get_movie_stats(&client, &id).await,
        MoviesCommands::Studios { id } => api::Api::get_movie_studios(&client, &id).await,
        MoviesCommands::Watching { id } => api::Api::get_movie_watching(&client, &id, extended).await,
        MoviesCommands::Videos { id } => api::Api::get_movie_videos(&client, &id).await,
        MoviesCommands::Refresh { id } => api::Api::refresh_movie_metadata(&client, &id).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_networks(cli: &Cli, sub: NetworksCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        NetworksCommands::List => api::Api::get_networks(&client).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_notes(cli: &Cli, sub: NotesCommands) -> Result<i32> {
            let (client, _) = get_client_and_pagination(cli).await?;
        let res = match sub {
            NotesCommands::Create { item, notes, spoiler, privacy } => {
                let mut data: Value = serde_json::from_str(&item)?;
                data["notes"] = serde_json::json!(notes);
                data["spoiler"] = serde_json::json!(spoiler);
                if let Some(p) = privacy { data["privacy"] = serde_json::json!(p); }
                api::Api::add_notes(&client, data).await
            },
            NotesCommands::Get { id } => api::Api::get_note(&client, &id).await,
            NotesCommands::Update { id, notes, spoiler, privacy } => {
                let mut data = serde_json::json!({
                    "notes": notes,
                    "spoiler": spoiler
                });
                if let Some(p) = privacy { data["privacy"] = serde_json::json!(p); }
                api::Api::update_note(&client, &id, &notes).await
            },
            NotesCommands::Delete { id } => api::Api::delete_note(&client, &id).await,
            NotesCommands::Item { id } => api::Api::get_note_item(&client, &id).await,
        };
        Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_people(cli: &Cli, sub: PeopleCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let res = match sub {
        PeopleCommands::Updates { start_date } => api::Api::get_updated_people(&client, start_date.as_deref(), pagination).await,
        PeopleCommands::UpdatedIds { start_date } => api::Api::get_updated_people(&client, start_date.as_deref(), pagination).await,
        PeopleCommands::Get { id, extended } => api::Api::get_person(&client, &id, extended.as_deref()).await,
        PeopleCommands::Movies { id } => api::Api::get_person_movies(&client, &id).await,
        PeopleCommands::Shows { id } => api::Api::get_person_shows(&client, &id).await,
        PeopleCommands::Lists { id, type_, sort } => api::Api::get_person_lists(&client, &id, pagination).await,
        PeopleCommands::Refresh { id } => api::Api::refresh_person_metadata(&client, &id).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_recommendations(cli: &Cli, sub: RecommendationsCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let extended = cli.extended.as_deref();
    let res = match sub {
        RecommendationsCommands::Movies { ignore_collected, ignore_watchlisted } => api::Api::get_movie_recommendations(&client, extended).await,
        RecommendationsCommands::HideMovie { id } => api::Api::hide_movie_recommendation(&client, &id).await,
        RecommendationsCommands::Shows { ignore_collected, ignore_watchlisted } => api::Api::get_show_recommendations(&client, extended).await,
        RecommendationsCommands::HideShow { id } => api::Api::hide_show_recommendation(&client, &id).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_scrobble(cli: &Cli, sub: ScrobbleCommands) -> Result<i32> {
    let (client, _) = get_client_and_pagination(cli).await?;
    let res = match sub {
        ScrobbleCommands::Start { item, progress } => {
            let data: Value = serde_json::from_str(&item)?;
            api::Api::scrobble_start(&client, data).await
        },
        ScrobbleCommands::Stop { item, progress } => {
            let data: Value = serde_json::from_str(&item)?;
            api::Api::scrobble_stop(&client, data).await
        },
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_shows(cli: &Cli, sub: ShowsCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let extended = cli.extended.as_deref();
    let res = match sub {
        ShowsCommands::Trending => api::Api::get_trending_shows(&client, pagination, extended).await,
        ShowsCommands::Popular => api::Api::get_popular_shows(&client, pagination, extended).await,
        ShowsCommands::Favorited { period } => api::Api::get_shows_favorited(&client, pagination, extended).await,
        ShowsCommands::Played { period } => api::Api::get_shows_played(&client, pagination, extended).await,
        ShowsCommands::Watched { period } => api::Api::get_shows_watched(&client, pagination, extended).await,
        ShowsCommands::Collected { period } => api::Api::get_shows_collected(&client, pagination, extended).await,
        ShowsCommands::Anticipated => api::Api::get_shows_anticipated(&client, pagination, extended).await,
        ShowsCommands::Updates { start_date } => api::Api::get_updated_shows(&client, start_date.as_deref(), pagination).await,
        ShowsCommands::UpdatedIds { start_date } => api::Api::get_updated_shows(&client, start_date.as_deref(), pagination).await,
        ShowsCommands::Get { id, extended } => api::Api::get_show(&client, &id, extended.as_deref()).await,
        ShowsCommands::Aliases { id } => api::Api::get_show_aliases(&client, &id).await,
        ShowsCommands::Certifications { id } => api::Api::get_show_certifications(&client, &id).await,
        ShowsCommands::Translations { id, language } => api::Api::get_show_translations(&client, &id, language.as_deref()).await,
        ShowsCommands::Comments { id, sort } => api::Api::get_show_comments(&client, &id, pagination).await,
        ShowsCommands::Lists { id, type_, sort } => api::Api::get_show_lists(&client, &id, pagination).await,
        ShowsCommands::ProgressCollection { id, hidden: _, specials: _, count_specials: _, last_activity: _ } => api::Api::get_show_collection_progress(&client, &id, None, None).await,
        ShowsCommands::ProgressWatched { id, hidden: _, specials: _, count_specials: _, last_activity: _ } => api::Api::get_show_watched_progress(&client, &id, None, None).await,
        ShowsCommands::ProgressWatchedReset { id, reset_at: _ } => api::Api::reset_show_progress(&client, &id).await,
        ShowsCommands::ProgressWatchedUndoReset { id } => api::Api::undo_reset_show_progress(&client, &id).await,
        ShowsCommands::People { id } => api::Api::get_show_people_extended(&client, &id, cli.extended.as_deref()).await,
        ShowsCommands::Ratings { id } => api::Api::get_show_ratings(&client, &id).await,
        ShowsCommands::Related { id } => api::Api::get_show_related(&client, &id, pagination).await,
        ShowsCommands::Stats { id } => api::Api::get_show_stats(&client, &id).await,
        ShowsCommands::Studios { id } => api::Api::get_show_studios(&client, &id).await,
        ShowsCommands::Watching { id } => api::Api::get_show_watching(&client, &id, extended).await,
        ShowsCommands::NextEpisode { id } => api::Api::get_show_next_episode(&client, &id, extended).await,
        ShowsCommands::LastEpisode { id } => api::Api::get_show_last_episode(&client, &id, extended).await,
        ShowsCommands::Videos { id } => api::Api::get_show_videos(&client, &id).await,
        ShowsCommands::Refresh { id } => api::Api::refresh_show_metadata(&client, &id).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_seasons(cli: &Cli, sub: SeasonsCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let extended = cli.extended.as_deref();
    let res = match sub {
        SeasonsCommands::List { id, extended } => api::Api::get_show_seasons(&client, &id, extended.as_deref()).await,
        SeasonsCommands::Get { id, season } => api::Api::get_season(&client, &id, &season, extended).await,
        SeasonsCommands::Episodes { id, season, translations: _ } => api::Api::get_season_episodes(&client, &id, &season, extended).await,
        SeasonsCommands::Translations { id, season, language } => api::Api::get_season_translations(&client, &id, &season, language.as_deref()).await,
        SeasonsCommands::Comments { id, season, sort } => api::Api::get_season_comments(&client, &id, &season, pagination).await,
        SeasonsCommands::Lists { id, season, type_, sort } => api::Api::get_season_lists(&client, &id, &season, pagination).await,
        SeasonsCommands::People { id, season } => api::Api::get_season_people(&client, &id, &season, extended).await,
        SeasonsCommands::Ratings { id, season } => api::Api::get_season_ratings(&client, &id, &season).await,
        SeasonsCommands::Stats { id, season } => api::Api::get_season_stats(&client, &id, &season).await,
        SeasonsCommands::Watching { id, season } => api::Api::get_season_watching(&client, &id, &season, extended).await,
        SeasonsCommands::Videos { id, season } => api::Api::get_season_videos(&client, &id, &season).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_episodes(cli: &Cli, sub: EpisodesCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let extended = cli.extended.as_deref();
    let res = match sub {
        EpisodesCommands::Get { id, season, episode } => api::Api::get_episode(&client, &id, &season, &episode, extended).await,
        EpisodesCommands::Translations { id, season, episode, language } => api::Api::get_episode_translations(&client, &id, &season, &episode, language.as_deref()).await,
        EpisodesCommands::Comments { id, season, episode, sort } => api::Api::get_episode_comments(&client, &id, &season, &episode, pagination).await,
        EpisodesCommands::Lists { id, season, episode, type_, sort } => api::Api::get_episode_lists(&client, &id, &season, &episode, pagination).await,
        EpisodesCommands::People { id, season, episode } => api::Api::get_episode_people(&client, &id, &season, &episode, extended).await,
        EpisodesCommands::Ratings { id, season, episode } => api::Api::get_episode_ratings(&client, &id, &season, &episode).await,
        EpisodesCommands::Stats { id, season, episode } => api::Api::get_episode_stats(&client, &id, &season, &episode).await,
        EpisodesCommands::Watching { id, season, episode } => api::Api::get_episode_watching(&client, &id, &season, &episode, extended).await,
        EpisodesCommands::Videos { id, season, episode } => api::Api::get_episode_videos(&client, &id, &season, &episode).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_sync(cli: &Cli, sub: SyncCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let res = match sub {
        SyncCommands::LastActivities => api::Api::get_last_activity(&client).await,
        SyncCommands::Playback { type_, start_at: _, end_at: _ } => api::Api::get_playback_progress(&client, type_.as_deref(), pagination).await,
        SyncCommands::RemovePlayback { id } => api::Api::remove_playback_item(&client, &id).await,
        SyncCommands::Collection { type_ } => {
            if type_ == "movies" { api::Api::get_sync_collection_movies(&client, cli.extended.as_deref()).await } else { api::Api::getSync_collection_shows(&client, cli.extended.as_deref()).await }
        },
        SyncCommands::AddCollection { items } => api::Api::add_to_collection(&client, serde_json::from_str(&items)?).await,
        SyncCommands::RemoveCollection { items } => api::Api::remove_from_collection(&client, serde_json::from_str(&items)?).await,
        SyncCommands::Watched { type_ } => {
            if type_ == "movies" { api::Api::get_sync_watched(&client, "movies", cli.extended.as_deref()).await } else { api::Api::get_sync_watched(&client, "shows", cli.extended.as_deref()).await }
        },
        SyncCommands::History { type_, id, start_at, end_at } => {
            if let Some(t) = type_ {
                if t == "movies" { api::Api::get_sync_history(&client, Some("movies"), pagination).await }
                else if t == "shows" { api::Api::get_sync_history(&client, Some("shows"), pagination).await }
                else { api::Api::get_sync_history(&client, Some("episodes"), pagination).await }
            } else {
                api::Api::get_sync_history(&client, Some("movies"), pagination).await
            }
        },
        SyncCommands::AddHistory { items } => api::Api::add_to_history(&client, serde_json::from_str(&items)?).await,
        SyncCommands::RemoveHistory { items } => api::Api::remove_from_history(&client, serde_json::from_str(&items)?).await,
        SyncCommands::Ratings { type_, rating } => api::Api::get_sync_ratings(&client, type_.as_deref(), pagination).await,
        SyncCommands::AddRatings { items } => api::Api::add_ratings(&client, serde_json::from_str(&items)?).await,
        SyncCommands::RemoveRatings { items } => api::Api::remove_ratings(&client, serde_json::from_str(&items)?).await,
        SyncCommands::Watchlist { type_, sort_by, sort_how } => api::Api::get_sync_watchlist(&client, type_.as_deref(), pagination).await,
        SyncCommands::UpdateWatchlist { description, sort_by, sort_how } => {
            let mut data = serde_json::json!({});
            if let Some(d) = description { data["description"] = serde_json::json!(d); }
            if let Some(s) = sort_by { data["sort_by"] = serde_json::json!(s); }
            if let Some(s) = sort_how { data["sort_how"] = serde_json::json!(s); }
            api::Api::update_user_list(&client, "me", "watchlist", data).await
        },
        SyncCommands::AddWatchlist { items } => api::Api::add_to_watchlist(&client, serde_json::from_str(&items)?).await,
        SyncCommands::RemoveWatchlist { items } => api::Api::remove_from_watchlist(&client, serde_json::from_str(&items)?).await,
        SyncCommands::ReorderWatchlist { rank } => api::Api::reorder_watchlist(&client, serde_json::json!({ "rank": rank.split(',').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>() })).await,
        SyncCommands::UpdateWatchlistItem { list_item_id, notes } => api::Api::update_watchlist_item(&client, &list_item_id, Some(&notes)).await,
        SyncCommands::Favorites { type_, sort_by, sort_how } => api::Api::get_sync_favorites(&client, type_.as_deref(), pagination).await,
        SyncCommands::UpdateFavorites { description, sort_by, sort_how } => {
            let mut data = serde_json::json!({});
            if let Some(d) = description { data["description"] = serde_json::json!(d); }
            if let Some(s) = sort_by { data["sort_by"] = serde_json::json!(s); }
            if let Some(s) = sort_how { data["sort_how"] = serde_json::json!(s); }
            api::Api::update_user_list(&client, "me", "favorites", data).await
        },
        SyncCommands::AddFavorites { items } => api::Api::add_to_favorites(&client, serde_json::from_str(&items)?).await,
        SyncCommands::RemoveFavorites { items } => api::Api::remove_from_favorites(&client, serde_json::from_str(&items)?).await,
        SyncCommands::ReorderFavorites { rank } => api::Api::reorder_favorites(&client, serde_json::json!({ "rank": rank.split(',').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>() })).await,
        SyncCommands::UpdateFavoriteItem { list_item_id, notes } => api::Api::update_favorite_item(&client, &list_item_id, Some(&notes)).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}

async fn handle_users(cli: &Cli, sub: UsersCommands) -> Result<i32> {
    let (client, pagination) = get_client_and_pagination(cli).await?;
    let extended = cli.extended.as_deref();
    let res = match sub {
        UsersCommands::Settings => api::Api::get_user_settings(&client).await,
        UsersCommands::RequestsFollowing => api::Api::get_user_requests_following(&client).await,
        UsersCommands::Requests => api::Api::get_user_requests(&client).await,
        UsersCommands::ApproveRequest { id } => api::Api::approve_user_request(&client, &id).await,
        UsersCommands::DenyRequest { id } => api::Api::deny_user_request(&client, &id).await,
        UsersCommands::SavedFilters { section } => api::Api::get_saved_filters(&client, section.as_deref().unwrap_or(""), pagination).await,
        UsersCommands::Hidden { section, type_ } => api::Api::get_hidden_items(&client, &section, type_.as_deref()).await,
        UsersCommands::AddHidden { section, items } => api::Api::add_hidden_items(&client, &section, "", serde_json::from_str(&items)?).await,
        UsersCommands::RemoveHidden { section, items } => api::Api::remove_hidden_items(&client, &section, "", serde_json::from_str(&items)?).await,
        UsersCommands::Profile { id } => api::Api::get_user_profile(&client, &id).await,
        UsersCommands::Likes { id, type_ } => api::Api::get_user_likes(&client, &id, pagination).await,
        UsersCommands::Collection { id, type_ } => {
            if type_ == "movies" { api::Api::get_user_collection_movies(&client, &id, extended).await } else { api::Api::get_user_collection_shows(&client, &id, extended).await }
        },
        UsersCommands::Comments { id, comment_type, type_, include_replies } => api::Api::get_user_comments(&client, &id, comment_type.as_deref(), type_.as_deref(), None, pagination).await,
        UsersCommands::Notes { id, type_ } => api::Api::get_user_notes(&client, &id, type_.as_deref(), pagination).await,
        UsersCommands::Lists { id } => api::Api::get_user_lists(&client, &id).await,
        UsersCommands::CreateList { id, name, description, privacy, display_numbers, allow_comments, sort_by, sort_how } => {
            let mut data = serde_json::json!({ "name": name });
            if let Some(d) = description { data["description"] = serde_json::json!(d); }
            if let Some(p) = privacy { data["privacy"] = serde_json::json!(p); }
            if let Some(dn) = display_numbers { data["display_numbers"] = serde_json::json!(dn); }
            if let Some(ac) = allow_comments { data["allow_comments"] = serde_json::json!(ac); }
            if let Some(sb) = sort_by { data["sort_by"] = serde_json::json!(sb); }
            if let Some(sh) = sort_how { data["sort_how"] = serde_json::json!(sh); }
            api::Api::create_user_list(&client, &id, data).await
        },
        UsersCommands::ReorderLists { id, rank } => api::Api::reorder_lists(&client, &id, serde_json::json!({ "rank": rank.split(',').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>() })).await,
        UsersCommands::Collaborations { id } => api::Api::get_user_collaborations(&client, &id).await,
        UsersCommands::GetList { id, list_id } => api::Api::get_user_list(&client, &id, &list_id).await,
        UsersCommands::UpdateList { id, list_id, name, description, privacy, display_numbers, allow_comments, sort_by, sort_how } => {
            let mut data = serde_json::json!({});
            if let Some(n) = name { data["name"] = serde_json::json!(n); }
            if let Some(d) = description { data["description"] = serde_json::json!(d); }
            if let Some(p) = privacy { data["privacy"] = serde_json::json!(p); }
            if let Some(dn) = display_numbers { data["display_numbers"] = serde_json::json!(dn); }
            if let Some(ac) = allow_comments { data["allow_comments"] = serde_json::json!(ac); }
            if let Some(sb) = sort_by { data["sort_by"] = serde_json::json!(sb); }
            if let Some(sh) = sort_how { data["sort_how"] = serde_json::json!(sh); }
            api::Api::update_user_list(&client, &id, &list_id, data).await
        },
        UsersCommands::DeleteList { id, list_id } => api::Api::delete_user_list(&client, &id, &list_id).await,
        UsersCommands::ListLikes { id, list_id } => api::Api::get_list_likers(&client, &list_id, pagination).await,
        UsersCommands::LikeList { id, list_id } => api::Api::like_user_list(&client, &id, &list_id).await,
        UsersCommands::UnlikeList { id, list_id } => api::Api::unlike_user_list(&client, &id, &list_id).await,
        UsersCommands::ListItems { id, list_id, type_, sort_by: _, sort_how: _ } => api::Api::get_user_list_items(&client, &id, &list_id, pagination).await,
        UsersCommands::AddListItems { id, list_id, items } => api::Api::add_user_list_items(&client, &id, &list_id, serde_json::from_str(&items)?).await,
        UsersCommands::RemoveListItems { id, list_id, items } => api::Api::remove_user_list_items(&client, &id, &list_id, serde_json::from_str(&items)?).await,
        UsersCommands::ReorderListItems { id, list_id, rank } => api::Api::reorder_user_list_items(&client, &id, &list_id, serde_json::json!({ "rank": rank.split(',').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>() })).await,
        UsersCommands::UpdateListItem { id, list_id, list_item_id, notes } => {
            let data = serde_json::json!({ "notes": notes });
            api::Api::update_user_list_item(&client, &id, &list_id, &list_item_id, data).await
        },
        UsersCommands::ListComments { id, list_id, sort } => api::Api::get_list_comments(&client, &list_id, pagination).await,
        UsersCommands::Follow { id } => api::Api::follow_user(&client, &id).await,
        UsersCommands::Unfollow { id } => api::Api::unfollow_user(&client, &id).await,
        UsersCommands::Followers { id } => api::Api::get_user_followers(&client, &id, pagination).await,
        UsersCommands::Following { id } => api::Api::get_user_following(&client, &id, pagination).await,
        UsersCommands::Friends { id } => api::Api::get_user_friends(&client, &id, pagination).await,
        UsersCommands::History { id, type_, item_id: _, start_at: _, end_at: _ } => {
            if let Some(t) = type_ {
                if t == "movies" { api::Api::get_user_history_movies(&client, &id, pagination).await }
                else if t == "shows" { api::Api::get_user_history_shows(&client, &id, pagination).await }
                else { api::Api::get_user_history_episodes(&client, &id, pagination).await }
            } else {
                api::Api::get_user_history_movies(&client, &id, pagination).await
            }
        },
        UsersCommands::Ratings { id, type_, rating } => api::Api::get_user_ratings(&client, &id, type_.as_deref()).await,
        UsersCommands::Watchlist { id, type_, sort_by: _, sort_how: _ } => {
            if let Some(t) = type_ {
                if t == "movies" { api::Api::get_user_watchlist_movies(&client, &id, pagination).await }
                else { api::Api::get_user_watchlist_shows(&client, &id, pagination).await }
            } else {
                api::Api::get_user_watchlist_movies(&client, &id, pagination).await
            }
        },
        UsersCommands::WatchlistComments { id, sort } => api::Api::get_watchlist_comments(&client, &id, sort.as_deref(), pagination).await,
        UsersCommands::Favorites { id, type_, sort_by: _, sort_how: _ } => {
            api::Api::get_user_favorites(&client, &id, type_.as_deref(), pagination).await
        },
        UsersCommands::FavoritesComments { id, sort } => api::Api::get_favorites_comments(&client, &id, sort.as_deref(), pagination).await,
        UsersCommands::Watching { id } => api::Api::get_user_watching(&client, &id, extended).await,
        UsersCommands::Watched { id, type_ } => {
            if type_ == "movies" { api::Api::get_user_watched(&client, &id, "movies", extended).await } else { api::Api::get_user_watched(&client, &id, "shows", extended).await }
        },
        UsersCommands::Stats { id } => api::Api::get_user_stats(&client, &id).await,
    };
    Ok(output_result(res, cli.json, cli.quiet))
}
