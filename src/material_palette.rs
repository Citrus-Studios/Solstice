use bevy::{prelude::{Color, Image, Assets, ResMut}, render::render_resource::{Extent3d, TextureDimension, TextureFormat}, math::Vec2, pbr::StandardMaterial};

#[derive(Clone)]
pub struct MaterialPalette {
    pub palette: Vec<FlatMaterial>
}

pub struct CompiledMaterials {
    pub base_color_texture: Image,
    pub emissive_texture: Image,
    pub metallic_roughness_texture: Image,
}

#[derive(Clone, Copy)]
pub struct FlatMaterial {
    pub base_color: Color,
    pub emissive: Color,
    pub metallic: f32,
    pub roughness: f32,
}

// #[test]
// fn test_image() {
//     let mut palette = MaterialPalette::new();

//     palette.push(FlatMaterial::default().metallic(0.75));
//     palette.push(FlatMaterial::default().base_color(Color::RED));

//     let compiled = palette.compile(None);
//     let uv_pos = compiled.get_uv_pos(2);

//     println!("{:?}", uv_pos);
// }

impl Default for FlatMaterial {
    fn default() -> Self {
        FlatMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0),
            emissive: Color::BLACK,
            metallic: 0.01,
            roughness: 0.089,
        }
    }
}

impl FlatMaterial {
    pub fn base_color(mut self, e: Color) -> Self { self.base_color = e; self }
    pub fn emissive(mut self, e: Color) -> Self { self.emissive = e; self }
    pub fn metallic(mut self, e: f32) -> Self { self.metallic = e; self }
    pub fn roughness(mut self, e: f32) -> Self { self.roughness = e; self }
}

impl From<(Color, Color, f32, f32)> for FlatMaterial {
    fn from(tuple: (Color, Color, f32, f32)) -> Self {
        FlatMaterial {
            base_color: tuple.0,
            emissive: tuple.1,
            // blue
            metallic: tuple.2,
            // green
            roughness: tuple.3,
        }
    }
}

impl From<StandardMaterial> for FlatMaterial {
    fn from(mat: StandardMaterial) -> Self {
        FlatMaterial::default().base_color(mat.base_color).emissive(mat.emissive).metallic(mat.metallic).roughness(mat.perceptual_roughness)
    }
}

impl MaterialPalette {
    pub fn new() -> Self { MaterialPalette { palette: Vec::new() } }

    pub fn push(&mut self, material: FlatMaterial) {
        self.palette.push(material); 
    }

    pub fn compile(&self, size: Option<u32>) -> CompiledMaterials {
        let dimensions = (size.unwrap_or(self.palette.len() as u32), 1u32);
        let len = self.palette.len() as u32;
        let mut self_clone = self.to_owned();

        if dimensions.0 < len {
            panic!("Width of image cannot be less than the number of materials given.");
        } else if dimensions.0 > len {
            for _ in 0..(dimensions.0 - len) {
                self_clone.push(FlatMaterial::default());
            }
        }

        let (mut base_color_vec, mut emissive_vec, mut metallic_roughness_vec) = (Vec::new(), Vec::new(), Vec::new());
        for material in self_clone.palette {
            base_color_vec.append(&mut material.base_color.to_vec_u8());
            emissive_vec.append(&mut material.emissive.to_vec_u8());
            metallic_roughness_vec.append(&mut Color::rgba(0.0, material.roughness, material.metallic, 1.0).to_vec_u8());
        }

        let mut basic_data = Vec::new();
        for _ in 0..(dimensions.0 * 4) {
            basic_data.push(0u8);
        }

        let basic_image = Image::new(
            Extent3d { width: dimensions.0, height: dimensions.1, ..Default::default() },
            TextureDimension::D2,
            basic_data,
            TextureFormat::Rgba8Uint
        );

        let base_color_texture = basic_image.with_data(base_color_vec);
        let emissive_texture = basic_image.with_data(emissive_vec);
        let metallic_roughness_texture = basic_image.with_data(metallic_roughness_vec);

        CompiledMaterials { base_color_texture, emissive_texture, metallic_roughness_texture }
    }
}

impl CompiledMaterials {
    pub fn get_uv_pos(&self, material_num: u32) -> Vec2 {
        let num_materials = self.base_color_texture.texture_descriptor.size.width;

        assert!(num_materials > material_num, "Material {} out of bounds. Palette had {} materials.", material_num, num_materials);

        let length = num_materials as f32;
        let x = (material_num as f32 + 0.5) / length;
        let y = 0.5;

        Vec2::new(x, y)
    }

    pub fn into_standard_material(self, images: &mut ResMut<Assets<Image>>) -> StandardMaterial {
        StandardMaterial {
            base_color: Color::WHITE,
            emissive: Color::WHITE,
            perceptual_roughness: 1.0,
            metallic: 1.0,

            base_color_texture: Some(images.add(self.base_color_texture)),
            emissive_texture: Some(images.add(self.emissive_texture)),
            metallic_roughness_texture: Some(images.add(self.metallic_roughness_texture)),

            ..Default::default()
        }
    }
}

pub trait ColorToVec {
    fn to_vec_u8(self) -> Vec<u8>;
}

impl ColorToVec for Color {
    fn to_vec_u8(self) -> Vec<u8> {
        vec![
            (self.r() * 255.0).round() as u8,
            (self.g() * 255.0).round() as u8,
            (self.b() * 255.0).round() as u8,
            (self.a() * 255.0).round() as u8,
        ]
    }
}

trait WithData {
    fn with_data(&self, data: Vec<u8>) -> Self;
}

impl WithData for Image {
    fn with_data(&self, data: Vec<u8>) -> Self {
        let mut return_image = self.clone();
        return_image.data = data;
        return_image
    }
}