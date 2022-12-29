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

    use crate::request::get;

    use super::links::ready_url;

    pub enum Ready {
        Yes,
        No,
    }

    pub async fn ready() -> Ready {
        let result = get(&ready_url(), "".to_owned(), "".to_owned()).await;
        match result {
            Ok(_v) => Ready::Yes,
            Err(_e) => Ready::No,
        }
    }
}

pub mod examples {

    use futures::executor::block_on;

    use super::links;
    use crate::request::get;

    pub fn exampletranslate() -> Result<String, String> {
        let result = block_on(get(
            &links::exampletranslate(),
            "".to_owned(),
            "".to_owned(),
        ));

        match result {
            Ok(v) => {
                let body = block_on(v.text());
                match body {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }

            Err(e) => Err(e.to_string()),
        }
    }

    pub fn examplenewtrans() -> Result<String, String> {
        let result = block_on(get(&links::examplenewtrans(), "".to_owned(), "".to_owned()));

        match result {
            Ok(v) => {
                let body = block_on(v.text());
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

    use futures::executor::block_on;
    use translation_server_dtos_silen::{TransReq, TransResponse};

    use super::links;
    use crate::request::post;

    pub fn translate(content: &str, from: &str, to: &str) -> Result<TransResponse, String> {
        let req = TransReq::new(content, from, to);
        let as_json = serde_json::to_string(&req).map_err(|e| e.to_string())?;

        let result = block_on(post(
            &links::translate(),
            "".to_owned(),
            "".to_owned(),
            as_json,
        ));

        match result {
            Ok(v) => {
                let body = block_on(v.text());
                match body {
                    Ok(v) => {
                        let response: TransResponse =
                            serde_json::from_str(v.as_str()).map_err(|e| e.to_string())?;
                        Ok(response)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }

            Err(e) => Err(e.to_string()),
        }
    }
}

pub mod newtrans {

    use futures::executor::block_on;
    use translation_server_dtos_silen::NewTransReq;

    use super::links;
    use crate::request::post;

    pub fn newtrans(
        from_lang: &str,
        to_lang: &str,
        word: &str,
        meanings: Vec<&str>,
    ) -> Result<String, String> {
        let req = NewTransReq::new(from_lang, to_lang, word, meanings);
        let as_json = serde_json::to_string(&req).map_err(|e| e.to_string())?;

        let result = block_on(post(
            &links::newtrans(),
            "".to_owned(),
            "".to_owned(),
            as_json,
        ));

        match result {
            Ok(v) => {
                let body = block_on(v.text());
                match body {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.to_string()),
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
