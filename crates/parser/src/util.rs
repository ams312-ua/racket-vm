use std::fmt::Display;

use ariadne::{Color, Config, IndexType, Label, Report, ReportKind, sources};
use chumsky::error::Rich;



pub fn print_errors<'a, 'file: 'a, I, T>(filename: &'file str, source: &'a str, errors: I)
where
    I: IntoIterator<Item = Rich<'a, T>>,
    T: Display + 'a
{

    // SAFETY: We're telling the compiler to interpret filename as if it had a
    // static lifetime, this is certainly not true, but due to function signatures we need to do
    // this.
    // However this function does not return nor retain any data that could reference
    // the filename after running, so it's safe
    let filename = unsafe {
        std::mem::transmute::<&'file str, &'static str>(filename)
    };
    errors.into_iter().for_each(move |error| {
        Report::build(ReportKind::Error, (filename, error.span().into_range()))
            .with_config(Config::new().with_index_type(IndexType::Byte))
            .with_message(error.to_string())
            .with_label(
                Label::new((filename, error.span().into_range()))
                    .with_message(error.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .print(sources([(
                filename,
                source
            )]))
            .unwrap()
    });
}