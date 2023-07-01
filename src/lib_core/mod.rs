use crate::model;

pub fn id_uuidv4_generetor(table: String) -> surrealdb::sql::Thing {
    surrealdb::sql::Thing{
        tb: table,
        id: surrealdb::sql::Id::String(surrealdb::sql::Uuid::new_v4().to_string())
    }
}

pub mod func_for_testing{
    use rand::Rng;
    use super::{
        id_uuidv4_generetor,
        model::{
            Camera, Video, Photo
        },
    };


    pub fn generate_random_camera() -> Camera {
        let mut rng = rand::thread_rng();

        // Generate random values
        let name: Option<String> = Some(format!("Cam{}", rng.gen_range(0..=65535)));
        let state: bool = rng.gen();
        let url_str: String = format!(
            "http://{}:{}@{}/ISAPI/Streaming/channels/102/httpPreview",
            "admin",
            "123qazwsx",
            format!(
                "{}.{}.{}.{}",
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256)
            )
        );
        let url = url::Url::parse(url_str.clone().as_str()).unwrap();
        let ip: String = url.host_str().unwrap().to_string();
        let login: String = url.username().to_string();
        let password: String = url.password().unwrap().to_string();
        let port: u64 = rng.gen_range(1..=65535);
        let fire_pr: u64 = rng.gen_range(0..=100);
        let smoke_pr: u64 = rng.gen_range(0..=100);

        // Create and return the Camera object
        Camera {
            id: id_uuidv4_generetor("camera".to_string()),
            name: name,
            state,
            url: Some(url_str),
            ip: Some(ip),
            login: Some(login),
            password: Some(password),
            port,
            fire_pr,
            smoke_pr,
        }
    }

    use chrono::{DateTime, Duration, Utc, TimeZone};
    pub fn generate_random_video() -> Video {
        let mut rng = rand::thread_rng();
    
        // Generate random values
        let url: Option<String> = Some(format!("/home/ali-0211-tr/Videos/vid_{:06}.mp4", rng.gen_range(0..=999999)));
        let begined_at: DateTime<Utc> = Utc.ymd(
            2023,
            rng.gen_range(0..=12), 
            rng.gen_range(0..=28)
        ).and_hms(
            rng.gen_range(0..=23), 
            rng.gen_range(0..=59), 
            rng.gen_range(0..=59)
        );
        let ended_at: DateTime<Utc> = begined_at + Duration::hours(2);
        
        // Create and return the Video object
        Video {
            id: id_uuidv4_generetor("video".to_string()),
            url,
            created_at: ended_at.clone(),
            begined_at,
            ended_at,
            is_paniced: false,
            camera_id: surrealdb::sql::Id::String(surrealdb::sql::Uuid::new_v4().to_string()),
        }
    }
    
    pub fn generate_random_photo() -> Photo {
        let mut rng = rand::thread_rng();
    
        // Generate random values
        let url: Option<String> = Some(format!("/home/ali-0211-tr/Pictures/photo_{:03}.png", rng.gen_range(1..=999)));
        let created_at: DateTime<Utc> = Utc.ymd(
            2023,
            rng.gen_range(0..=12), 
            rng.gen_range(0..=28)
        ).and_hms(
            rng.gen_range(0..=23), 
            rng.gen_range(0..=59), 
            rng.gen_range(0..=59)
        );;
        
        // Create and return the Photo object
        Photo {
            id: id_uuidv4_generetor("photo".to_string()),
            url,
            created_at,
            is_paniced: false,
            camera_id: surrealdb::sql::Id::String(surrealdb::sql::Uuid::new_v4().to_string()),
        }
    }
    

}