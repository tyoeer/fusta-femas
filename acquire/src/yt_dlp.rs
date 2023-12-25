use std::{ffi::OsString, process::Stdio};

use entities::prelude::*;
use sea_orm::*;
use super::strategy::*;

/*
--dateafter does not appear to work with flat-playlist
This works to select a subset.
yt-dlp --break-on-reject --skip-download --dateafter 20230101 --dump-json https://www.youtube.com/@NevasBuildings >test.json
yt-dlp --verbose --break-on-reject --skip-download --playlist-end 10 --dump-json https://www.youtube.com/@NevasBuildings >test.json
*/

pub const YTDLP_DATE_FORMAT: &[time::format_description::FormatItem<'_>] = time::macros::format_description!("[year padding:zero][month padding:zero][day padding:zero]");

time::serde::format_description!(ytdlp_serde_format, Date, YTDLP_DATE_FORMAT);

struct YtdlpCommand {
	date_after: Option<time::Date>,
	playlist_end: Option<u16>,
	url: String,
	verbose: bool
}

impl YtdlpCommand {
	pub fn new(url: String) -> Self {
		Self {
			date_after: None,
			playlist_end: None,
			url,
			verbose: false,
		}
	}
	
	pub fn date_after(&mut self, date_after: time::Date) {
		self.date_after = Some(date_after);
	}
	
	pub fn playlist_end(&mut self, playlist_end: u16) {
		self.playlist_end = Some(playlist_end);
	}
	
	pub fn verbose(&mut self, verbose: bool) {
		self.verbose = verbose;
	}
	
	pub fn get_args(&self) -> Result<Vec<OsString>,time::error::Format> {
		let mut out: Vec<OsString> = vec![
			"--break-on-reject".into(),
			"--simulate".into(),
			"--dump-json".into(),
		];
		
		if self.verbose {
			out.push("--verbose".into());
		}
		if let Some(date) = self.date_after {
			out.push("--dateafter".into());
			out.push(date.format(YTDLP_DATE_FORMAT)?.into());
		}
		if let Some(end) = self.playlist_end {
			out.push("--playlist-end".into());
			out.push(format!("{end}").into());
		}
		
		out.push(self.url.clone().into());
		
		Ok(out)
	}
}


#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct YtdlpVideoInfo {
	id: String,
	title: String,
	webpage_url: String,
	#[serde(with = "ytdlp_serde_format")]
	upload_date: time::Date,
	playable_in_embed: bool,
}

impl From<YtdlpVideoInfo> for EntryInfo {
	fn from(info: YtdlpVideoInfo) -> EntryInfo {
		let mut entry = EntryInfo::new(info.id.clone(), info.title, info.webpage_url, info.upload_date);
		if info.playable_in_embed {
			entry.embed_url(format!("www.youtube-nocookie.com/embed/{}",info.id));
		}
		entry
	}
}

#[derive(Clone, PartialEq, Eq)]
pub enum Limit {
	AfterDate(time::Date),
	Amount(u16),
}

impl Limit {
	fn apply(&self, cmd: &mut YtdlpCommand) {
		match self {
			Self::AfterDate(date) => {
				
				cmd.date_after(*date);
			},
			Self::Amount(amount) => {
				cmd.playlist_end(*amount);
			}
		}
	}
}

#[derive(Clone,PartialEq,Eq)]
pub struct YtDlpStrategy {
	command: String,
	backup_limit: Limit,
}

impl Default for YtDlpStrategy {
	fn default() -> Self {
		Self {
			command: "yt-dlp".into(),
			backup_limit: Limit::Amount(10),
		}
	}
}

#[async_trait::async_trait]
impl Strategy for YtDlpStrategy {
	fn name(&self) -> &'static str {
		"yt-dlp"
	}
	
	async fn fetch(&self, conn: &DatabaseConnection, feed: &feed::Model) -> anyhow::Result<String> {
		let maybe_last_entry = feed.find_related(entry::Entity)
			.order_by_desc(entry::Column::ProducedDate)
			.one(conn).await?;
		
		let mut cmd_args = YtdlpCommand::new(feed.url.clone());
		
		cmd_args.verbose(true);
		
		if let Some(last_entry) = maybe_last_entry {
			cmd_args.date_after(last_entry.produced_date.into());
		} else {
			self.backup_limit.apply(&mut cmd_args);
		};
		
		let mut cmd = tokio::process::Command::new(self.command.clone());
		cmd
			.args(cmd_args.get_args()?)
			.kill_on_drop(true)
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
		;
		
		tracing::info!(?cmd, "Running yt-dlp command to fetch");
		
		let out = cmd.output().await?;
		
		//Formatted like this to get to print out the newlines instead of escaping them
		tracing::info!("yt-dlp stderr:\n{}", String::from_utf8_lossy(&out.stderr));
		
		if !out.status.success() && !matches!(out.status.code(),Some(101))  {
			anyhow::bail!("Process returned non-successful exit code: {}",out.status);
		}
		
		Ok(String::from_utf8(out.stdout)?)
	}
	
	async fn parse(&self, data: &str) -> anyhow::Result<Vec<EntryInfo>> {
		data
			.trim() // remove empty segments at the ends
			.split('\n') // data is newline delimited json
			.filter(|segment| !segment.is_empty()) //empty data turns into an empty segment
			.map(|segment| -> anyhow::Result<EntryInfo> {
				let parse_res = serde_json::from_str::<YtdlpVideoInfo>(segment);
				let context_res = parse_res.map_err(|e| {
					anyhow::Error::from(e).context(format!("While parsing: \"{segment}\""))
				});
				context_res.map(|info| info.into())
			})
			.collect()
	}
}