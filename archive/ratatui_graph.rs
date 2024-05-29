use std::process::Command;

#[macro_use]
mod macros;

fn setup() {
    cmd!(clear); // clearコマンドを実行する
    cmd!(utf - 8); // utf-8コマンドを実行する
    cmd!(line) // lineコマンドを実行する
}

use std::{
    io::{self, stdout, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, symbols::scrollbar, widgets::*};

fn main() -> io::Result<()> {
    App::run() // Appのrunメソッドを実行する
}

struct App {
    marker: Marker,
}

impl App {
    fn new() -> Self {
        Self {
            marker: Marker::Braille,
        }
    }

    pub fn run() -> io::Result<()> {
        setup(); // セットアップを実行する
        let mut terminal = init_terminal()?; // ターミナルを初期化する
        let app = Self::new(); // Appを作成する
        let mut last_tick = Instant::now(); // 最後のtickの時間を記録する
        let tick_rate = Duration::from_millis(16); // tickのレートを設定する
        loop {
            let _ = terminal.draw(|frame| app.ui(frame)); // ターミナル上にUIを描画する

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now(); // 最後のtickの時間を更新する
            }
        }
    }

    fn ui(&self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Percentage(0), Constraint::Percentage(100)]);
        let vertical = Layout::vertical([Constraint::Percentage(0), Constraint::Percentage(100)]);
        let [_, right] = horizontal.areas(frame.size());
        let [_, boxes] = vertical.areas(right);

        frame.render_widget(self.boxes_canvas(boxes), boxes); // ボックスキャンバスを描画する
    }

    fn boxes_canvas(&self, area: Rect) -> impl Widget {
        let left = 0.0;
        let right = f64::from(area.width);
        let bottom = 0.0;
        let top = f64::from(area.height).mul_add(2.0, -4.0);
        Canvas::default()
            .block(Block::bordered().title("Rects")) // 枠線付きのタイトルを持つキャンバスを作成する
            .marker(self.marker) // マーカーを設定する
            .x_bounds([left, right]) // x座標の範囲を設定する
            .y_bounds([bottom, top]) // y座標の範囲を設定する
            .paint(|ctx| {
                for i in 0..=20 {
                    ctx.draw(&Rectangle {
                        x: f64::from(i),
                        y: 0.,
                        width: f64::from(1),
                        height: f64::from(i),
                        color: Color::Blue,
                    });
                }
            })
    }
}

fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?; // ローモードを有効にする
    stdout().execute(EnterAlternateScreen)?; // 代替画面に切り替える
    Terminal::new(CrosstermBackend::new(stdout())) // ターミナルを作成する
}
