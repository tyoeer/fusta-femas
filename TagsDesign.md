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

AutoTag: Tag:
- type
- config
	- E.g. how much older something is

ManualTag: Tag:

FeedTag:
- id, updated_at, created_at,
- feed_id
- manual_tag_id
- applies_to_entries

EntryTag:
- id, updated_at, created_at,
- entry_id
- manual_tag_id

## Questions

- Split feed and entry tags?
	- Not doing so allows applying a feed tag to all entries
	- That might perhaps get confusing though
		- E.g. favorite feed vs favorite entry

## Todo
- Create tag
	- Label
	- Type
		- manual for now
- Add manual tag to feed
- Search that allows filtering by tag

## Followup ideas

- Numerical valued tags, e.g. a scoring system for entries