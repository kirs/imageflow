#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}


mod nodes {
    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct Decode {
        pub io_id: i32
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub enum Encoder{
        Png,
        Png24,
        Png8,
        Jpeg

    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub struct Encode {
        pub io_id: i32,
        pub encoder: Option<Encoder>
    }

    #[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
    pub enum AnyNode {
        Decode(Decode),
        Encode(Encode),
    }

}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum MNode {
    Decode{io_id: i32},
    Encode{io_id: i32, encoder: Option<nodes::Encoder>},
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum PixelFormat{
    Bgra32, Bgr24, Gray8
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Encoder{
    Png,
    Jpeg
}



#[repr(C)]
#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub enum Filter {
    RobidouxFast = 1,
    Robidoux = 2,
    RobidouxSharp = 3,
    Ginseng = 4,
    GinsengSharp = 5,
    Lanczos = 6,
    LanczosSharp = 7,
    Lanczos2 = 8,
    Lanczos2Sharp = 9,
    CubicFast = 10,
    Cubic = 11,
    CubicSharp = 12,
    CatmullRom = 13,
    Mitchell = 14,

    CubicBSpline = 15,
    Hermite = 16,
    Jinc = 17,
    RawLanczos3 = 18,
    RawLanczos3Sharp = 19,
    RawLanczos2 = 20,
    RawLanczos2Sharp = 21,
    Triangle = 22,
    Linear = 23,
    Box = 24,
    CatmullRomFast = 25,
    CatmullRomFastSharp = 26,

    Fastest = 27,

    MitchellFast = 28,
    NCubic = 29,
    NCubicSharp = 30,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EncoderHints{
    Jpeg{quality: Option<i32>},
    Png{disable_alpha: Option<bool>}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ColorSrgb{
    Hex(String)
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Color{
    Srgb(ColorSrgb)
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Node{
    #[serde(rename="flipV")]
    FlipV,
    #[serde(rename="flipH")]
    FlipH,
    #[serde(rename="crop")]
    Crop{ x1: u32, y1: u32, x2: u32, y2: u32},
    #[serde(rename="createCanvas")]
    CreateCanvas{ format: PixelFormat, w: usize, h: usize, color: Color},
    #[serde(rename="copyRectToCanvas")]
    CopyRectToCanvas {
        #[serde(rename="fromX")]
        from_x: u32,
        #[serde(rename="fromY")]
        from_y: u32, width: u32, height: u32, x: u32, y: u32},
    #[serde(rename="decode")]
    Decode{
        #[serde(rename="ioId")]
        io_id: i32},
    #[serde(rename="encode")]
    Encode{
        #[serde(rename="ioId")]
        io_id: i32, encoder: Option<Encoder>,
        #[serde(rename="encoderId")]
        encoder_id: Option<i64>, hints: Option<EncoderHints> },
    #[serde(rename="fillRect")]
    FillRect {x1: u32, y1: u32, x2: u32, y2: u32, color: Color},
    #[serde(rename="expandCanvas")]
    ExpandCanvas {left: u32, top: u32, right: u32, bottom: u32, color: Color},
    #[serde(rename="transpose")]
    Transpose,
    #[serde(rename="rotate90")]
    Rotate90,
    #[serde(rename="rotate180")]
    Rotae180,
    #[serde(rename="rotate270")]
    Rotate270,
    #[serde(rename="scale")]
    Scale{ w: usize, h: usize,
        #[serde(rename="downFilter")]
        down_filter: Option<Filter>,
        #[serde(rename="upFilter")]
        up_filter: Option<Filter>,
        #[serde(rename="sharpenPercent")]
        sharpen_percent: Option<f32>, flags: Option<usize>}
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum EdgeKind{
    #[serde(rename="input")]
    Input,
    #[serde(rename="canvas")]
    Canvas
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Edge{
    pub from: i32,
    pub to: i32,
    pub kind: EdgeKind
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Graph{
    pub nodes: std::collections::HashMap<u32, Node>,
    pub edges: Vec<Edge>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum TestEnum{
    A,
    B{c: i32}
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum IoDirection {
    #[serde(rename="output")]
    Output = 8,
    #[serde(rename="input")]
    Input = 4,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum IoEnum{
    #[serde(rename="bytesHex")]
    BytesHex(String),
    #[serde(rename="file")]
    Filename(String),
    #[serde(rename="url")]
    Url(String)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]

pub enum IoChecksum{
    #[serde(rename="djb2Hex")]
    Djb2Hex(String)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct IoObject{
    #[serde(rename="ioId")]
    pub io_id: i32,
    pub direction: IoDirection,
    pub io: IoEnum,
    pub checksum: Option<IoChecksum>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Framewise {
    #[serde(rename="graph")]
    Graph(Graph),
    #[serde(rename="steps")]
    Steps(Vec<Node>)
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Build001Config {
    #[serde(rename="enableJpegBlockScaling")]
    pub enable_jpeg_block_scaling: Option<bool>,
    #[serde(rename="processAllGifFrames")]
    pub process_all_gif_frames: Option<bool>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Build001 {
    #[serde(rename="builderConfig")]
    pub  builder_config: Option<Build001Config>,
    pub io: Vec<IoObject>,
    pub framewise: Framewise,
}