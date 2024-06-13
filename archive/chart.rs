use std::vec;

use charming::{
    component::{Axis, Grid, VisualMap},
    datatype::{CompositeValue, DataFrame},
    df,
    element::{AxisLabel, AxisType, Emphasis, Label, Orient, SplitArea, TextStyle, Tooltip},
    series::Heatmap,
    theme::Theme,
    Chart, ImageRenderer,
};

fn create_vector(input: Vec<i32>, rows: isize, cols: isize) -> Vec<Vec<i32>> {
    let mut output = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            let index = i * cols + j;
            if index < input.len().try_into().unwrap() {
                output.push(vec![i as i32, j as i32, input[index as usize]]);
            }
        }
    }
    output
}

fn main() {
    let data = vec![
        96, 99, 98, 97, 97, 95, 96, 96, 94, 91, 90, 95, 94, 93, 96, 94, 94, 88, 97, 92, 92, 92, 90,
        99, 98, 96, 95, 95, 94, 94, 87, 85, 71, 88, 89, 88, 89, 91, 91, 95, 88, 88, 89, 81, 87, 76,
        84, 92, 89, 88, 87, 87, 88, 92, 93, 83, 87, 82, 81, 90, 87, 87, 86, 85, 88, 85, 80, 83, 85,
        83, 84, 79, 82, 82, 83, 77, 85, 75, 68, 74, 82, 77, 76, 77, 71, 75, 74, 73, 73, 90, 80, 69,
        66, 68, 75, 75, 74, 72, 73, 70, 70, 74, 71, 72, 72, 75, 74, 71, 69, 75, 70, 57, 69, 72, 68,
        76, 77, 72, 77, 74, 78, 70, 71, 70, 76, 77, 72, 70, 73, 69, 69, 62, 71, 71, 69, 68, 73, 67,
        68, 65, 68, 69, 68, 65, 90, 76, 68, 68, 65, 61, 65, 61, 62, 60, 67, 48, 58, 68, 65, 69, 63,
        66, 68, 66, 63, 66, 84, 79, 65, 69, 58, 56, 52, 53, 53, 49, 60, 52, 61, 49, 53, 53, 57, 58,
        60, 57, 65, 58, 58, 64, 56, 58, 47, 55, 54, 62, 53, 64, 57, 61, 62, 45, 54, 51, 56, 58, 59,
        58, 64, 59, 61, 58, 59, 60, 53, 59, 49, 55, 53, 59, 57, 55, 62, 49, 52, 51, 58, 57, 58, 55,
        63, 55, 46, 62, 46, 56, 49, 53, 54, 59, 54, 63, 53, 62, 64, 56, 48, 47, 50, 52, 42, 51, 47,
        52, 73, 65, 50, 57, 49, 42, 37, 41, 39, 36, 47, 53, 52, 47, 49, 51, 47, 45, 50, 46, 51, 38,
        33, 39, 43, 44, 43, 38, 41, 34, 37, 39, 39, 33, 32, 35, 41, 36, 38, 42, 38, 37, 39, 41, 46,
        42, 49, 44, 34, 35, 31, 33, 33, 32, 36, 51, 52, 42, 39, 33, 29, 29, 32, 33, 30, 38, 58, 37,
        28, 30, 31, 34, 31, 37, 37, 34, 32, 22, 26, 35, 31, 39, 32, 38, 32, 33, 34, 30, 36, 44, 38,
        36, 34, 34, 30, 26, 26, 22, 31, 21, 34, 35, 34, 29, 32, 30, 37, 35, 29, 30, 57, 35, 24, 28,
        24, 31, 26, 31, 29, 33, 29, 48, 29, 26, 28, 31, 28, 30, 28, 28, 32, 34, 65, 41, 28, 29, 30,
        22, 30, 23, 25, 16, 28, 63, 34, 25, 30, 30, 24, 24, 24, 21, 25, 25, 64, 27, 25, 30, 31, 23,
        23, 23, 15, 22, 25, 48, 32, 18, 25, 21, 26, 30, 25, 21, 27, 28, 21, 27, 26, 29, 29, 27, 26,
        27, 29, 27, 27, 48, 40, 25, 28, 25, 16, 16, 15, 17, 16, 22, 12, 15, 28, 24, 22, 18, 25, 26,
        24, 23, 17, 14, 14, 13, 18, 16, 14, 15, 16, 18, 11, 16, 13, 12, 9, 13, 15, 14, 13, 20, 13,
        24, 11, 4, 8, 5, 12, 8, 12, 9, 9, 10, 5,
    ];

    let vector = create_vector(data, 45, 11);
    let vector: Vec<DataFrame> = vector
        .into_iter()
        .map(|d| {
            df![
                d[1],
                d[0],
                if d[2] == 0 {
                    CompositeValue::from("-")
                } else {
                    CompositeValue::from(d[2])
                }
            ]
        })
        .collect();

    let chart = Chart::new()
        .tooltip(Tooltip::new().position("top"))
        .grid(Grid::new().height("90%").bottom("5%"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec![
                    "ALL", "1~10", "11~20", "21~30", "31~40", "41~50", "51~60", "61~70", "71~80",
                    "81~90", "91~100",
                ])
                //テキストのサイズを20にする
                .split_area(SplitArea::new().show(true))
                .axis_label(AxisLabel::new().interval(0).rotate(0).font_size(50)),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(vec![
                    "16", "9", "38", "41", "39", "40", "32", "17", "33", "34", "42", "15", "1",
                    "10", "47", "11", "2", "24", "6", "29", "4", "5", "12", "18", "43", "44", "45",
                    "7", "19", "26", "22", "8", "13", "23", "46", "3", "35", "28", "27", "36",
                    "31", "30", "37", "25", "20",
                ])
                .split_area(SplitArea::new().show(true))
                .axis_label(AxisLabel::new().interval(0).rotate(0).font_size(50)),
        )
        .visual_map(
            VisualMap::new()
                .min(0)
                .max(100)
                .calculable(true)
                .orient(Orient::Horizontal)
                .left("center")
                .bottom("0%")
                .text_style(TextStyle::new().font_size(40)),
        )
        .series(
            Heatmap::new()
                .name("Punch Card")
                .label(Label::new().show(true).font_size(60))
                .emphasis(Emphasis::new())
                .data(vector),
        );

    let mut renderer = ImageRenderer::new(3000, 4000).theme(Theme::Chalk);
    renderer.save(&chart, "./test.svg").unwrap();
}
