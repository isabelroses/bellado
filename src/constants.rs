use lazy_static::lazy_static;
use prettytable::{format, row, Row};

lazy_static! {
    pub static ref HEADER: Row = row![
        "ID",
        "Task",
        "Categories",
        "Created At",
        "Completed At",
        "Completed"
    ];
    pub static ref PADDING_FORMAT: format::TableFormat =
        format::FormatBuilder::new().padding(1, 1).build();
}
