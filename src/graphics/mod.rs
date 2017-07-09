//! 2D graphics module: sprites, text, shapes..

extern crate csfml_graphics_sys;

pub use self::blend_mode::BlendMode;
pub use self::circle_shape::CircleShape;
pub use self::color::Color;
pub use self::convex_shape::{ConvexShape, ConvexShapePoints};
pub use self::custom_shape::{CustomShape, CustomShapePoints};
pub use self::drawable::Drawable;
pub use self::font::{Font, FontRef, Info as FontInfo};
pub use self::glyph::Glyph;
pub use self::image::Image;
pub use self::primitive_type::PrimitiveType;
pub use self::rect::{FloatRect, IntRect, Rect};
pub use self::rectangle_shape::RectangleShape;
pub use self::render_states::RenderStates;
pub use self::render_target::RenderTarget;
pub use self::render_texture::RenderTexture;
pub use self::render_window::{Events, RenderWindow};
pub use self::shader::Shader;
pub use self::shape::Shape;
pub use self::sprite::{Sprite, RcSprite};
pub use self::text::Text;
pub use self::text_style::TextStyle;
pub use self::texture::{Texture, TextureRef};
pub use self::transform::Transform;
pub use self::transformable::Transformable;
pub use self::vertex::Vertex;
pub use self::vertex_array::{VertexArray, Vertices};
pub use self::view::{View, ViewRef};

mod drawable;
mod shape;
mod transformable;
mod render_target;
mod render_states;
mod render_window;
mod texture;
pub mod blend_mode;
mod transform;
mod text;
pub mod text_style;
mod shader;
mod color;
mod font;
mod view;
mod image;
mod sprite;
mod circle_shape;
mod rectangle_shape;
mod convex_shape;
mod primitive_type;
mod vertex;
mod vertex_array;
mod render_texture;
mod custom_shape;
mod rect;
mod glyph;
pub mod glsl;
