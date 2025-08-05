use core::fmt::Debug;
use std::collections::HashMap;

use anyhow::Result;
use embedded_graphics::prelude::DrawTarget;
use epd_waveshare::color::TriColor;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScreenIdentifier {
    SurfReport24h,
    SurfReportWeek,
}

pub trait Screen<P> {
    fn parse_params(params: &HashMap<String, Value>) -> Result<P>;
    fn from_params(params: &P) -> impl std::future::Future<Output = Result<Box<Self>>> + Send;
    fn draw<D, E>(&self, target: &mut D) -> Result<(), E>
    where
        E: Debug,
        D: DrawTarget<Color = TriColor, Error = E>;
    fn draw_to_qoi<W>(&self, writer: &mut W) -> Result<()>
    where
        W: std::io::Write + std::io::Seek;
}
