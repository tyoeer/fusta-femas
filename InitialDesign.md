# Full Stack Feed Management System

- Read multiple input feeds
	- Multiple input types
		- Videos
		- Webcomics
		- Blogs
	- Read options
		- YT-DLP
		- RSS
		- Repeatedly click "previous" button
		- Repeatedly click "next" from last known entry
	- Combine entries
		- Derive a feed-specific id
- Aggregate inputs into output feeds
	- Tag input feeds
	- Output feeds are tags
	- Tag entries?
		- How to handle untagging a tag gained from the feed?
	- Auto-tag system
		- Based on: title, duration, other tags
- Provide ways of easily viewing output feeds
	- iframe based wrapper like comic-rocket
- Error management:
	- Report extensive fetch error details
	- List feed with latest entry long ago
	- List feeds with last succesful fetch long ago
	- Update URLs in case of 3xx Moved permanently
		- Allow manual moves?
	- Run integrity checks button

## Terminology

- To consume an entry: e.g. to read a blog/strip, watch a video
- Input feed: A source of consumable entries. E.g. a YT channel, an RSS feed
- Output feed: A list of entries for the user to consume
- Strategy/feed type: how to fetch entries for a given input feed

## Crates to use?:

- tokio
- leptos
- sea-orm
- color-eyre?: Also deals with SpanTraces, but might emit to much to easily store in the DB
- reqwest? For fetching rss/non yt
- https://crates.io/crates/youtube_dl ?


## Schema

- *:
	- updated_at: timestamp, use trigger
	- created_at: timestamp use default

- feed
	- id
	- url
	- type/strategy
		- how to fetch entries?
		- e.g. yt-dlp, rss, repeated clicking
	- substrategy?
		- repeated click from which direction
	- Constraints:
		- UNIQUE url
		- strategy accepts URL: probably has to be checked in server runtime
- entry
	- id
	- feed_id: foreign key
	- feed\_entry\_id: feed specific id for easy equality checking/to allow updating old entries
	- view_url: URL to view this entry at for humans
	- embed_url?: just the entry itself, e.g. direct link to image for webcomic, yt embed URL
	- viewed: bool, if this has been viewed
	- duration?: Might be also aplicable to blogs, but such information is usuaaly not included an probably non-standard
	- Constraints:
		- UNIQUE(feed_id, feed_entry_id) / No duplicate feed_entry_ids for a single feed_id

- fetch
	- id
	- feed_id: foreign key
	- fetched_at: timestamp
	- status: enum: Success/Network Error/Parse Error/Other Error?
	- content: response body on success
	- error: stacktrace of error
		- Separate field so content is also available in case of parse errors
	- fetch_url: to keep correct info even if the URL of the feed was updated
	- method/type/strategy: how it was fetched, e.g. Reqwest, yt-dl, same reason as fetch_url
	- Constraints:
		- if status == Success then content != null end
		- if status == Success then error == null end
		- if status != success then error != null end

- tag
	- id
	- name: string
	- Constraints:
		- UNIQUE name

- feed_tag
	- feed_id
	- tag_ig

- entry_tag_owned
	- entry_id
	- tag_id

- entry_tag: View. Combines feed and entry tags into a nice lookup
	```sql
	SELECT entry_id, tag_id
	FROM entry_tag_owned
	UNION
	SELECT entry.entry_id as entry_id, feed_tag.tag_id
	FROM entry
	INNER JOIN feed_tag ON feed_tag.feed_id = entry.feed_id
	```

## Architecture

Plugin system for:
- Backends / fetching strategies
	- Registered per strategy name
	- Validate URLs
	- Fetch new entries
	- Merge entries when no feed\_entry\_id?

- Migrations crate
- Entities crate
- "Backend" crate
	- Gets data from the internet into our DB
	- Can be used as a dependency for plugins
- "Frontend" crate
	- Feed consumption by the user
		- Data could in theory perhaps be deleted afterwards
	- Comic-rocket style frame that reads from an output feed
- "Midend" crate
	- Data management
	- Add/remove feeds
	- Add/remove tags
	- Add/remove auto-tag rules
	- Run integrity checks
- App crate
	- Combines everything
	- UI for misc. stuff

	
	
	
	
	
	
	
	
	
	
	
	
	
