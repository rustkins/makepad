use crate::makepad_widgets::{text::{color::Color, non_nan::NonNanF32, geom::Point, layout::{Align, Baseline, Style, LayoutParams, Span, LayoutOptions}}, *};

live_design!{
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    pub MyWidget = {{MyWidget}} {
        width: Fill, height: Fill,
        spacing: 7.5,
        align: {x: 0.5, y: 0.5},
        padding: <THEME_MSPACE_2> {}
        label_walk: { width: Fit, height: Fit },
                        
        draw_text: {
            color: (THEME_COLOR_TEXT_DEFAULT)
            text_style: <THEME_FONT_REGULAR> {
                font_size: (THEME_FONT_SIZE_P)
            }
            fn get_color(self) -> vec4 {
                return self.color
            }
        }
                        
        draw_bg: {
            fn pixel(self) -> vec4 {
                return #0
            }
        }
    }
    
    App = {{App}} {
        ui: <Window> {
            show_bg: true
            width: Fill,
            height: Fill,
            window: {
                inner_size: vec2(400, 300)
            },
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #4;
                }
            }
            body = <View> {
                padding: 20
                flow: Down
                widget = <MyWidget> {}
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct MyWidget {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[live]
    draw_text: DrawText2,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    label_walk: Walk,
}

impl Widget for MyWidget {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}
    
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);
        let text = "The quick brown fox jumps over the lazy dog繁😊😔";
        let text = cx.fonts.borrow_mut().get_or_layout(LayoutParams {
            text: text.into(),
            spans: [
                Span {
                    style: Style {
                        font_family_id: "Sans".into(),
                        font_size_in_lpxs: NonNanF32::new(16.0).unwrap(),
                        color: Color::BRIGHT_RED,
                        baseline: Baseline::Alphabetic,
                    },
                    range: 0..10,
                },
                Span {
                    style: Style {
                        font_family_id: "Sans".into(),
                        font_size_in_lpxs: NonNanF32::new(16.0).unwrap(),
                        baseline: Baseline::Top,
                        color: Color::BRIGHT_GREEN,
                    },
                    range: 10..20,
                },
                Span {
                    style: Style {
                        font_family_id: "Sans".into(),
                        font_size_in_lpxs: NonNanF32::new(16.0).unwrap(),
                        baseline: Baseline::Bottom,
                        color: Color::BRIGHT_BLUE,
                    },
                    range: 20..text.len(),
                },
            ].into(),
            options: LayoutOptions {
                max_width_in_lpxs: NonNanF32::new(256.0).unwrap(),
                align: Align::Right,
            },
        });
        self.draw_text.draw_laidout_text(cx, Point::new(50.0, 50.0), &text);
        self.draw_bg.end(cx);
        DrawStep::done()
    }
}
