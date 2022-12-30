mod links {
    fn base_url() -> String {
        "http://localhost:8080".to_string()
    }
    pub(crate) fn ready_url() -> String {
        base_url() + r#"/ready"#
    }

    pub(crate) fn exampletranslate() -> String {
        base_url() + r#"/exampletranslate"#
    }

    pub(crate) fn examplenewtrans() -> String {
        base_url() + r#"/examplenewtrans"#
    }

    pub(crate) fn translate() -> String {
        base_url() + r#"/translate"#
    }

    pub(crate) fn newtrans() -> String {
        base_url() + r#"/newtrans"#
    }
}

pub mod maintenance {

    use reqwest::Response;

    use crate::request::get;

    use super::links::ready_url;

    pub async fn ready() -> Result<Response, String> {
        let result = get(&ready_url()).await;
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub mod examples {

    use super::links;
    use crate::request::get;

    pub async fn exampletranslate() -> Result<String, String> {
        let result = get(&links::exampletranslate()).await;

        match result {
            Ok(v) => {
                let body = v.text().await;
                match body {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }

            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn examplenewtrans() -> Result<String, String> {
        let result = get(&links::examplenewtrans()).await;

        match result {
            Ok(v) => {
                let body = v.text().await;
                match body {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }

            Err(e) => Err(e.to_string()),
        }
    }
}

pub mod translate {

    use translation_server_dtos_silen::{TransReq, TransResponse, TransErr};

    use super::links;
    use crate::request::post;

    pub async fn translate(content: &str, from: &str, to: &str) -> Result<TransResponse, TransErr> {
        let req = TransReq::new(content, from, to);
        let as_json = serde_json::to_string(&req).map_err(|e| e.to_string()).map_err(|e|TransErr::new(e.as_str()))?;

       
        let result = post(&links::translate(), as_json).await;

        let only_response = result.map_err(|e| TransErr::new(&e.to_string()))?;

        let body = only_response.text().await.map_err(|e| TransErr::new(&e.to_string()))?;

        let as_response: TransResponse =serde_json::from_str(body.as_str()).map_err(|e| TransErr::new(&e.to_string()))?;

        Ok(as_response)
}
}

pub mod newtrans {

    use translation_server_dtos_silen::NewTransReq;

    use super::links;
    use crate::request::post;

    pub async fn newtrans(
        from_lang: &str,
        to_lang: &str,
        word: &str,
        meanings: Vec<&str>,
    ) -> Result<String, String> {
        let req = NewTransReq::new(from_lang, to_lang, word, meanings);
        let as_json = serde_json::to_string(&req).map_err(|e| e.to_string())?;

        let result = post(&links::newtrans(), as_json).await;

        match result {
            Ok(v) => {
                let body = v.text().await;
                match body {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
