use std::time::Duration;

use ratatui::{
	DefaultTerminal, 
	Frame,
	style::{Style, Stylize}, 
	text::{Line, Text, ToText}, 
	layout::{Constraint, Layout}, 
	crossterm::event::{self, Event, KeyCode, KeyEventKind}, 
	widgets::{Bar, BarChart, BarGroup, Block, Borders, Clear, Padding, Paragraph}, 
};

use crate::{sort::SortSnapshot, analytics::Analytics, Error, Renderer};

const MAX_BAR_GAP: u16 = 1;
const BAR_WIDTH_MIN: u16 = 1;
const BAR_WIDTH_MAX: u16 = 3;

const HORIZ_PAD: u16 = 4;
const CHART_PAD: u16 = 2;


pub struct Terminal {
	term: DefaultTerminal
}

impl Terminal {
    /* Initialise terminal to use for rendering chart */
	pub fn new() -> Result<Terminal, Error> {
		Ok(Terminal {
			term: ratatui::init()
		})
	}

    /* Destroy chart terminal and return to normal terminal */
	pub fn restore(self) -> Result<(), Error> {
		ratatui::restore();
		Ok(())
	}
}


impl Renderer for Terminal {
	fn render(&mut self, snapshot: SortSnapshot) -> Result<(), Error> {
		self.term.draw(|frame| {
			render_graph(frame, &snapshot).unwrap();
			if snapshot.is_sorted() {
				render_popup(frame, &snapshot);
			}
		})?;
				
		Ok(())
	}

	/* Returns other io error if interrupted */
	fn sleep(&self, duration: Duration) -> Result<(), Error> {
		if event::poll(duration)? {
			if let Event::Key(key) = event::read()? {
				if key.kind == KeyEventKind::Press {
					if let KeyCode::Char('q') | KeyCode::Esc = key.code {
						return Err(Error::Interrupted);
					}
				}
			}
		}

		Ok(())
	}
}


/* Render bar graph */
fn render_graph(frame: &mut Frame, snapshot: &SortSnapshot) -> Result<(), Error> {
	let data = snapshot.get_data();
	let sort_type = snapshot.get_sort_type();
	
	/* Calculate bar width and gaps */
	let (bar_width, bar_gap) = bar_sizes(frame.area().width, data.len())?;

	/* Chart Width = n * (width + gap) - extra gap + padding */
	let chart_width = (data.len() as u16 * (bar_width + bar_gap)) - bar_gap + CHART_PAD;
	
	/* Set up layout of chart - set width and center */
	let [_, area, _] = Layout::horizontal([
			Constraint::Fill(1),
			Constraint::Length(chart_width + HORIZ_PAD), 
			Constraint::Fill(1),
		]).vertical_margin(5) 
		.areas(frame.area());
	
	/* Set up containing block */
	let block = Block::default()
		.title(Line::styled(sort_type.to_string(), sort_type.color()).bold())
		.padding(Padding::new(HORIZ_PAD / 2, HORIZ_PAD / 2, 2, 0))
		.borders(Borders::ALL);

	/* Set up bar chart */
	let bar_chart = BarChart::default()
		.block(block)
		.bar_style(Style::default().fg(sort_type.color()))
		.bar_width(bar_width)
		.bar_gap(bar_gap)
		.data(build_bars(bar_width, &data));

	/* Render bar chart with set area */
	frame.render_widget(bar_chart, area);

	Ok(())
}


/* Render popup to show sorted */
fn render_popup(frame: &mut Frame, snapshot: &SortSnapshot) {
	let sort_type = snapshot.get_sort_type();

	let [_, horiz_area, _] = Layout::horizontal([
			Constraint::Min(0), 
			Constraint::Percentage(20), 
			Constraint::Min(0)
		]).areas(frame.area());

	let [_, popup_area, _] = Layout::vertical([
			Constraint::Min(0), 
			Constraint::Percentage(25), 
			Constraint::Min(0)
		]).areas(horiz_area);

	/* Clear popup area */
	frame.render_widget(Clear, popup_area);
	
	/* Set up containing block */
	let block = Block::default().borders(Borders::ALL);

	let mut text = Text::from(vec![
		Line::styled("Sorted!", sort_type.color()).bold(),
		Line::styled(format!("{}", snapshot.get_count()), sort_type.color()),
		Line::raw(""),
	]);
	
	let analytics: Analytics = sort_type.analytics();
	text.extend(analytics.to_text().lines.into_iter());

 	let popup = Paragraph::new(text)
		.block(block)
		.centered();

	frame.render_widget(popup, popup_area)
}


/* Build a bar from value */
fn bar<'a>(value: u64, max_pows: u32, bar_width: u16) -> Bar<'a> {
	let format_val = |x: u64| (max_pows <= bar_width as u32)
		.then_some(x.to_string())
		.unwrap_or(String::from(""));

	Bar::default()
		.value(value)
		.text_value(String::from(""))
		.label(Line::from(format_val(value)))
}

/* Build group of bars from the data */
fn build_bars(bar_width: u16, data: &Vec<u64>) -> BarGroup {
	let max: usize = data.len() + 1;
	let max_pows: u32 = max.ilog10() + 1 as u32;
	
	BarGroup::default().bars(
		&data.iter()
			.map(|value| bar(*value, max_pows, bar_width))
			.collect::<Vec<Bar>>()
	)
}

/* Calculate width and gap of bars */
fn bar_sizes(term_width: u16, quantity: usize) -> Result<(u16, u16), Error> {
	let usable_term_width = term_width - HORIZ_PAD - CHART_PAD;
	let mut bar_gap: u16 = MAX_BAR_GAP;
	
	loop {
		/* Width of all gaps */
		let total_gap_width: u16 = bar_gap * (quantity - 1) as u16;

		/* If terminal width - total gaps > 0 */
		if let Some(all_bars_width) = usable_term_width.checked_sub(total_gap_width) {
			/* Calculate individual bar width */
			let bar_width: f32 = all_bars_width as f32 / (quantity as f32);
			
			/* If bar width is valid, clamp and return */
			if bar_width >= 1.0 {
				let clamped_bar_width = (bar_width.floor() as u16).clamp(BAR_WIDTH_MIN, BAR_WIDTH_MAX);
				return Ok((clamped_bar_width, bar_gap));
			}
		} 
		
		/* If gap is invalid, minus 1 and reattempt */
		bar_gap = bar_gap.checked_sub(1).ok_or(Error::BarOverflow(quantity))?;
	}
}
