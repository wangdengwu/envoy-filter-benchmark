use std::path::MAIN_SEPARATOR_STR;

use charming::{Chart, ImageRenderer};
use charming::component::{Axis, Legend, Title};
use charming::element::{AxisLabel, AxisType, Label};
use charming::series::Bar;

use crate::config::EnvConfig;
use crate::parse::FortioReport;

pub struct ChartData {
    pub title: String,
    pub sub_title: String,
    pub x_axis_data: Vec<String>,
    pub series_list: Vec<SeriesList>,
}

pub struct SeriesList {
    pub name: String,
    pub label_show: bool,
    pub data: Vec<f64>,
}

pub fn convert_to_chart_data(fortio_reports: &Vec<FortioReport>, sub_title: String) {
    let mut series_list: Vec<SeriesList> = vec![];
    for fortio_report in fortio_reports {
        let mut p50_p99: Vec<f64> = vec![];
        for percentiles in &fortio_report.DurationHistogram.Percentiles {
            p50_p99.push(format!("{:.2}", percentiles.Value * 1000f64).parse().unwrap());
        }
        series_list.push(SeriesList {
            name: fortio_report.Labels.to_string(),
            label_show: false,
            data: p50_p99,
        })
    }
    report_to_svg(ChartData {
        title: "response time".to_string(),
        sub_title,
        x_axis_data: vec!["P50".to_string(), "P75".to_string(), "P90".to_string(), "P99".to_string(), "P99.9".to_string()],
        series_list,
    })
}

fn report_to_svg(chart_data: ChartData) {
    let mut chart = Chart::new()
        .title(
            Title::new()
                .right(0)
                .text(chart_data.title)
                .subtext(&chart_data.sub_title),
        )
        .legend(Legend::new())
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data(chart_data.x_axis_data),
        )
        .y_axis(Axis::new().axis_label(AxisLabel::new().formatter("{value} ms")).type_(AxisType::Value));
    for series in chart_data.series_list {
        chart = chart.series(Bar::new().label(Label::new().show(series.label_show)).name(series.name).data(series.data));
    }
    let mut renderer = ImageRenderer::new(900, 400);
    let report_svg_dir = EnvConfig::get_instance().report_svg_dir.clone();

    renderer.save(&chart, report_svg_dir + MAIN_SEPARATOR_STR + &chart_data.sub_title + ".svg").unwrap();
}