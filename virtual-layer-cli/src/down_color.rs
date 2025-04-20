use regex::Regex;
use std::{collections::HashMap, sync::LazyLock};

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)\x1b\[(?P<code>\d{1,2})(?:;(?:2;(?P<r>\d{1,3});(?P<g>\d{1,3});(?P<b>\d{1,3})))?m",
    )
    .unwrap()
});

static MAP16: LazyLock<HashMap<u8, (u8, u8, u8)>> = LazyLock::new(|| {
    let mut map16 = HashMap::new();
    map16.insert(30, (0, 0, 0)); // Black
    map16.insert(31, (128, 0, 0)); // Red
    map16.insert(32, (0, 128, 0)); // Green
    map16.insert(33, (128, 128, 0)); // Yellow
    map16.insert(34, (0, 0, 128)); // Blue
    map16.insert(35, (128, 0, 128)); // Magenta
    map16.insert(36, (0, 128, 128)); // Cyan
    map16.insert(37, (192, 192, 192)); // White

    map16
});

pub fn reduce_saturation(input: &str, ratio: f32) -> String {
    // \x1b[38;2;R;G;Bm or \x1b[48;2;R;G;Bm

    RE.replace_all(input, |caps: &regex::Captures| {
        let code: u8 = caps["code"].parse().unwrap_or(0);
        // 真のRGBモードなら直接マッピング
        let (r0, g0, b0) = if let (Some(r_), Some(g_), Some(b_)) =
            (caps.name("r"), caps.name("g"), caps.name("b"))
        {
            (
                r_.as_str().parse::<f32>().unwrap_or(0.0),
                g_.as_str().parse::<f32>().unwrap_or(0.0),
                b_.as_str().parse::<f32>().unwrap_or(0.0),
            )
        } else {
            // 16色モードの場合
            let base = *MAP16.get(&code).unwrap_or(&(0, 0, 0));
            (base.0 as f32, base.1 as f32, base.2 as f32)
        };

        // 彩度調整
        let avg = (r0 + g0 + b0) / 3.0;
        let nr = (avg + (r0 - avg) * ratio).round().clamp(0.0, 255.0) as u8;
        let ng = (avg + (g0 - avg) * ratio).round().clamp(0.0, 255.0) as u8;
        let nb = (avg + (b0 - avg) * ratio).round().clamp(0.0, 255.0) as u8;

        // 常に24bit真RGBで返しますわ
        format!("\x1b[38;2;{};{};{}m", nr, ng, nb)
    })
    .into_owned()
}
