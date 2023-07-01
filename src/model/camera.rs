#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord,
    Hash, serde::Serialize, serde::Deserialize)
]
pub struct Camera{
   pub id:         surrealdb::sql::Thing,
   pub name:       Option<String>,
   pub state:      bool,
   pub url:        Option<String>,
   pub ip:         Option<String>,
   pub login:      Option<String>,
   pub password:   Option<String>,
   pub port:       u64,
   pub fire_pr:    u64,
   pub smoke_pr:   u64,
   
}


/*
use opencv::{
    core::*,
    imgcodecs::*,
    highgui::*, videoio::*, imgproc::gaussian_blur,
    
};

#[derive(Debug, Default)]
pub struct CameraData{
    pub login: String,
    pub pass: String,
    pub ip: String,
    pub port: u32,
    pub name: String,
}

impl CameraData {
    pub fn get_url_from_data(&self)-> String{
        format!(
            "http://{login}:{password}@{ip}:{port}/ISAPI/Streaming/channels/102/httpPreview",
            login = self.login,
            password = self.pass,
            ip = self.ip,
            port = self.port
        )
    }
    pub fn from_url(url_str: &str)-> std::result::Result<CameraData, Box<dyn std::error::Error>>{
        let mut cdata = CameraData::default();
        let url = url::Url::parse(url_str)?;
        let ip = match url.host_str(){
            Some(x) => {
                x.to_string()
            }
            None => {
                return Err("IP not FAUND by url from Camera!".into());
            }
        };
        Ok(CameraData{
            login: url.username().to_string(),
            pass: url.password().unwrap_or("").to_string(),
            ip: ip.clone(),
            port: url.port().unwrap_or(80) as u32,
            name: format!("CAM/{}", ip.clone())
        })
        
    }
}

pub struct Camera{
    data: CameraData,
    url: String,
    cap: opencv::videoio::VideoCapture,
}

impl Camera{
    
    pub fn new(url: &str) -> opencv::Result<Camera, opencv::Error> {
        Ok(Camera{
            data: match CameraData::from_url(url){
                Ok(x) => {x},
                Err(e) => {return Err(opencv::Error::new(
                    opencv::core::StsError,
                    format!("Failed to create CameraData: {}", e),
                    ));
                }
            },
            url: url.to_string(),
            cap: opencv::videoio::VideoCapture::from_file(url, CAP_ANY)?,
        })
    }
    pub fn from_data(dta: CameraData) -> opencv::Result<Camera, opencv::Error> {
        let url = dta.get_url_from_data().to_string();
        Ok(Camera{
            data: dta,
            url: url.clone(),
            cap: opencv::videoio::VideoCapture::from_file(url.as_str(), CAP_ANY)?
        })
    }
    pub fn read(&mut self) ->  opencv::Result<opencv::core::Mat, opencv::Error>{
        let mut img = opencv::core::Mat::default();
        self.cap.read(&mut img)?;
        Ok(img)
    }
}
*/
/*
impl MyCamera {
    pub fn from_url(url: &str) -> opencv::Result<MyCamera, opencv::Error> {
        Ok(MyCamera {
            login: "admin".to_string(),
            pass: "123qazwsx".to_string(),
            host: ("169.254.78.49".to_string(), 8000),
            name: "CAM3".to_string(),   
            cap: VideoCapture::from_file(url, CAP_ANY).unwrap(),
        })
    }
    pub fn new(nm: &str, lg: &str, pass: &str, ip: &str, port: u32) 
    -> opencv::Result<MyCamera, opencv::Error> {
        let cap = VideoCapture::default()?;

        Ok(MyCamera {
            login: lg.to_string(),
            pass: pass.to_string(),
            host: (ip.to_string(), port),
            name: nm.to_string(),   
            cap: cap,
        })
    }
    pub fn get_url(&self)->String{
        format!(
            //"http://admin:123qazwsx@169.254.78.49/ISAPI/Streaming/channels/102/httpPreview"
            "http://{}:{}@{}/ISAPI/Streaming/channels/102/httpPreview", 
            self.login, 
            self.pass, 
            self.host.0
        )
    }
    pub fn get_ip(&self)->String{
        format!(
            "{}", self.host.0
        )
    }
    pub fn connect(&mut self) -> opencv::Result<(), opencv::Error>{
        self.cap = VideoCapture::from_file(self.get_url().as_str(), CAP_ANY)?;

        Ok(())
    }
    pub fn is_opened(&self) -> opencv::Result<bool, opencv::Error>{
        self.cap.is_opened()
    }
    pub fn get_image(&mut self) -> Mat{
        let mut frame = Mat::default();
        self.cap.read(&mut frame).unwrap();
        frame
    }

    pub fn f(&mut self){
        let b = self.get_url();
        let url = b.as_str();
        self.cap = VideoCapture::from_file(url, CAP_ANY).unwrap();
        let window = "IP Camera";
        loop {
            
            let mut frame = Mat::default();
            self.cap.read(&mut frame).unwrap();
                opencv::highgui::imshow(window, &frame).unwrap();
                opencv::highgui::wait_key(1).unwrap();
        }
    }
}
*/