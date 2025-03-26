use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{BarChart, Block, Borders, List, ListItem, Paragraph, Wrap},
};

use super::app::App;

pub fn render(f: &mut Frame, app: &mut App) {
    // Create main layout with left pane for users and right pane for stats
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    render_users_list(f, app, chunks[0]);
    render_user_stats(f, app, chunks[1]);
}

fn render_users_list(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .users
        .iter()
        .map(|user| {
            ListItem::new(format!("{} <{}>", user.name, user.email)).style(Style::default())
        })
        .collect();

    let users_list = List::new(items)
        .block(
            Block::default()
                .title("Users")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if app.focus_users {
                    Color::Yellow
                } else {
                    Color::White
                })),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(
        users_list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(app.selected_user_index),
    );
}

fn render_user_stats(f: &mut Frame, app: &App, area: Rect) {
    let selected_user = match app.selected_user() {
        Some(user) => user,
        None => {
            let paragraph = Paragraph::new("No user selected")
                .block(Block::default().title("Stats").borders(Borders::ALL))
                .wrap(Wrap { trim: true });
            f.render_widget(paragraph, area);
            return;
        }
    };

    // Split the right area into upper (stats) and lower (charts) sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(area);

    let user_stats = &selected_user.stats;

    // Build stats text
    let stats_text = vec![
        Line::from(vec![Span::styled(
            format!("User: {} <{}>", selected_user.name, selected_user.email),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(format!("Total commits: {}", user_stats.total_commits)),
        Line::from(format!(
            "Avg commits per day: {:.2}",
            user_stats.average_commits_per_day
        )),
        Line::from(format!(
            "Avg commits per month: {:.2}",
            user_stats.average_commits_per_month
        )),
        Line::from(format!(
            "Avg commits per year: {:.2}",
            user_stats.average_commits_per_year
        )),
        Line::from(""),
        Line::from(format!(
            "Total lines added: {}",
            user_stats.total_lines_added
        )),
        Line::from(format!(
            "Total lines removed: {}",
            user_stats.total_lines_removed
        )),
        Line::from(format!(
            "Avg lines added per day: {:.2}",
            user_stats.average_lines_added_per_day
        )),
        Line::from(format!(
            "Avg lines removed per day: {:.2}",
            user_stats.average_lines_removed_per_day
        )),
    ];

    let stats_widget = Paragraph::new(stats_text)
        .block(Block::default().title("Statistics").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(stats_widget, chunks[0]);

    // Create charts section with multiple visualizations
    let charts_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    // Hold the year data as strings for later use
    let year_strings: Vec<String> = user_stats
        .line_changes_per_year
        .keys()
        .map(|year| year.to_string())
        .collect();
    // Render a bar chart for line changes by year
    let year_data: Vec<(&str, u64)> = user_stats
        .line_changes_per_year
        .iter()
        .enumerate()
        .map(|(i, (_, changes))| (year_strings[i].as_str(), *changes as u64))
        .collect();

    let bar_chart = BarChart::default()
        .block(
            Block::default()
                .title("Changes by Year")
                .borders(Borders::ALL),
        )
        .data(&year_data)
        .bar_width(9)
        .bar_gap(2)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(Style::default().fg(Color::Black).bg(Color::Green));

    f.render_widget(bar_chart, charts_chunks[0]);

    // Render a contribution percentage section
    let total_project_commits: u32 = app.users.iter().map(|u| u.stats.total_commits).sum();
    let contribution_percentage = if total_project_commits > 0 {
        (selected_user.stats.total_commits as f64 / total_project_commits as f64) * 100.0
    } else {
        0.0
    };

    let contribution_text = vec![
        Line::from(vec![Span::styled(
            "Contribution Percentage",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(format!("{:.2}% of total commits", contribution_percentage)),
        Line::from(""),
        Line::from(render_percentage_bar(contribution_percentage as usize)),
    ];

    let contribution_widget = Paragraph::new(contribution_text)
        .block(
            Block::default()
                .title("Project Contribution")
                .borders(Borders::ALL),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(contribution_widget, charts_chunks[1]);
}

fn render_percentage_bar(percentage: usize) -> String {
    let width = 20;
    let filled = (percentage * width) / 100;
    let empty = width - filled;

    let filled_part = "█".repeat(filled);
    let empty_part = "░".repeat(empty);

    format!("[{}{}] {}%", filled_part, empty_part, percentage)
}
