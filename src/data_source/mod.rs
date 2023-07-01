pub struct FileDataBase{
    pub db: surrealdb::Surreal<surrealdb::engine::local::Db>,
}

impl FileDataBase{
    pub async fn init() -> surrealdb::Result<Self>{
        let fdb = FileDataBase { 
            db: surrealdb::Surreal::new::<surrealdb::engine::local::File>("ziyrak.db").await? 
        };
        fdb.db.use_ns("namespace").use_db("database").await?;
        Ok(fdb)
    }

    pub async fn create_camera(&mut self, cam: super::model::Camera)-> surrealdb::Result<super::model::Camera>{
        let sql =
"CREATE camera 
SET id = $id, name = $name, state = $state, url = $url, ip = $ip, 
login = $login, password = $password, port = $port,
fire_pr = $fire_pr, smoke_pr = $smoke_pr"
;

        let mut ress = self.db
            .query(sql)
            .bind(cam)
            .await?;

        let c: Option<super::model::Camera> = ress.take(0)?;

        Ok(c.unwrap())
    }
    pub async fn create_video(&mut self, vid: super::model::Video) -> surrealdb::Result<super::model::Video>{
        let sql = 
"CREATE video 
SET id = $id, url = $url, created_at = $created_at, 
begined_at = $begined_at, ended_at = $ended_at, 
is_paniced = $is_paniced, camera_id = $camera_id;"
;
        let mut ress = self.db
            .query(sql)
            .bind(vid)
            .await?;
        

        let v: Option<super::model::Video> = ress.take(0)?;

        Ok(v.unwrap())
    }
    pub async fn create_photo(&mut self, photo: super::model::Photo) -> surrealdb::Result<super::model::Photo>{
        let sql = 
"CREATE photo
SET id = $id, url = $url, created_at = $created_at,
is_paniced = $is_paniced, camera_id = $camera_id";
        
        let mut ress = self.db
            .query(sql)
            .bind(photo)
            .await?;

        let p: Option<super::model::Photo> = ress.take(0)?;

        Ok(p.unwrap())
    }
    
    pub async fn insert_into_camera(&mut self, v_cam: Vec<super::model::Camera>) -> surrealdb::Result<Vec<super::model::Camera>>{
        let sql = "INSERT INTO camera $data";
        let mut ress = self.db
            .query(sql)
            .bind(("data", v_cam))
            .await?;

        let vc: Vec<super::model::Camera> = ress.take(0)?;

        Ok(vc)
    }

    pub async fn insert_into_video(&mut self, v_cam: Vec<super::model::Video>) -> surrealdb::Result<Vec<super::model::Video>>{
        let sql = "INSERT INTO video $data";
        let mut ress = self.db
            .query(sql)
            .bind(("data", v_cam))
            .await?;

        let vc: Vec<super::model::Video> = ress.take(0)?;

        Ok(vc)
    }

    pub async fn insert_into_photo(&mut self, v_cam: Vec<super::model::Photo>) -> surrealdb::Result<Vec<super::model::Photo>>{
        let sql = "INSERT INTO photo $data";
        let mut ress = self.db
            .query(sql)
            .bind(("data", v_cam))
            .await?;

        let vc: Vec<super::model::Photo> = ress.take(0)?;

        Ok(vc)
    }
}
