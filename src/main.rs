///
/// Custom DatePicker Widget
/// 
mod date_picker {
    use iced_graphics::{Backend, Defaults, Primitive, Renderer};
    use iced_native::{
        layout, mouse, Background, Color, Element, Hasher, Layout, Length,
        Point, Size, Widget, Event, Clipboard, Rectangle, Font, HorizontalAlignment,
        VerticalAlignment
    };
    
    use chrono::prelude::*;

    const DAYS_EACH_MONTH: [u32; 13] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 29];
    const WEEK_DAYS: [&str; 7] = ["Mon","Tue","Web","Thu","Fri","Sat","Sun"];

    const HIGH_LIGHT_COLOR: Color = Color{r: 118.0/255.0, g: 179.0/225.0, b: 175.0/255.0, a: 1.0};
    const BACKGROUND_COLOR: Color = Color{r: 241.0/255.0, g: 241.0/255.0, b: 241.0/255.0, a: 1.0}; 
    const BORDER_COLOR: Color = Color{r: 0.0/255.0, g: 0.0/255.0, b: 0.0/255.0, a: 1.0};
    const FIRST_TEXT_COLOR: Color = Color{r: 0.0/255.0, g: 0.0/255.0, b: 0.0/255.0, a: 1.0};
    const SECOND_TEXT_COLOR: Color = Color{r: 200.0/255.0, g: 200.0/255.0, b: 200.0/255.0, a: 1.0}; 

    /// A field that can select a date
    pub struct DatePicker<'a, Message> {
        w: u32,
        h: u32,
        placeholder: String,
        value: String,
        on_change: Box<dyn Fn(String) -> Message>,
        on_focus: Option<Message>,
        padding: Option<u16>,
        size: Option<u16>,
        state: &'a mut State,
    }

    impl<'a, Message> DatePicker<'a, Message> {

        /// create a new [`DatePicker`]
        pub fn new<F>(
            w: u32,
            state: &'a mut State,
            placeholder: &str,
            value: &str,
            on_change: F,
        ) -> Self 
        where
            F: 'static + Fn(String) -> Message, 
        {
            Self { 
                w,
                h: w,
                state,
                placeholder: String::from(placeholder),
                value: String::from(value),
                on_change: Box::new(on_change),
                on_focus: None,
                padding: None,
                size: None
            }
        }

        /// check if the year is leap year.
        /// leap year: # of days of Feb = 29
        /// not leap year: # of days of Feb = 28
        fn check_leap_year(&self, year: i32) -> bool {
            if year % 100 == 0 { 
                if year % 400 == 0 { true } else { false }
            } else { 
                 if year % 4 == 0 { true } else { false }
            }
        }

        /// how many days of the current month
        fn number_days_month(&self, month: u32, year:i32) -> u32 {
            let days = if month == 2 {
                if self.check_leap_year(year) {
                    &DAYS_EACH_MONTH[12]
                } else {
                    &DAYS_EACH_MONTH[(month - 1) as usize]
                }
            } else {
                &DAYS_EACH_MONTH[(month - 1) as usize]
            };

            *days
        }

        /// how many days of the last month
        fn number_days_last_month(&self, month: u32, year:i32) -> u32 {
            let last_month = if month == 1 {
                12
            } else {
                month - 1
            };
            let new_year = if month == 1 {
                year - 1
            } else {
                year
            };

            return self.number_days_month(last_month, new_year);
        }

        /// how many days of the next month
        fn number_days_next_month(&self, month: u32, year:i32) -> u32 {
            let next_month = if month == 12 {
                1
            } else {
                month + 1
            };
            let new_year = if month == 12 {
                year + 1
            } else {
                year
            };

            return self.number_days_month(next_month, new_year);
        }

        /// format date 2020-06-05
        pub fn format_date(&self) -> String{
            let dt = Utc.ymd(
                self.state.year,
                self.state.month,
                self.state.day);
            dt.format("%Y-%m-%d").to_string()
        }

        /// set the handler when the [`DatePicker`] is
        /// focus
        pub fn on_focus(mut self, msg: Message) -> Self {
            self.on_focus = Some(msg);
            self
        }

        /// Sets the padding of the [`DatePicker`].
        pub fn padding(mut self, padding: u16) -> Self {
            self.padding = Some(padding);
            self
        }
    
        /// Sets the text size of the [`DatePicker`].
        pub fn size(mut self, size: u16) -> Self {
            self.size = Some(size);
            self
        }
    }

    /// The state of a [`DatePicker`]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct State {
        is_pressed: bool,
        pub is_focused: bool,
        year: i32,
        month: u32,
        day: u32,
    }

    impl State {

        /// Creates a new [`State`], representing an unfocus [`DatePicker`]
        /// with current date: Year, Month, Day
        pub fn new() -> State {
            let local: DateTime<Local> = Local::now();
            State {
                is_pressed: false,
                is_focused: false,
                year: local.year(),
                month: local.month(),
                day: local.day(),
            }
        }
    }

    impl<'a, Message, B> Widget<Message, Renderer<B>> for DatePicker<'a, Message>
    where
        B: Backend,
        Message: Clone,
    {
        fn width(&self) -> Length {
            Length::from(self.w as u16)
        }

        fn height(&self) -> Length {
            Length::from(self.h as u16)
        }

        /// Layout function
        /// Change when the focus event and unfocus event happen
        /// return the frame of the widget
        fn layout(
            &self,
            _renderer: &Renderer<B>,
            limits: &layout::Limits,
        ) -> layout::Node {
            let padding = self.padding.unwrap_or(10) as f32;
            let text_size = self.size.unwrap_or(20);

            let limits = limits
                .pad(padding)
                .width(Length::Units((self.w as f32 - 2.0 * padding) as u16))
                .max_width(self.w)
                .height(Length::Units(text_size));

            let mut text = layout::Node::new(limits.resolve(Size::ZERO));
            text.move_to(Point::new(padding, padding));

            let input = layout::Node::with_children(text.size().pad(padding), vec![text]);
            let mut calendar = layout::Node::new(Size::new(
                self.w as f32,
                self.h as f32 / 7.0 * 8.0
            ));
            calendar.move_to(Point::new(0.0, input.bounds().height));

            layout::Node::with_children(Size::new(
                self.w as f32,
                if self.state.is_focused {
                    input.bounds().height + calendar.bounds().height
                } else {
                    input.bounds().height
                }
            ), if self.state.is_focused {
                vec![input, calendar]
            } else {
                vec![input]
            })
        }

        /// through the hash values of several properties
        /// to decide if need to relayout the most bottom view
        fn hash_layout(&self, state: &mut Hasher) {
            use std::hash::Hash;
            struct Marker;
            std::any::TypeId::of::<Marker>().hash(state);

            self.w.hash(state);
            self.h.hash(state);
            self.padding.hash(state);
            self.size.hash(state);
            self.state.is_focused.hash(state);
        }

        /// According the bound of the layout node,
        /// Draw the view
        fn draw(
            &self,
            _renderer: &mut Renderer<B>,
            _defaults: &Defaults,
            layout: Layout<'_>,
            _cursor_position: Point,
        ) -> (Primitive, mouse::Interaction) {

            let is_mouse_over = layout.bounds().contains(_cursor_position);
            let mut v = Vec::new();

            // Draw the most bottom view: Background
            {
                let bound = layout.bounds();
                v.push(Primitive::Quad {
                    bounds: bound,
                    background: Background::Color(Color::WHITE),
                    border_radius: 0,
                    border_width: 0,
                    border_color: Color::TRANSPARENT,
                });
            }

            for (i, child) in layout.children().enumerate() {

                let bound = child.bounds();
                if i == 0 {
                    // Draw the input view and text view 
                    v.push(Primitive::Quad {
                        bounds: bound,
                        background: Background::Color(Color::WHITE),
                        border_radius: 5,
                        border_width: 1,
                        border_color: BORDER_COLOR,
                    });

                    for chi in child.children() {
                        v.push(Primitive::Text {
                            content: if self.value.is_empty() {
                                self.placeholder.clone()
                            } else {
                                self.value.clone()
                            },
                            color: if self.value.is_empty() {
                                SECOND_TEXT_COLOR
                            } else {
                                FIRST_TEXT_COLOR 
                            },
                            font: Font::default(),
                            bounds: Rectangle {
                                y: chi.bounds().center_y(),
                                width: f32::INFINITY,
                                ..chi.bounds()
                            },
                            size: self.size.unwrap_or(20) as f32,
                            horizontal_alignment: HorizontalAlignment::Left,
                            vertical_alignment: VerticalAlignment::Center,
                        });
                    }
                } else if i == 1 {

                    // Draw the calendar view
                    let size = bound.width/7.0;
                    let font1 = 24.0/(400.0/7.0) * size;
                    let font2 = 36.0/(400.0/7.0) * size;

                    // Draw a background
                    v.push(Primitive::Quad {
                        bounds: bound,
                        background: Background::Color(Color::WHITE),
                        border_radius: 0,
                        border_width: 1,
                        border_color: BORDER_COLOR,
                    });

                    // Draw the weekdays menu
                    // Mon, Tue, Web, Thu, Fri, Sat, Sun
                    for weekday in 0..7 {
                        let b = Rectangle {
                            x: bound.x + weekday as f32 * size,
                            y: bound.y,
                            width: bound.width/7.0,
                            height: bound.width/7.0,
                        };

                        let x = b.center_x();
                        let y = b.center_y();
                        
                        v.push(Primitive::Quad {
                            bounds: b,
                            background: Background::Color(BACKGROUND_COLOR),
                            border_radius: 0,
                            border_width: 1,
                            border_color: BORDER_COLOR,
                        });

                        v.push(Primitive::Text {
                            content: WEEK_DAYS[weekday].to_string(),
                            bounds: Rectangle { x, y, ..b },
                            color: FIRST_TEXT_COLOR,
                            size: font1,
                            font: Font::default(),
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                        }) 
                    }

                    // first day of the current month
                    let dt = Utc.ymd(self.state.year, self.state.month, 1);

                    // weekday of the first day
                    let weekday = dt.weekday().num_days_from_monday();

                    // # of days of current month
                    let days = self.number_days_month(self.state.month,
                        self.state.year);

                    // # of days of last month
                    let last_month_days = self.number_days_last_month(
                        self.state.month, self.state.year);

                    // Draw the days of last month
                    for day in 1..weekday + 1 {
                        let temp = last_month_days - (weekday - day);
                        let column = day - 1;

                        let b = Rectangle {
                            x: bound.x + column as f32 * size,
                            y: bound.y + size,
                            width: size,
                            height: size,
                        };

                        v.push(Primitive::Quad {
                            bounds: b,
                            background: Background::Color(BACKGROUND_COLOR),
                            border_radius: 0,
                            border_width: 1,
                            border_color: BORDER_COLOR,
                        }); 

                        let x = b.center_x();
                        let y = b.center_y();

                        v.push(Primitive::Text {
                            content: temp.to_string(),
                            bounds: Rectangle { x, y, ..b},
                            color: SECOND_TEXT_COLOR,
                            size: font2,
                            font: Font::default(),
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                        })
                    }

                    // Draw the days of current month
                    for day in 1..(days + 1){
                        let temp = day + weekday - 1;
                        let row = temp / 7 + 1;
                        let column = temp % 7; 

                        let color = if self.state.day == day {
                            HIGH_LIGHT_COLOR
                        } else {
                            BACKGROUND_COLOR
                        };

                        let b = Rectangle {
                            x: bound.x + column as f32 * size,
                            y: bound.y + row as f32 * size,
                            width: size,
                            height: size,
                        };

                        v.push(Primitive::Quad {
                            bounds: b,
                            background: Background::Color(color),
                            border_radius: 0,
                            border_width: 1,
                            border_color: BORDER_COLOR,
                        });
                        
                        let x = b.center_x();
                        let y = b.center_y();

                        v.push(Primitive::Text {
                            content: day.to_string(),
                            bounds: Rectangle { x, y, ..b},
                            color: FIRST_TEXT_COLOR,
                            size: font2,
                            font: Font::default(),
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                        })
                    }   

                    // Draw the days of next month
                    for day in weekday + days..42 {
                        let temp = day - weekday - days + 1;
                        let row = day / 7 + 1;
                        let column = day % 7;  

                        let b = Rectangle {
                            x: bound.x + column as f32 * size,
                            y: bound.y + row as f32 * size,
                            width: size,
                            height: size,
                        };

                        v.push(Primitive::Quad {
                            bounds: b,
                            background: Background::Color(BACKGROUND_COLOR),
                            border_radius: 0,
                            border_width: 1,
                            border_color: BORDER_COLOR,
                        }); 

                        let x = b.center_x();
                        let y = b.center_y();

                        v.push(Primitive::Text {
                            content: temp.to_string(),
                            bounds: Rectangle { x, y, ..b},
                            color: SECOND_TEXT_COLOR,
                            size: font2,
                            font: Font::default(),
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                        })
                    }

                    // Draw the select date label on the bottom
                    {
                        let b = Rectangle {
                            x: bound.x + 2.0 * size,
                            y: bound.y + bound.height - size,
                            width: 3.0 * size,
                            height: size,
                        }; 
                        
                        let x = b.center_x();
                        let y = b.center_y();

                        v.push(Primitive::Text {
                            content: self.format_date(),
                            bounds: Rectangle { x, y, ..b },
                            color: FIRST_TEXT_COLOR,
                            size: font1,
                            font: Font::default(),
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                        });
                    }

                    // Draw the control button on the bottom
                    // Pre and Next operation
                    {
                        let mut b = Rectangle {
                            x: bound.x,
                            y: bound.y + bound.height - size,
                            width: 2.0 * size,
                            height: size,
                        }; 
                        
                        v.push(Primitive::Quad {
                            bounds: b,
                            background: Background::Color(HIGH_LIGHT_COLOR),
                            border_radius: 0,
                            border_width: 1,
                            border_color: BORDER_COLOR,
                        });

                        let mut x = b.center_x();
                        let mut y = b.center_y();

                        v.push(Primitive::Text {
                            content: String::from("Pre"),
                            bounds: Rectangle { x, y, ..b },
                            color: FIRST_TEXT_COLOR,
                            size: font1,
                            font: Font::default(),
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                        });

                        b.x = bound.x + 5.0 * size;

                        v.push(Primitive::Quad {
                            bounds: b,
                            background: Background::Color(HIGH_LIGHT_COLOR),
                            border_radius: 0,
                            border_width: 1,
                            border_color: BORDER_COLOR,
                        });

                        x = b.center_x();
                        y = b.center_y();

                        v.push(Primitive::Text {
                            content: String::from("Next"),
                            bounds: Rectangle {x, y, ..b},
                            color: FIRST_TEXT_COLOR,
                            size: font1,
                            font: Font::default(),
                            horizontal_alignment: HorizontalAlignment::Center,
                            vertical_alignment: VerticalAlignment::Center,
                        });
                    }
                }
            }
            
            (
                Primitive::Group{
                    primitives: v
                },
                if is_mouse_over {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                },
            )
        }

        // listen all the event on the window
        fn on_event(
            &mut self,
            event: Event,
            layout: Layout<'_>,
            cursor_position: Point,
            messages: &mut Vec<Message>,
            _renderer: &Renderer<B>,
            _clipboard: Option<&dyn Clipboard>,
        ) {
            let size = layout.bounds().width/7.0; 
            match event {

                // listen press event
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {

                    let bounds = layout.bounds();
                    self.state.is_pressed = bounds.contains(cursor_position);
                }

                // listen released event
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {

                    let bounds = layout.bounds();
                    let is_clicked = self.state.is_pressed
                        && bounds.contains(cursor_position);
                    self.state.is_pressed = false;

                    // if the click position is in the region of the widget
                    if is_clicked {

                        // if click in the input area
                        let input_height = (self.padding.unwrap_or(10) * 2 +
                            self.size.unwrap_or(20)) as f32;
                        if cursor_position.y < layout.bounds().y + input_height  {

                            // send message to change the [state.is_focus]
                            if let Some(on_focus) = self.on_focus.clone() {
                                messages.push(on_focus);
                            }
                            return
                        }

                        // if click in the calendar area
                        let column = ((cursor_position.x - layout.bounds().x)/size).ceil() as i32;
                        let row = ((cursor_position.y - layout.bounds().y - input_height)/size).ceil() as i32;

                        let dt = Utc.ymd(self.state.year, self.state.month, 1);
                        let weekday = dt.weekday().num_days_from_monday() as u32;

                        // if click in the date area in the calendar
                        if row > 1 && row < 8{

                            let days = self.number_days_month(
                                self.state.month, self.state.year);
                            let day = ((row - 2) * 7 + column) as u32;

                            // if click the date in the current month
                            if day > weekday && day <= days + weekday {

                                self.state.day = day - weekday;
                            } else if day <= weekday {

                                // if click the date in the last month
                                let last_month_days = self.number_days_last_month(
                                    self.state.month, self.state.year);

                                if self.state.month == 1 {

                                    self.state.year = self.state.year - 1;
                                    self.state.month = 12;
                                } else {

                                    self.state.month = self.state.month - 1;
                                }
                                self.state.day = last_month_days - (weekday - day);
                            } else if day > days + weekday {

                                // if click the date in the next month
                                if self.state.month == 12 {

                                    self.state.year = self.state.year + 1;
                                    self.state.month = 1;
                                } else {

                                    self.state.month = self.state.month + 1; 
                                }

                                self.state.day = day - weekday - days;
                            }

                            let message = (self.on_change)(self.format_date());
                            messages.push(message);
                        } else if row == 8 {

                            // if click in the control area in the calendar
                            if column == 1 || column == 2 {

                                // if click pre button
                                let last_month_days = self.number_days_last_month(
                                    self.state.month, self.state.year);

                                if self.state.month == 1 {

                                    self.state.year = self.state.year - 1;
                                    self.state.month = 12;
                                } else {

                                    self.state.month = self.state.month - 1;
                                }

                                // if last month doesn't have current day
                                if self.state.day > last_month_days {

                                    self.state.day = last_month_days;
                                }
                            } else if column == 6 || column == 7 {
                                
                                // if click the next button
                                let next_month_days = self.number_days_next_month(
                                    self.state.month, self.state.year); 

                                if self.state.month == 12 {

                                    self.state.year = self.state.year + 1;
                                    self.state.month = 1;
                                } else {

                                    self.state.month = self.state.month + 1; 
                                }

                                // if next month doesn't have current day
                                if self.state.day > next_month_days {

                                    self.state.day = next_month_days;
                                }
                            }
                        }
                    } else {
                        
                        // if click on the area outside of the widget
                        if let Some(on_focus) = self.on_focus.clone() {
                            if self.state.is_focused {
                                messages.push(on_focus);
                            }
                        }
                        return 
                    }
                }
                _ => {}
            }
        }
    }

    impl<'a, Message, B> Into<Element<'a, Message, Renderer<B>>> for DatePicker<'a, Message>
    where
        B: Backend,
        Message: 'a + Clone,
    {
        fn into(self) -> Element<'a, Message, Renderer<B>> {
            Element::new(self)
        }
    }
}



/////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////



use date_picker::DatePicker;
use iced::{
    Align, Column, Container, Element, Length, Sandbox, Settings, Text, 
};

pub fn main() {
    Example::run(Settings::default())
}

struct Example {
    width: u32,
    date_picker: date_picker::State,
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    DatePickerfocus,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            width: 320,
            date_picker: date_picker::State::new(),
            input_value: String::default(),
        }
    }

    fn title(&self) -> String {
        String::from("Custom widget - DatePicker")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                self.date_picker.is_focused = !self.date_picker.is_focused;
            },
            Message::DatePickerfocus => self.date_picker.is_focused = !self.date_picker.is_focused,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Align::Center)
            .push(DatePicker::new(
                self.width, 
                &mut self.date_picker,
                "Choose a date...",
                &self.input_value,
                Message::InputChanged
            ).padding(10)
            .size(30)
            .on_focus(Message::DatePickerfocus))
            .push(Text::new(format!("Width: {}", self.width.to_string())));
        
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .align_y(Align::Start)
            .into()
    }
}
