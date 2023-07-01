pub type DateTimeUtc = chrono::DateTime<chrono::Utc>;

mod camera;
mod photo;
mod video;

pub use camera::Camera;
pub use photo::Photo;
pub use video::Video;
