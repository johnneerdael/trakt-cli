---
name: trakt-cli-agent
description: Use this skill when a user wants to use trakt-cli to look up movies, shows, seasons, episodes, people, ratings, stats, calendars, recommendations, lists, comments, or user profiles on Trakt, or to manage authenticated Trakt account data such as watchlist, favorites, collection, history, ratings, notes, follows, hidden items, check-ins, and scrobbles. Especially useful when the user asks in natural language to find something on Trakt, see what is trending or upcoming, inspect metadata, or add, remove, update, rate, follow, comment on, note, check in to, or scrobble a Trakt item.
license: MIT
metadata:
  author: OpenAI
  version: 1.1.0
  category: entertainment
  tags: [trakt, cli, movies, shows, watchlist, recommendations, automation]
---

# trakt-cli Agent Skill

Use this skill to translate natural-language movie and TV requests into reliable `trakt-cli` commands.

## Use this skill when

Use this skill when the user wants to:

- find a movie, show, season, episode, person, comment, list, or user on Trakt
- inspect metadata such as ratings, stats, cast, crew, studios, translations, aliases, release dates, videos, related titles, certifications, or release calendars
- see what is trending, popular, anticipated, recently updated, recommended, airing soon, currently being watched, or releasing soon
- access personalized Trakt data such as watchlist, favorites, collection, playback, history, ratings, hidden items, saved filters, or last activities
- manage authenticated Trakt account state such as adding or removing watchlist items, favorites, ratings, history entries, collection items, list items, follows, comments, notes, check-ins, or scrobbles
- work from a title or name in plain English and resolve the correct Trakt item before acting

## Do not use this skill when

Do not use this skill when:

- the user only wants general entertainment discussion with no need to query or change Trakt data
- the task requires non-Trakt providers such as IMDb, Letterboxd, TMDb, Netflix, Plex, Jellyfin, or local media files unless Trakt is explicitly the system of record
- the user wants filesystem operations, shell scripting help unrelated to `trakt-cli`, or package installation guidance unrelated to using the CLI itself
- the request is too vague to map to Trakt concepts and there is no reasonable item, user, or activity to resolve

## Prerequisites

- `trakt-cli` must be installed and available in the shell.
- Authenticated or personalized commands require a configured Trakt account.
- Commands that take `--item`, `--items`, `--sharing`, or `--rank` require valid JSON.
- Prefer commands that read state first when a write could affect the wrong item.

## Core operating rules

1. Start with discovery when the user gives names, not IDs.
   - Use `trakt-cli search text` when the user gives a title or person name.
   - Use `trakt-cli search id` when the user already provides an external ID type and value.
   - After search, resolve the correct Trakt item before running detail or mutation commands.

2. Prefer read operations before write operations.
   - Inspect the current state before changing it when that reduces mistakes.
   - For example, check whether an item is already in a watchlist before adding, or fetch list items before reordering.

3. Be precise about media level.
   - Use `movies` for films.
   - Use `shows` for series-level data.
   - Use `seasons` and `episodes` only when the user asks for season-specific or episode-specific information.

4. Use user-scoped commands only for personalized data.
   - Use `sync` for the authenticated account's library and activity.
   - Use `users` for public or user-profile-oriented operations on a specific user.
   - Use `calendars my-*` for personalized release schedules and `calendars all-*` for global schedules.

5. Treat mutations as high-attention operations.
   - Double-check item identity before writes.
   - For destructive operations such as delete, remove, unlike, unfollow, reset, or reorder, be explicit about what will change.
   - If the request is ambiguous, resolve the item first rather than guessing.

6. Use structured payloads carefully.
   - Build the smallest valid JSON payload that matches the target item or list of items.
   - Keep payloads type-correct: movies under `movies`, shows under `shows`, episodes under `episodes`, and so on.

## Fast routing hints

- “Find”, “look up”, “show me details for”, “who is in”, “what are the ratings for” → search first, then `get`, `ratings`, `people`, `stats`, or related metadata commands.
- “What should I watch”, “recommend”, “trending”, “popular”, “anticipated”, “what’s new” → `recommendations`, `movies`, `shows`, `lists`, or `calendars`.
- “Add to watchlist”, “favorite this”, “rate”, “mark watched”, “remove from history” → resolve item first, then use `sync` write commands.
- “What is user X watching”, “show user X’s watchlist”, “follow user X” → `users` commands.
- “Check me in”, “start scrobbling”, “stop scrobbling” → `checkin` or `scrobble`.

## Command selection guide

### Discovery and identification

Use these first when the target is not already pinned down.

```bash
trakt-cli search text movie "The Matrix"
trakt-cli search text show "Severance"
trakt-cli search text person "Pedro Pascal"
trakt-cli search id imdb tt0133093 --type movie
```

Use `movies get`, `shows get`, `people get`, `seasons get`, or `episodes get` only after you know the item identity.

### Lookup by content type

#### Movies

Use `movies` for film discovery and metadata:

```bash
trakt-cli movies trending
trakt-cli movies get 603 --extended full
trakt-cli movies ratings 603
trakt-cli movies related 603
trakt-cli movies releases 603 --country us
trakt-cli movies videos 603
```

#### Shows

Use `shows` for series-wide discovery and metadata:

```bash
trakt-cli shows trending
trakt-cli shows get 1390 --extended full
trakt-cli shows progress-watched 1390 --hidden --specials
trakt-cli shows next-episode 1390
trakt-cli shows last-episode 1390
trakt-cli shows people 1390
```

#### Seasons and episodes

Use these only for season- or episode-level requests:

```bash
trakt-cli seasons list 1390 --extended full
trakt-cli seasons get 1390 1
trakt-cli seasons episodes 1390 1
trakt-cli episodes get 1390 1 1
trakt-cli episodes ratings 1390 1 1
trakt-cli episodes comments 1390 1 1 --sort newest
```

#### People

Use `people` for cast and crew profiles and credits:

```bash
trakt-cli people get 12345 --extended full
trakt-cli people movies 12345
trakt-cli people shows 12345
```

### Personalized data

#### Recommendations

```bash
trakt-cli recommendations movies --ignore-collected --ignore-watchlisted
trakt-cli recommendations shows --ignore-collected --ignore-watchlisted
```

Hide an unwanted recommendation only after confirming the resolved item:

```bash
trakt-cli recommendations hide-movie 603
trakt-cli recommendations hide-show 1390
```

#### Calendars

Use calendars for date-bounded release and air schedules:

```bash
trakt-cli calendars my-shows --start-date 2026-03-09 --days 7
trakt-cli calendars my-movies --start-date 2026-03-09 --days 30
trakt-cli calendars all-new-shows --start-date 2026-03-09 --days 14
```

#### Sync for the authenticated user

Use `sync` for the signed-in account:

```bash
trakt-cli sync watchlist --type shows --sort-by added
trakt-cli sync favorites --type movies
trakt-cli sync history --type episodes --start-at 2026-01-01
trakt-cli sync watched shows
trakt-cli sync collection movies
trakt-cli sync playback --type episodes
trakt-cli sync last-activities
```

#### Users

Use `users` for profile-centric requests involving a specific user:

```bash
trakt-cli users profile someuser
trakt-cli users watching someuser
trakt-cli users watchlist someuser --type movies
trakt-cli users history someuser --type shows
trakt-cli users stats someuser
trakt-cli users followers someuser
```

### Community and social features

#### Lists

Use community `lists` for trending or popular lists, and `users ... list-*` for a user's personal lists.

```bash
trakt-cli lists trending --type official
trakt-cli lists popular --type personal
trakt-cli lists get 123
trakt-cli lists items 123 --sort-by rank --sort-how asc
trakt-cli users lists someuser
trakt-cli users get-list someuser favorites
trakt-cli users list-items someuser favorites --type movies
```

#### Comments and notes

Use `comments` for public discussion and `notes` for private user notes.

```bash
trakt-cli comments create --item '{"movie":{"ids":{"trakt":603}}}' --comment "Great rewatch."
trakt-cli comments replies 12345
trakt-cli comments like 12345
trakt-cli notes create --item '{"show":{"ids":{"trakt":1390}}}' --notes "Start before vacation" --privacy private
```

### Activity updates and real-time state

#### Check-in and scrobble

Use `checkin` when the user wants to mark what they are watching now. Use `scrobble` for progress-aware playback events.

```bash
trakt-cli checkin create --item '{"movie":{"ids":{"trakt":603}}}' --message "Rewatch time"
trakt-cli scrobble start --item '{"episode":{"ids":{"trakt":987654}}}' --progress 12.5
trakt-cli scrobble stop --item '{"episode":{"ids":{"trakt":987654}}}' --progress 98.0
```

## Common workflows

### 1. Find a title, then inspect it

When the user asks for details about a title but does not know the Trakt ID:

1. Search by text.
2. Pick the best match using title, year, and type.
3. Fetch details and the requested secondary data.

Example:

```bash
trakt-cli search text show "The Bear"
trakt-cli shows get SHOW_ID --extended full
trakt-cli shows ratings SHOW_ID
trakt-cli shows related SHOW_ID
```

### 2. Add a title to watchlist or favorites

1. Resolve the item by search if needed.
2. Build a minimal `--items` JSON payload.
3. Use the corresponding `sync add-*` command.
4. Optionally verify with `sync watchlist` or `sync favorites`.

Examples:

```bash
trakt-cli sync add-watchlist --items '{"movies":[{"ids":{"trakt":603}}]}'
trakt-cli sync add-favorites --items '{"shows":[{"ids":{"trakt":1390}}]}'
```

### 3. Remove an item from watchlist, favorites, collection, history, or ratings

1. Resolve the item exactly.
2. Build the matching removal payload.
3. Run the relevant remove command.
4. Confirm the affected title in the response.

Examples:

```bash
trakt-cli sync remove-watchlist --items '{"movies":[{"ids":{"trakt":603}}]}'
trakt-cli sync remove-favorites --items '{"shows":[{"ids":{"trakt":1390}}]}'
trakt-cli sync remove-history --items '{"episodes":[{"ids":{"trakt":987654}}]}'
trakt-cli sync remove-ratings --items '{"movies":[{"ids":{"trakt":603}}]}'
```

### 4. Rate something

Ratings use `sync add-ratings` with a JSON payload that includes item identity and rating.

```bash
trakt-cli sync add-ratings --items '{"movies":[{"ids":{"trakt":603},"rating":9}]}'
trakt-cli sync add-ratings --items '{"shows":[{"ids":{"trakt":1390},"rating":10}]}'
```

### 5. Review show progress

For watched or collected progress, use the show-level progress endpoints.

```bash
trakt-cli shows progress-watched 1390 --hidden --specials --count-specials --last-activity
trakt-cli shows progress-collection 1390 --hidden --specials --count-specials --last-activity
```

### 6. Work with personal lists

For a user-owned list:

1. Inspect available lists.
2. Get the target list.
3. Add, remove, reorder, or update items.

```bash
trakt-cli users lists someuser
trakt-cli users add-list-items someuser my-list --items '{"movies":[{"ids":{"trakt":603}}]}'
trakt-cli users update-list-item someuser my-list 555 --notes "Move this up"
trakt-cli users reorder-list-items someuser my-list --rank '[555,556,557]'
```

## JSON payload patterns

Use these templates and replace IDs and values as needed.

### Single movie

```json
{"movies":[{"ids":{"trakt":603}}]}
```

### Single show

```json
{"shows":[{"ids":{"trakt":1390}}]}
```

### Single episode

```json
{"episodes":[{"ids":{"trakt":987654}}]}
```

### Rating payload

```json
{"movies":[{"ids":{"trakt":603},"rating":9}]}
```

### Check-in or scrobble item payload

```json
{"movie":{"ids":{"trakt":603}}}
```

```json
{"episode":{"ids":{"trakt":987654}}}
```

## Mapping natural language to commands

- “What should I watch tonight?”
  - Start with `recommendations movies` or `recommendations shows`.
  - If they want trend-based rather than personalized results, use `movies trending` or `shows trending`.

- “What episodes are coming up this week?”
  - Use `calendars my-shows` for the authenticated user.
  - Use `calendars all-shows` for a global schedule.

- “Show me my watchlist”
  - Use `sync watchlist`.

- “Show Alice’s watchlist”
  - Use `users watchlist alice`.

- “Rate Dune 9 out of 10”
  - Search first if needed, then `sync add-ratings`.

- “Add Severance to my favorites”
  - Search first if needed, then `sync add-favorites`.

- “Who is in this show?”
  - Use `shows people` or `episodes people` depending on requested scope.

- “What am I currently watching?”
  - Use `users watching me-or-target-user` when a user context is requested.
  - Use `sync playback` for active playback progress on the authenticated account.

## Decision rules for ambiguous requests

- If a title search returns multiple plausible matches, prefer the one whose type and year align best with the request.
- If the user says “add it” or “remove it,” anchor the operation to the most recently resolved item in the conversation.
- If the user asks for “watch progress,” prefer `shows progress-watched` for a single show and `sync history` or `sync watched` for account-wide activity.
- If the user asks for “what’s new,” choose between `calendars`, `movies updates`, `shows updates`, `comments recent`, or `lists trending` based on whether they mean releases, metadata updates, discussion, or community curation.

## Never do this

- Do not mutate an item that has not been resolved to a specific Trakt identity.
- Do not guess whether a request should use `sync` or `users`; choose based on authenticated-self versus named-user scope.
- Do not use season or episode commands for a show-level request unless the user specifically asks for that level.
- Do not post a public `comment` when the user wants a private `note`.
- Do not send oversized JSON payloads when a single-item payload is enough.

## Troubleshooting

### Search returns too many or wrong matches

- Narrow by media type first.
- Prefer exact-title matches.
- Use year, known external IDs, or follow-up detail calls to verify the result.

### A write command could affect the wrong item

- Stop and verify the Trakt ID first.
- Re-run search and fetch details before mutating.
- Use the smallest possible JSON payload.

### The user asks for a private note or public comment

- Use `notes` for private annotations.
- Use `comments` for public posts and replies.

### The request mixes authenticated-account and other-user data

- Use `sync` for the signed-in account.
- Use `users` for another user's public profile and public-facing resources.

### The user wants “currently watching” information

- Use `checkin` to set active current watching.
- Use `users watching` to read what a user is currently watching.
- Use `scrobble start` and `scrobble stop` for playback lifecycle updates with progress.

## Response expectations

When reporting results back to the user:

- identify the resolved title, person, list, or user clearly
- include the media type and year when available
- mention the specific command outcome for writes such as added, removed, updated, liked, followed, checked in, or scrobbled
- summarize only the most relevant fields rather than dumping raw CLI output unless raw output is explicitly requested

## Examples

### Example 1: Find a show and get its next episode

User says: “When is the next episode of Severance?”

Actions:
1. Search for the show.
2. Resolve the correct show ID.
3. Run `shows next-episode`.
4. Return the episode title, season and episode number, and scheduled date if present.

Likely commands:

```bash
trakt-cli search text show "Severance"
trakt-cli shows next-episode SHOW_ID
```

### Example 2: Add a movie to watchlist

User says: “Add The Matrix to my watchlist.”

Actions:
1. Search for the movie.
2. Resolve the correct movie ID.
3. Add it with `sync add-watchlist`.
4. Confirm the title that was added.

Likely commands:

```bash
trakt-cli search text movie "The Matrix"
trakt-cli sync add-watchlist --items '{"movies":[{"ids":{"trakt":MOVIE_ID}}]}'
```

### Example 3: Get a user’s recent activity

User says: “Show me what janedoe has been watching lately.”

Actions:
1. Use `users history` for account-wide watched activity.
2. Filter by type if the user only wants movies or shows.
3. Summarize the most recent items returned.

Likely command:

```bash
trakt-cli users history janedoe
```

### Example 4: Reply to a comment

User says: “Reply to comment 12345 and say I agree.”

Likely command:

```bash
trakt-cli comments reply 12345 --comment "I agree."
```

## Final quality checklist

Before finishing:

- Confirm that the chosen command scope matches the media type and user scope.
- Confirm that IDs are resolved before detail or mutation commands.
- Confirm that JSON payloads match the target object type.
- Prefer concise command sequences over broad exploratory calls.
- For writes, state exactly what item was changed.

## Installation for AI Agents

To install `trakt-cli` so it persists across shell restarts and is always available in the `PATH`:

1. Download the appropriate release binary (`trakt-cli-...`) for the host OS and architecture from the [GitHub Releases](https://github.com/johnneerdael/trakt-cli/releases) page.
2. Unzip or extract the downloaded file if necessary.
3. Move the binary into your system's path. For example: `sudo mv trakt-cli /usr/local/bin/trakt-cli` (or `~/.local/bin/trakt-cli`).
4. Make the binary executable: `chmod +x /usr/local/bin/trakt-cli`.
5. Verify the installation by running `trakt-cli --help`. It should now survive shell restarts and be easily accessible.
