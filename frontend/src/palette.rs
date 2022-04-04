pub struct PaletteColor {
    pub name: String,
    pub color: [u8; 3],
}

pub fn create_palette() -> Vec<PaletteColor> {
    vec![
        PaletteColor {
            name: "burgundy".to_string(),
            color: [109, 0, 26],
        },
        PaletteColor {
            name: "dark red".to_string(),
            color: [190, 0, 57],
        },
        PaletteColor {
            name: "red".to_string(),
            color: [255, 69, 0],
        },
        PaletteColor {
            name: "orange".to_string(),
            color: [255, 168, 0],
        },
        PaletteColor {
            name: "yellow".to_string(),
            color: [255, 214, 53],
        },
        PaletteColor {
            name: "pale yellow".to_string(),
            color: [255, 248, 184],
        },
        PaletteColor {
            name: "dark green".to_string(),
            color: [0, 163, 104],
        },
        PaletteColor {
            name: "green".to_string(),
            color: [0, 204, 120],
        },
        PaletteColor {
            name: "light green".to_string(),
            color: [126, 237, 86],
        },
        PaletteColor {
            name: "dark teal".to_string(),
            color: [0, 117, 111],
        },
        PaletteColor {
            name: "teal".to_string(),
            color: [0, 158, 170],
        },
        PaletteColor {
            name: "light teal".to_string(),
            color: [0, 204, 192],
        },
        PaletteColor {
            name: "dark blue".to_string(),
            color: [36, 80, 164],
        },
        PaletteColor {
            name: "blue".to_string(),
            color: [54, 144, 234],
        },
        PaletteColor {
            name: "light blue".to_string(),
            color: [81, 233, 244],
        },
        PaletteColor {
            name: "indigo".to_string(),
            color: [73, 58, 193],
        },
        PaletteColor {
            name: "periwinkle".to_string(),
            color: [106, 92, 255],
        },
        PaletteColor {
            name: "lavender".to_string(),
            color: [148, 179, 255],
        },
        PaletteColor {
            name: "dark purple".to_string(),
            color: [129, 30, 159],
        },
        PaletteColor {
            name: "purple".to_string(),
            color: [180, 74, 192],
        },
        PaletteColor {
            name: "pale purple".to_string(),
            color: [228, 171, 255],
        },
        PaletteColor {
            name: "magenta".to_string(),
            color: [222, 16, 127],
        },
        PaletteColor {
            name: "pink".to_string(),
            color: [255, 56, 129],
        },
        PaletteColor {
            name: "light pink".to_string(),
            color: [255, 153, 170],
        },
        PaletteColor {
            name: "dark brown".to_string(),
            color: [109, 72, 47],
        },
        PaletteColor {
            name: "brown".to_string(),
            color: [156, 105, 38],
        },
        PaletteColor {
            name: "beige".to_string(),
            color: [255, 180, 112],
        },
        PaletteColor {
            name: "black".to_string(),
            color: [0, 0, 0],
        },
        PaletteColor {
            name: "dark gray".to_string(),
            color: [81, 82, 82],
        },
        PaletteColor {
            name: "gray".to_string(),
            color: [137, 141, 144],
        },
        PaletteColor {
            name: "light gray".to_string(),
            color: [212, 215, 217],
        },
        PaletteColor {
            name: "white".to_string(),
            color: [255, 255, 255],
        },
    ]
}
