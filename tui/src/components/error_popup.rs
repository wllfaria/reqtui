use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget, Wrap},
};

pub struct ErrorPopupLayout {
    message_pane: Rect,
    confirmation_pane: Rect,
}

pub struct ErrorPopup<'a> {
    message: String,
    colors: &'a colors::Colors,
}

impl<'a> ErrorPopup<'a> {
    pub fn new(message: String, colors: &'a colors::Colors) -> Self {
        ErrorPopup { message, colors }
    }

    fn build_popup(&self) -> (Paragraph<'_>, Paragraph<'_>) {
        let message = Paragraph::new(self.message.clone().fg(self.colors.normal.red))
            .wrap(Wrap { trim: true });

        let confirmation = Paragraph::new("(O)k".fg(self.colors.normal.green).to_centered_line())
            .wrap(Wrap { trim: true });

        (message, confirmation)
    }

    fn layout(&self, size: &Rect) -> ErrorPopupLayout {
        let size = Rect::new(
            size.x + 2,
            size.y + 2,
            size.width.saturating_sub(4),
            size.height.saturating_sub(4),
        );

        let [message_pane, confirmation_pane] = Layout::default()
            .direction(Direction::Vertical)
            .flex(Flex::SpaceBetween)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .areas(size);

        ErrorPopupLayout {
            message_pane,
            confirmation_pane,
        }
    }
}

impl Widget for ErrorPopup<'_> {
    fn render(self, size: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Clear.render(size, buf);
        let layout = self.layout(&size);
        let (message, confirmation) = self.build_popup();

        let full_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.colors.bright.black.into()))
            .padding(Padding::new(2, 2, 1, 1))
            .bg(self.colors.normal.black.into());

        full_block.render(size, buf);
        message.render(layout.message_pane, buf);
        confirmation.render(layout.confirmation_pane, buf);
    }
}
