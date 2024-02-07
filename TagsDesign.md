Taggable: feeds, entries

A tag is a predicate: does X have tag Y?

## Automatic tags:

- Examples:
	- does feed X have < N entries
	- is feed X's latest successful fetch > X time ago
	- Is entry X older than Y
	- Expression of other tags
	- Custom SQL?

Different "types": how it is calculated.
Generates SQL that selects this tag

## Manual tags

- Entry-level
- Feed-level
	- Option to apply to all entries

## Structs / tables

Tag:
- id, updated_at, created_at,
- title
- type
	- "feed" (manual)
	- "feed_and_entries" (manual for feeds, entries inherit from feed)
	- "entry" (manual)
- config
	- E.g. how much older something is
	- nullable

FeedTag:
- id, updated_at, created_at,
- feed_id
- tag_id

EntryTag:
- id, updated_at, created_at,
- entry_id
- tag_id

## Questions

- Split feed and entry tags?
	- Not doing so allows applying a feed tag to all entries
	- That might perhaps get confusing though
		- E.g. favorite feed vs favorite entry

## Todo MVP

+ Tags in DB
+ Create tag
	- Label
	- Type
		- manual for now
+ List tags
+ FeedTag in DB
- Add tag to feed (FeedTag creation)
- Search that allows filtering by tag
	- Search system with filters
	- Tags filter
## Followup todo

- List feeds for a given tags
- List tags for a given feed
- Batch fetch filtered feed search
- Add entry tag inheritance
	- Requires system for automatic tags
- Add entry manual tags

## Redesign

What's called a "tag" is actually a "filter".
The `tags` crate should be `filter`.
"Tags" should be just the manual ones/
The DB stuff should get rid of the `config` field, and probably `type` as well,
having manual tags be applyable to both feeds and entries always seems good enough for now.

For searching it seems like a good idea to have a custom query system:
- A GUI for building them
- A text version for advanced users
- The system should probably work with with booleans (&&, ||), functions (the filters), and function arguments (tag name, date) for start.

## Followup ideas

- Numerical valued tags, e.g. a scoring system for entries